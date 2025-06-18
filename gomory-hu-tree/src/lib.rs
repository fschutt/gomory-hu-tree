//! # Gomory-Hu Tree Construction (`gomory-hu-tree`)
//!
//! [![Crates.io](https://img.shields.io/crates/v/gomory-hu-tree.svg)](https://crates.io/crates/gomory-hu-tree) <!-- TODO: Update when published -->
//! [![Documentation](https://docs.rs/gomory-hu-tree/badge.svg)](https://docs.rs/gomory-hu-tree) <!-- TODO: Update when published -->
//! <!-- [![Build Status](https://github.com/username/gomory-hu-tree/workflows/CI/badge.svg)](https://github.com/username/gomory-hu-tree/actions) -->
//! <!-- [![Coverage](https://codecov.io/gh/username/gomory-hu-tree/branch/main/graph/badge.svg)](https://codecov.io/gh/username/gomory-hu-tree) -->
//!
//! A Rust implementation of **Gomory-Hu Cut Tree Construction**, providing efficient
//! all-pairs minimum cut queries. After an initial preprocessing step to build the tree
//! (typically O(N * MaxFlowTime)), the minimum cut value between any pair of nodes
//! can be queried efficiently (currently O(N) in this implementation via BFS on tree edges).
//!
//! ## Features
//!
//! *   **Gusfield's Algorithm**: Implements the simplified algorithm by Gusfield (1990) for tree construction.
//! *   **Max-Flow Backend**: Uses Dinic's algorithm as the default max-flow solver.
//! *   **Graph Representation**: Includes a basic directed graph representation (`AdjacencyListFlowGraph`)
//!     suitable for flow algorithms, using `petgraph` internally.
//! *   **Query API**: Allows querying min-cut values between vertex pairs using the constructed tree.
//! *   **Property-Based Testing**: Includes comprehensive tests to ensure correctness.
//! *   **Benchmarking**: Performance benchmarks for construction and queries are available.
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! gomory_hu_tree = "0.1.0" // Replace with the actual version from crates.io
//! ```
//!
//! Basic usage with a simple line graph:
//! ```rust
//! use gomory_hu_tree::{gusfield_tree, DinicSolver, AdjacencyListFlowGraph, GomoryHuError};
//!
//! fn main() -> Result<(), GomoryHuError> {
//!     // 1. Create a graph (e.g., a line graph 0-1-2)
//!     let mut graph = AdjacencyListFlowGraph::new();
//!     let n0 = graph.add_node(()); // node 0
//!     let n1 = graph.add_node(()); // node 1
//!     let n2 = graph.add_node(()); // node 2
//!
//!     // Add undirected edges (as pairs of directed edges with same capacity)
//!     graph.add_edge(n0, n1, 10.0); graph.add_edge(n1, n0, 10.0); // 0-1 with capacity 10
//!     graph.add_edge(n1, n2, 5.0);  graph.add_edge(n2, n1, 5.0);  // 1-2 with capacity 5
//!
//!     // 2. Initialize a MaxFlowSolver
//!     let solver = DinicSolver::new();
//!
//!     // 3. Build Gomory-Hu tree
//!     let gh_tree = gusfield_tree(&graph, &solver)?;
//!
//!     // 4. Query minimum cut values
//!     // For the line graph 0 --10-- 1 --5-- 2:
//!     // Min-cut(0,1) = 10.0
//!     // Min-cut(1,2) = 5.0
//!     // Min-cut(0,2) = 5.0 (bottleneck on path 0-1-2)
//!
//!     // Note: Comparing floats requires care, typically using an epsilon.
//!     // For this example, direct assertion might fail due to tiny float inaccuracies.
//!     // let epsilon = 1e-9;
//!     // assert!((gh_tree.min_cut_value(n0, n1) - 10.0).abs() < epsilon);
//!     // assert!((gh_tree.min_cut_value(n1, n2) - 5.0).abs() < epsilon);
//!     // assert!((gh_tree.min_cut_value(n0, n2) - 5.0).abs() < epsilon);
//!
//!     println!("Min-cut between node {} and {}: {}", n0, n1, gh_tree.min_cut_value(n0,n1));
//!     println!("Min-cut between node {} and {}: {}", n1, n2, gh_tree.min_cut_value(n1,n2));
//!     println!("Min-cut between node {} and {}: {}", n0, n2, gh_tree.min_cut_value(n0,n2));
//!
//!     // To visualize the tree (e.g., print in DOT format):
//!     // println!("\nGraphviz DOT format of the Gomory-Hu tree:\n{}", gh_tree.to_dot());
//!     Ok(())
//! }
//! ```
//!
//! ## Algorithm Background
//! The **Gomory-Hu Cut Tree** (R. E. Gomory and T. C. Hu, 1961) is a data structure
//! that represents the minimum s-t cuts for all N(N-1)/2 vertex pairs in an undirected graph.
//! It is a weighted tree where edges correspond to min-cuts in the original graph.
//! The min-cut value between any two nodes `s` and `t` in the original graph is equal to
//! the minimum capacity of any edge on the unique path between `s` and `t` in the Gomory-Hu tree.
//!
//! This implementation uses **Gusfield's simplified algorithm** (D. Gusfield, 1990),
//! which requires N-1 max-flow computations, avoiding graph contractions used in the original Gomory-Hu method.
//!
//! ## Current Implementation Details
//! *   **Query Complexity**: `min_cut_value` is currently O(N) due to BFS/DFS on tree edges.
//!     Future optimizations could use LCA algorithms for O(log N) or O(alpha(N)) queries.
//! *   **Error Handling**: Errors from max-flow computations or graph inconsistencies are propagated
//!     via `MaxFlowError` and `GomoryHuError`.
//! *   **Graph Input**: The primary graph input `AdjacencyListFlowGraph` is a directed graph.
//!     To model undirected edges for Gomory-Hu construction, users should add pairs of directed edges
//!     (i.e., `u->v` and `v->u` with the same capacity).

// --- Public API Re-exports ---

pub mod algorithms;
pub mod flow;
pub mod tree;
pub mod utils;

// Core data structures for the Gomory-Hu tree itself
pub use tree::{GomoryHuTree, TreeEdge};

// Main algorithm for constructing the Gomory-Hu tree and its associated error type
pub use algorithms::{gusfield_tree, GomoryHuError};

// Max-flow solvers, traits, errors, and graph representation
pub use flow::{
    DinicSolver,            // A concrete max-flow solver using Dinic's algorithm
    MaxFlowSolver,          // Trait for max-flow algorithms
    MaxFlowError,           // Error type for max-flow computations
    AdjacencyListFlowGraph, // Graph data structure compatible with the solvers
    // MinCut, FlowGraph, OriginalGraphView might be used internally or by users building custom graphs/solvers
    // but are not essential for basic Gomory-Hu tree usage.
};

// (utils module is currently empty or internal, so not re-exporting anything from it yet)
