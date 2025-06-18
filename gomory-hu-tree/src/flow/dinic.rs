use super::traits::{MaxFlowError, MaxFlowSolver, MinCut, OriginalGraphView};
use std::collections::VecDeque;

// Internal helper struct for representing edges in the residual graph.
#[derive(Clone, Copy, Debug)]
struct ResidualEdge {
    to: usize,
    capacity: f64,
    reverse_edge_index: usize, // Index of the reverse edge in adj[to]
}

// Internal helper struct for the residual graph used by Dinic's algorithm.
// Not part of the public API.
struct ResidualGraph {
    adj: Vec<Vec<ResidualEdge>>,
    vertex_count: usize,
}

impl ResidualGraph {
    // Creates a new residual graph from an original graph view.
    pub fn new_from_original<G: OriginalGraphView>(original_graph: &G) -> Self {
        let vertex_count = original_graph.vertex_count();
        let mut adj = vec![Vec::new(); vertex_count];

        for (u, v, capacity) in original_graph.all_edges() {
            if u >= vertex_count || v >= vertex_count {
                continue;
            }
            let u_rev_idx = adj[v].len();
            let v_rev_idx = adj[u].len();
            adj[u].push(ResidualEdge { to: v, capacity, reverse_edge_index: u_rev_idx });
            adj[v].push(ResidualEdge { to: u, capacity: 0.0, reverse_edge_index: v_rev_idx });
        }
        Self { adj, vertex_count }
    }

    // Pushes flow along an edge and updates residual capacities for the edge and its reverse.
    pub fn push_flow_on_edge(&mut self, u: usize, edge_idx_in_u: usize, flow_amount: f64) {
        if flow_amount <= 1e-9 { return; }

        self.adj[u][edge_idx_in_u].capacity -= flow_amount;

        let v = self.adj[u][edge_idx_in_u].to;
        let reverse_edge_idx_in_v = self.adj[u][edge_idx_in_u].reverse_edge_index;
        self.adj[v][reverse_edge_idx_in_v].capacity += flow_amount;
    }
}

// Moved dfs_path function before DinicSolver struct definition
// Finds an augmenting path using DFS from u to sink in the level graph.
// `ptr` is used for the "pointer" optimization to avoid re-exploring dead-end edges.
// Returns `Some(pushed_flow)` if a path is found, `None` otherwise.
fn dfs_path(graph: &mut ResidualGraph, u: usize, sink: usize, flow_limit: f64, levels: &[i32], ptr: &mut [usize]) -> Option<f64> {
    if u == sink { return Some(flow_limit); }
    if flow_limit < 1e-9 { return None; } // No capacity to push

    while ptr[u] < graph.adj[u].len() {
        let edge_idx = ptr[u];
        let v = graph.adj[u][edge_idx].to;
        let current_edge_capacity = graph.adj[u][edge_idx].capacity;

        // Path must go to the next level, and edge must have capacity.
        if levels[v] != levels[u] + 1 || current_edge_capacity < 1e-9 {
            ptr[u] += 1;
            continue;
        }

        let path_flow_limit = flow_limit.min(current_edge_capacity);
        if let Some(pushed_flow) = dfs_path(graph, v, sink, path_flow_limit, levels, ptr) { // Recursive call to free function
            if pushed_flow > 1e-9 { // Ensure some meaningful flow was pushed
                 graph.push_flow_on_edge(u, edge_idx, pushed_flow);
                return Some(pushed_flow);
            }
        }
        ptr[u] += 1; // Move to the next edge for node u
    }
    None // No path found from u
}


/// Implements Dinic's algorithm for finding the maximum flow in a flow network.
///
/// Dinic's algorithm is an efficient blocking flow algorithm. It repeatedly:
/// 1. Builds a level graph from the source in the current residual graph using BFS.
/// 2. Finds a blocking flow in the level graph using DFS.
///
/// This process continues until no more augmenting paths can be found from source to sink
/// in the level graph.
#[derive(Debug, Clone, Copy)]
pub struct DinicSolver {
    max_iterations: usize,
}

impl DinicSolver {
    /// Creates a new `DinicSolver` with a default maximum number of iterations.
    pub fn new() -> Self {
        Self { max_iterations: 100_000 }
    }

    /// Creates a new `DinicSolver` with a specified maximum number of iterations.
    pub fn with_max_iterations(max_iterations: usize) -> Self {
        Self { max_iterations }
    }

    // This is a method of DinicSolver, called via self.build_level_graph
    fn build_level_graph(&self, graph: &ResidualGraph, source: usize, sink: usize) -> Vec<i32> {
        let mut levels = vec![-1; graph.vertex_count];
        if source >= graph.vertex_count { return levels; }

        levels[source] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(u) = queue.pop_front() {
            if u == sink { break; }
            for edge in &graph.adj[u] {
                if edge.capacity > 1e-9 && levels[edge.to] == -1 {
                    levels[edge.to] = levels[u] + 1;
                    queue.push_back(edge.to);
                }
            }
        }
        levels
    }

    // This is a method of DinicSolver, called via self.find_reachable
    fn find_reachable(&self, graph: &ResidualGraph, source: usize) -> Vec<bool> {
        let mut reachable = vec![false; graph.vertex_count];
        if source >= graph.vertex_count { return reachable; }

        let mut stack = vec![source];
        reachable[source] = true;
        while let Some(u) = stack.pop() {
            for edge in &graph.adj[u] {
                if edge.capacity > 1e-9 && !reachable[edge.to] {
                    reachable[edge.to] = true;
                    stack.push(edge.to);
                }
            }
        }
        reachable
    }

    // This is a method of DinicSolver, called via self.extract_min_cut
    fn extract_min_cut(&self, graph: &ResidualGraph, source: usize) -> MinCut {
        MinCut::new(self.find_reachable(graph, source))
    }
}

impl<G: OriginalGraphView> MaxFlowSolver<G> for DinicSolver {
    type Flow = f64;

    fn max_flow_min_cut(&self, graph_view: &G, source: usize, sink: usize) -> Result<(Self::Flow, MinCut), MaxFlowError> {
        let n = graph_view.vertex_count();
        if n == 0 { return Err(MaxFlowError::EmptyGraph); }
        if source >= n { return Err(MaxFlowError::VertexNotFound(source)); }
        if sink >= n { return Err(MaxFlowError::VertexNotFound(sink)); }
        if source == sink { return Err(MaxFlowError::SourceEqualsSink(source)); }

        let mut residual_graph = ResidualGraph::new_from_original(graph_view);
        let mut total_flow = 0.0;

        let mut iterations_completed = 0;
        while iterations_completed < self.max_iterations {
            let levels = self.build_level_graph(&residual_graph, source, sink);
            if levels[sink] == -1 { break; }

            let mut ptr = vec![0; n];
            let mut flow_this_phase = 0.0;
            // Call to the free function dfs_path
            while let Some(pushed_path_flow) = dfs_path(&mut residual_graph, source, sink, f64::INFINITY, &levels, &mut ptr) {
                if pushed_path_flow < 1e-9 { break; }
                flow_this_phase += pushed_path_flow;
            }

            if flow_this_phase < 1e-9 { break; }
            total_flow += flow_this_phase;
            iterations_completed += 1;
        }

        if iterations_completed == self.max_iterations &&
           self.build_level_graph(&residual_graph, source, sink)[sink] != -1 {
            return Err(MaxFlowError::MaxIterationsReached(self.max_iterations));
        }

        let min_cut = self.extract_min_cut(&residual_graph, source);
        Ok((total_flow, min_cut))
    }
}

impl Default for DinicSolver {
    fn default() -> Self { Self::new() }
}
