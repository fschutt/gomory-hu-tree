use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Directed;
use std::marker::PhantomData;

use super::traits::{FlowGraph, OriginalGraphView};

/// An adapter for `petgraph::Graph` to be used as a `FlowGraph` and `OriginalGraphView`.
///
/// This graph is directed. Edge weights are `f64` representing capacities.
/// Node weights are generic (`N`), defaulting to `()`.
///
/// It assumes that vertex indices used by algorithms (`usize`) correspond directly to
/// `petgraph::NodeIndex::index()` values. This holds if nodes are added and not removed.
#[derive(Debug, Clone)]
pub struct AdjacencyListFlowGraph<N = ()>
where
    N: Clone + std::fmt::Debug,
{
    /// The internal `petgraph::Graph` instance.
    graph: Graph<N, f64, Directed, u32>,
    _phantom_n: PhantomData<N>,
}

impl<N> AdjacencyListFlowGraph<N>
where
    N: Clone + std::fmt::Debug,
{
    /// Creates a new, empty `AdjacencyListFlowGraph`.
    pub fn new() -> Self {
        Self {
            graph: Graph::default(), // Graph::default() is Directed
            _phantom_n: PhantomData,
        }
    }

    /// Adds a new node with the given weight to the graph.
    ///
    /// # Arguments
    /// * `weight`: The weight of the node (e.g., `()` if no specific data is needed).
    ///
    /// # Returns
    /// The `usize` index of the newly added node. This index is what should be used
    /// in `add_edge` calls and by flow algorithms.
    pub fn add_node(&mut self, weight: N) -> usize {
        self.graph.add_node(weight).index()
    }

    /// Adds a directed edge to the graph.
    ///
    /// # Arguments
    /// * `u_idx`: The `usize` index of the source node (previously returned by `add_node`).
    /// * `v_idx`: The `usize` index of the target node (previously returned by `add_node`).
    /// * `capacity`: The capacity of the edge (must be `f64`).
    ///
    /// # Panics
    /// Panics if `u_idx` or `v_idx` do not correspond to existing nodes in the graph,
    /// or if they are out of bounds for the current node count.
    /// `petgraph` itself panics if `NodeIndex`s are invalid.
    pub fn add_edge(&mut self, u_idx: usize, v_idx: usize, capacity: f64) {
        // Ensure indices are within the current node count before creating NodeIndex.
        // This prevents panics from NodeIndex::new if idx is too large,
        // though petgraph.add_edge would panic anyway if node doesn't exist.
        let node_count = self.graph.node_count();
        if u_idx >= node_count || v_idx >= node_count {
            panic!("Attempted to add edge with out-of-bounds vertex index. u_idx: {}, v_idx: {}, node_count: {}",
                   u_idx, v_idx, node_count);
        }
        let u_node = NodeIndex::new(u_idx);
        let v_node = NodeIndex::new(v_idx);

        self.graph.add_edge(u_node, v_node, capacity);
    }
}

/// Implements `FlowGraph` for `AdjacencyListFlowGraph`.
impl<N> FlowGraph for AdjacencyListFlowGraph<N>
where
    N: Clone + std::fmt::Debug,
{
    /// Returns the number of vertices in the graph.
    fn vertex_count(&self) -> usize {
        self.graph.node_count()
    }
}

/// Implements `OriginalGraphView` for `AdjacencyListFlowGraph`.
impl<N> OriginalGraphView for AdjacencyListFlowGraph<N>
where
    N: Clone + std::fmt::Debug,
{
    /// Returns an iterator over all edges in the graph, yielding tuples of
    /// `(source_idx, target_idx, capacity)`.
    ///
    /// Vertex indices are `usize`.
    fn all_edges(&self) -> Box<dyn Iterator<Item = (usize, usize, f64)> + '_> {
        Box::new(self.graph.edge_references().map(|edge_ref| {
            (
                edge_ref.source().index(), // NodeIndex::index() gives usize
                edge_ref.target().index(), // NodeIndex::index() gives usize
                *edge_ref.weight(),        // Dereference to get f64 capacity
            )
        }))
    }
}

/// Default implementation for `AdjacencyListFlowGraph`.
/// Creates an empty graph. Requires `N` to implement `Default`.
impl<N> Default for AdjacencyListFlowGraph<N>
where N: Default + Clone + std::fmt::Debug
{
    fn default() -> Self {
        Self {
            graph: Graph::default(),
            _phantom_n: PhantomData,
        }
    }
}
