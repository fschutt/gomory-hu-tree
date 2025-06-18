/// A trait for basic graph structures used in flow algorithms.
///
/// Graphs implementing this trait provide essential information like vertex count.
pub trait FlowGraph {
    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize;
    // Potential future methods:
    // fn edge_count(&self) -> usize;
    // fn get_capacity(&self, u: usize, v: usize) -> Option<f64>;
}

/// Represents a min-cut partition in a graph.
///
/// The partition divides the graph's vertices into two sets, typically the source-side
/// (S) and the sink-side (T) of the cut.
#[derive(Debug, Clone)]
pub struct MinCut {
    /// A boolean vector where `partition[i]` is true if vertex `i` is on the source-side
    /// of the cut, and false otherwise (meaning it's on the sink-side).
    /// The length of this vector should be equal to the number of vertices in the graph.
    pub partition: Vec<bool>,
}

impl MinCut {
    /// Creates a new `MinCut` from a given partition.
    ///
    /// # Arguments
    /// * `partition`: A vector where `partition[i]` indicates if vertex `i` is on the
    ///   source-side of the cut.
    pub fn new(partition: Vec<bool>) -> Self {
        Self { partition }
    }

    /// Checks if a vertex `v` is on the same side of the cut as a representative vertex `s_representative`.
    ///
    /// Typically, `s_representative` is the source node used in the max-flow computation that yielded this cut.
    ///
    /// # Arguments
    /// * `v`: The vertex index to check.
    /// * `s_representative`: The vertex index of the representative (e.g., the source node `s`).
    ///
    /// # Returns
    /// `true` if `v` is on the same side of the cut as `s_representative`, `false` otherwise.
    ///
    /// # Panics
    /// Panics if `v` or `s_representative` are out of bounds for the partition data.
    pub fn same_side(&self, v: usize, s_representative: usize) -> bool {
        if v >= self.partition.len() || s_representative >= self.partition.len() {
            panic!("Vertex index out of bounds in MinCut::same_side. v: {}, s_rep: {}, partition_len: {}",
                   v, s_representative, self.partition.len());
        }
        // If partition[s_representative] is true, it means "true" denotes the s-side.
        // So, v is on s-side if partition[v] is also true.
        // If partition[s_representative] is false, it means "false" denotes the s-side (unexpected, but possible).
        // So, v is on s-side if partition[v] is also false.
        // This is equivalent to checking if their boolean values are the same.
        self.partition[v] == self.partition[s_representative]
    }
}

/// Errors that can occur during max-flow computations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum MaxFlowError {
    /// The source and sink vertices are the same.
    #[error("Source and sink are the same vertex (s: {0}, t: {0})")]
    SourceEqualsSink(usize),
    /// A specified vertex was not found in the graph (e.g., index out of bounds).
    #[error("Vertex {0} not found in graph")]
    VertexNotFound(usize),
    /// The algorithm reached its maximum allowed iterations before completion.
    /// This might indicate a very complex graph or an iteration limit that is too low.
    #[error("Maximum iterations ({0}) reached in max-flow computation")]
    MaxIterationsReached(usize),
    /// The graph has no vertices, making max-flow undefined.
    #[error("Graph has 0 vertices, max-flow is undefined")]
    EmptyGraph,
    /// An unspecified internal error occurred within the max-flow algorithm.
    #[error("Internal max-flow error: {0}")]
    InternalError(String),
}

/// A trait for max-flow algorithms.
///
/// Implementors of this trait can compute the maximum flow and corresponding minimum cut
/// in a given flow network (graph).
/// `G` is the type of the graph, which must implement `FlowGraph`.
pub trait MaxFlowSolver<G: FlowGraph> {
    /// The type used for representing flow values (e.g., `f64`, `i32`).
    /// Must support basic arithmetic, comparison, and debugging.
    type Flow: Copy + Default + PartialOrd + std::ops::AddAssign + std::fmt::Debug;

    /// Computes the maximum flow from a `source` to a `sink` in the given `graph`.
    ///
    /// # Arguments
    /// * `graph`: A reference to the graph implementing `FlowGraph`.
    /// * `source`: The source vertex index.
    /// * `sink`: The sink vertex index.
    ///
    /// # Returns
    /// A `Result` containing a tuple `(max_flow_value, min_cut_partition)` if successful.
    /// The `min_cut_partition` (a `MinCut` struct) identifies nodes on the source-side of one minimum cut.
    /// Returns `MaxFlowError` if an issue occurs.
    fn max_flow_min_cut(
        &self,
        graph: &G,
        source: usize,
        sink: usize,
    ) -> Result<(Self::Flow, MinCut), MaxFlowError>;
}

/// A trait for graph views that provide access to all original edges.
///
/// This is used by algorithms like Dinic's to construct an initial residual graph.
/// It extends `FlowGraph`.
pub trait OriginalGraphView: FlowGraph {
    /// Returns an iterator over all edges in the graph.
    /// Each item is a tuple `(source_vertex_idx, target_vertex_idx, capacity)`.
    /// For undirected graphs, an edge `u-v` with capacity `C` might be represented
    /// by two directed edges `(u,v,C)` and `(v,u,C)` from this iterator if the underlying
    /// graph implementation is directed (like `AdjacencyListFlowGraph`).
    fn all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f64)> + '_>;
}
