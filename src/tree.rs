use std::collections::{HashMap, VecDeque};

/// Represents an edge in the Gomory-Hu tree.
///
/// Each edge connects two nodes from the original graph (or supernodes formed during construction)
/// and stores a capacity, which corresponds to the min-cut value between those two nodes
/// (or sets of nodes) in the original graph.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct TreeEdge {
    /// One endpoint of the tree edge, representing a vertex from the original graph.
    pub source: usize,
    /// The other endpoint of the tree edge, representing a vertex from the original graph.
    pub target: usize,
    /// The capacity of this tree edge, equal to the min-cut value between `source` and `target`
    /// (or the components they represented) in the step it was added.
    pub capacity: f64,
}

impl TreeEdge {
    /// Creates a new `TreeEdge`.
    ///
    /// # Arguments
    /// * `source` - The source vertex index.
    /// * `target` - The target vertex index.
    /// * `capacity` - The capacity of the edge, representing a min-cut value.
    pub fn new(source: usize, target: usize, capacity: f64) -> Self {
        Self {
            source,
            target,
            capacity,
        }
    }
}

/// Represents a Gomory-Hu tree.
///
/// The tree stores the all-pairs min-cut information for an undirected graph.
/// The min-cut value between any two nodes `s` and `t` in the original graph
/// is equal to the minimum capacity of any edge on the unique path between
/// `s` and `t` in this Gomory-Hu tree.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub struct GomoryHuTree {
    /// The number of vertices in the original graph for which this tree was constructed.
    pub(crate) vertex_count: usize,
    /// A list of edges that form the Gomory-Hu tree.
    pub(crate) edges: Vec<TreeEdge>,
}

impl GomoryHuTree {
    /// Creates a new `GomoryHuTree` from a list of tree edges and the original vertex count.
    ///
    /// This constructor is typically used by tree construction algorithms like `gusfield_tree`.
    ///
    /// # Arguments
    /// * `edges` - A vector of `TreeEdge`s forming the tree.
    /// * `vertex_count` - The number of vertices in the original graph.
    pub fn new(edges: Vec<TreeEdge>, vertex_count: usize) -> Self {
        Self {
            edges,
            vertex_count,
        }
    }

    /// Calculates the min-cut value between two vertices `s` and `t` from the original graph.
    ///
    /// This method finds the unique path between `s` and `t` in the Gomory-Hu tree
    /// and returns the minimum capacity of an edge on that path.
    ///
    /// The current implementation uses a Breadth-First Search (BFS) to find the path
    /// and its bottleneck capacity, resulting in O(V) query time, where V is the number
    /// of vertices in the original graph.
    ///
    /// # Arguments
    /// * `s` - The source vertex index.
    /// * `t` - The target vertex index.
    ///
    /// # Returns
    /// The min-cut value between `s` and `t`. Returns `f64::INFINITY` if `s == t`.
    /// Returns `0.0` if `s` or `t` are out of bounds after initial checks, or if no path is found
    /// (though a path should always exist in a valid Gomory-Hu tree for `vertex_count > 0` and `s != t`).
    ///
    /// # Panics
    /// Panics if `s` or `t` are greater than or equal to `vertex_count` at the start of the function.
    pub fn min_cut_value(&self, s: usize, t: usize) -> f64 {
        if self.vertex_count == 0 {
            return 0.0;
        }
        if s >= self.vertex_count || t >= self.vertex_count {
            // This check is important as s and t are used as indices.
            panic!(
                "Vertex index out of bounds (s: {}, t: {}, vc: {})",
                s, t, self.vertex_count
            );
        }
        if s == t {
            return f64::INFINITY;
        }

        // Build an adjacency list representation of the tree for pathfinding.
        // This is done per-query; for higher performance, a persistent adjacency list
        // or a specialized tree data structure (e.g., for LCA) would be better.
        let mut adj: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
        for edge in &self.edges {
            adj.entry(edge.source)
                .or_default()
                .push((edge.target, edge.capacity));
            adj.entry(edge.target)
                .or_default()
                .push((edge.source, edge.capacity));
        }

        // BFS to find path from s to t and the minimum capacity on that path.
        // queue stores (current_node, min_capacity_on_path_from_s_to_current_node, path_for_debug)
        let mut queue: VecDeque<(usize, f64, Vec<usize>)> = VecDeque::new();
        queue.push_back((s, f64::INFINITY, vec![s]));

        // visited_bfs prevents cycles and redundant exploration.
        // Size based on vertex_count from original graph.
        let mut visited_bfs = vec![false; self.vertex_count];
        visited_bfs[s] = true; // s is known to be < self.vertex_count due to earlier panic.

        while let Some((curr, path_min_cap, current_path)) = queue.pop_front() {
            if curr == t {
                return path_min_cap; // Found path to t, return the bottleneck capacity.
            }

            if let Some(neighbors) = adj.get(&curr) {
                for &(neighbor, edge_cap) in neighbors {
                    // Ensure neighbor is a valid index before using it for visited_bfs.
                    if neighbor < self.vertex_count && !visited_bfs[neighbor] {
                        visited_bfs[neighbor] = true;
                        let mut next_path = current_path.clone(); // Clone path for debugging/logging if needed.
                        next_path.push(neighbor);
                        // The new path's bottleneck is the minimum of the current path's bottleneck
                        // and the capacity of the edge just traversed.
                        queue.push_back((neighbor, path_min_cap.min(edge_cap), next_path));
                    }
                }
            }
        }

        // Should ideally not be reached if s and t are in the same connected component of a valid Gomory-Hu tree.
        // A Gomory-Hu tree for a graph with N > 0 vertices should be connected.
        // If the original graph was disconnected, gusfield_tree might produce a forest.
        // In such a case, a query between nodes in different components would find no path.
        // The min-cut value in a disconnected graph between two nodes in different components is 0.
        0.0
    }

    /// Converts the Gomory-Hu tree to DOT format for visualization.
    ///
    /// The DOT format can be used with tools like Graphviz to generate images of the tree.
    /// Each edge in the output will be labeled with its capacity.
    /// If the tree has no edges but has vertices (e.g. for a 1-node original graph, or a 0-node graph),
    /// it will represent the nodes without edges.
    ///
    /// # Returns
    /// A string containing the DOT representation of the tree.
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("graph GomoryHuTree {\n");
        if self.vertex_count == 0 {
            // No nodes to represent for an empty original graph.
        } else if self.edges.is_empty() {
            // Graph might have nodes but no edges (e.g., a single node graph, or a graph of isolated nodes).
            // The Gomory-Hu "tree" for N isolated nodes would be N nodes and 0 edges.
            // Gusfield's algorithm as implemented should result in N-1 edges if N > 1 and graph is connected.
            // If N=1, edges list is empty. If N=0, vertex_count is 0.
            // This ensures all original nodes are listed if there are no tree edges.
            for i in 0..self.vertex_count {
                dot.push_str(&format!("  {i}\n"));
            }
        } else {
            for edge in &self.edges {
                dot.push_str(&format!(
                    "  {} -- {} [label=\"{:.2}\"]\n",
                    edge.source, edge.target, edge.capacity
                ));
            }
        }
        dot.push_str("}\n");
        dot
    }

    /// Returns the number of vertices in the original graph for which this tree was constructed.
    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Returns the number of edges in the Gomory-Hu tree.
    /// For a graph with N > 0 vertices, this should be N-1 if the graph is connected.
    /// Returns 0 if N=0 or N=1.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}
