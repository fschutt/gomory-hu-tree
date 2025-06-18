use crate::flow::{MaxFlowError, MaxFlowSolver, OriginalGraphView};
use crate::tree::{GomoryHuTree, TreeEdge};

/// Errors that can occur during the construction of a Gomory-Hu tree.
#[derive(Debug, thiserror::Error)]
pub enum GomoryHuError {
    /// Error originated from an underlying max-flow computation.
    #[error("Max-flow computation failed: {0}")]
    MaxFlowComputationError(#[from] MaxFlowError),
    /// Indicates an invalid graph structure was encountered (e.g., inconsistent vertex count).
    #[error("Invalid graph structure: {0}")]
    InvalidGraph(String),
    /// Indicates a vertex was not found during a pre-computation check (not typically raised by `gusfield_tree` itself).
    #[error("Vertex {0} not found in graph (pre-computation check)")]
    VertexNotFoundPreCheck(usize),
}

/// Constructs a Gomory-Hu tree for the given graph using Gusfield's algorithm.
///
/// Gusfield's algorithm (1990) provides a simplified method for building the Gomory-Hu tree
/// by performing N-1 max-flow computations, without requiring graph contractions.
///
/// # Arguments
/// * `graph`: A reference to a graph implementing `OriginalGraphView`. The graph is treated as undirected;
///   if using a directed `AdjacencyListFlowGraph`, ensure edges `(u,v)` and `(v,u)` are added
///   with the same capacity to model undirected edges.
/// * `solver`: A reference to a max-flow solver instance (e.g., `DinicSolver`) that implements
///   `MaxFlowSolver<G, Flow = f64>`. The flow values must be `f64`.
///
/// # Returns
/// A `Result` containing the `GomoryHuTree` if successful, or a `GomoryHuError` if an error occurs
/// (e.g., during a max-flow computation).
///
/// # Complexity
/// The algorithm performs N-1 calls to the max-flow solver. If T_max_flow is the time complexity
/// of one max-flow computation, Gusfield's algorithm is O(N * T_max_flow).
///
/// # Panics
/// This function may panic if the max-flow solver panics, or if there are unexpected issues
/// with vertex indices (though most vertex index issues should be caught by the solver or result in errors).
// TODO: Implement parallel execution for N-1 max-flow computations when 'parallel' feature is enabled.
// This would likely involve Rayon scopes within the main loop, if solver instances are Send/Sync
// or can be created per-thread. Each of the N-1 flow computations is independent.
pub fn gusfield_tree<G, S>(graph: &G, solver: &S) -> Result<GomoryHuTree, GomoryHuError>
where
    G: OriginalGraphView,
    S: MaxFlowSolver<G, Flow = f64>,
{
    let n = graph.vertex_count();

    if n == 0 {
        return Ok(GomoryHuTree::new(Vec::new(), 0));
    }
    if n == 1 {
        return Ok(GomoryHuTree::new(Vec::new(), 1));
    }

    // parent[i] stores the node to which vertex i is currently "contracted" or connected
    // in the context of the algorithm's evolving partitions. Initially, all nodes belong to
    // the component of vertex 0.
    let mut parent = vec![0; n];
    let mut tree_edges = Vec::with_capacity(n - 1);

    // Iterate from vertex 1 to n-1 (s_i in Gusfield's paper notation, using i here).
    for i in 1..n {
        // Calculate max flow between vertex `i` and its current parent `parent[i]`.
        // The `min_cut` result contains the partition of vertices on the `i`-side of the cut.
        let (flow_value, min_cut) = solver.max_flow_min_cut(graph, i, parent[i])?;

        // Add an edge to the Gomory-Hu tree between `i` and `parent[i]` with capacity `flow_value`.
        tree_edges.push(TreeEdge::new(i, parent[i], flow_value));

        // Update parent pointers for nodes that are on the same side of the cut as `i`
        // and were previously associated with `parent[i]`.
        // These nodes now effectively "contract" onto `i` for subsequent iterations
        // involving them as the `i` (source) parameter.
        for j in (i + 1)..n {
            if parent[j] == parent[i] && min_cut.same_side(j, i) {
                parent[j] = i;
            }
        }
    }

    Ok(GomoryHuTree::new(tree_edges, n))
}
