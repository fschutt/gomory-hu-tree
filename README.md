# Gomory-Hu Tree Construction (`gomory-hu-tree`)

[![Crates.io](https://img.shields.io/crates/v/gomory-hu-tree.svg)](https://crates.io/crates/gomory-hu-tree) <!-- TODO: Update when published -->
[![Documentation](https://docs.rs/gomory-hu-tree/badge.svg)](https://docs.rs/gomory-hu-tree) <!-- TODO: Update when published -->
<!-- [![Build Status](https://github.com/username/gomory-hu-tree/workflows/CI/badge.svg)](https://github.com/username/gomory-hu-tree/actions) -->
<!-- [![Coverage](https://codecov.io/gh/username/gomory-hu-tree/branch/main/graph/badge.svg)](https://codecov.io/gh/username/gomory-hu-tree) -->

A Rust implementation of **Gomory-Hu Cut Tree Construction**, providing efficient
all-pairs minimum cut queries. After an initial preprocessing step to build the tree
(typically O(N * MaxFlowTime)), the minimum cut value between any pair of nodes
can be queried efficiently (currently O(N) in this implementation, with potential for O(log N)).

## Features

*   **Gusfield's Algorithm**: Implements the simplified algorithm by Gusfield (1990) for tree construction.
*   **Max-Flow Backend**: Uses Dinic's algorithm as the default max-flow solver.
*   **Graph Representation**: Includes a basic directed graph representation (`AdjacencyListFlowGraph`)
    suitable for flow algorithms, using `petgraph` internally.
*   **Query API**: Allows querying min-cut values between vertex pairs using the constructed tree (`GomoryHuTree::min_cut_value`).
*   **DOT Output**: Trees can be exported to DOT format for visualization (`GomoryHuTree::to_dot`).
*   **Testing**: Includes unit, integration (academic cases), and property-based tests.
*   **Benchmarking**: Performance benchmarks for construction and queries are available via `cargo bench`.

## Quick Start

1.  **Add to `Cargo.toml`**:
    ```toml
    [dependencies]
    gomory_hu_tree = "0.1.0" # Replace with the actual version from crates.io
    ```

2.  **Basic Usage**:
    ```rust
    use gomory_hu_tree::{gusfield_tree, DinicSolver, AdjacencyListFlowGraph, GomoryHuError};

    fn main() -> Result<(), GomoryHuError> {
        // 1. Create a graph (e.g., a line graph 0-1-2)
        let mut graph = AdjacencyListFlowGraph::new();
        let n0 = graph.add_node(()); // node 0
        let n1 = graph.add_node(()); // node 1
        let n2 = graph.add_node(()); // node 2

        // Add undirected edges (as pairs of directed edges with same capacity)
        graph.add_edge(n0, n1, 10.0); graph.add_edge(n1, n0, 10.0); // 0-1 with capacity 10
        graph.add_edge(n1, n2, 5.0);  graph.add_edge(n2, n1, 5.0);  // 1-2 with capacity 5

        // 2. Initialize a MaxFlowSolver
        let solver = DinicSolver::new();

        // 3. Build Gomory-Hu tree
        let gh_tree = gusfield_tree(&graph, &solver)?;

        // 4. Query minimum cut values
        // For the line graph 0 --10-- 1 --5-- 2:
        // Min-cut(0,1) = 10.0
        // Min-cut(1,2) = 5.0
        // Min-cut(0,2) = 5.0 (bottleneck on path 0-1-2 in the GH tree)

        println!("Min-cut between node {} and {}: {}", n0, n1, gh_tree.min_cut_value(n0,n1));
        println!("Min-cut between node {} and {}: {}", n1, n2, gh_tree.min_cut_value(n1,n2));
        println!("Min-cut between node {} and {}: {}", n0, n2, gh_tree.min_cut_value(n0,n2));

        // Example: For the line graph, the tree structure would be:
        // 0 --10.0-- 1
        // 1 --5.0--- 2
        // min_cut_value(0,2) traverses 0-1 (10.0) and 1-2 (5.0), bottleneck is 5.0.

        // To visualize the tree (e.g., print in DOT format):
        // println!("\nGraphviz DOT format of the Gomory-Hu tree:\n{}", gh_tree.to_dot());
        Ok(())
    }
    ```

## Algorithm Background

The **Gomory-Hu Cut Tree** (R. E. Gomory and T. C. Hu, 1961) is a data structure
that represents the minimum s-t cuts for all N(N-1)/2 vertex pairs in an undirected graph.
It is a weighted tree where edges correspond to min-cuts in the original graph.
The min-cut value between any two nodes `s` and `t` in the original graph is equal to
the minimum capacity of any edge on the unique path between `s` and `t` in the Gomory-Hu tree.

This implementation uses **Gusfield's simplified algorithm** (D. Gusfield, 1990),
which requires N-1 max-flow computations, avoiding graph contractions used in the original Gomory-Hu method.

## Current Implementation Details

*   **Query Complexity**: `GomoryHuTree::min_cut_value` is currently O(N) (where N is number of vertices in original graph)
    due to a Breadth-First Search (BFS) on the tree edges to find the path and its bottleneck capacity.
    Future optimizations could use Lowest Common Ancestor (LCA) algorithms for O(log N) or O(alpha(N)) queries
    if a more advanced tree data structure is used internally.
*   **Error Handling**: Errors from max-flow computations or graph inconsistencies are propagated
    via `MaxFlowError` (from the solver) and `GomoryHuError` (from tree construction).
*   **Graph Input**: The primary graph input `AdjacencyListFlowGraph` is a directed graph.
    To model undirected edges for Gomory-Hu construction (which operates on undirected graphs),
    users should add pairs of directed edges (i.e., `u->v` and `v->u` both with the same capacity).
*   **`petgraph`**: The `AdjacencyListFlowGraph` uses `petgraph::Graph` internally for graph storage.

## Performance

Performance benchmarks for tree construction and min-cut queries are included in the crate.
You can run them using:
```bash
cargo bench
```
The results will be available in the `target/criterion` directory.
Current observations indicate that tree construction is the most computationally intensive part,
as expected, due to the N-1 max-flow computations. Query performance is linear with the number of nodes.

## Applications

Gomory-Hu trees are useful in various domains:
*   **Network Reliability**: Analyzing connectivity and bottlenecks in networks.
*   **Computer Vision**: Image segmentation tasks.
*   **Clustering**: Identifying clusters in data by finding minimum cuts.
*   **Bioinformatics**: Analyzing biological networks.

## Advanced Features (Future Work)

*   O(log N) or O(alpha(N)) min-cut queries using LCA.
*   Support for `no_std` environments (currently `std` is a default feature).
*   More sophisticated graph representations or adapters.
*   Parallelization of max-flow computations within Gusfield's algorithm (if feasible and beneficial).
*   Extraction of the actual min-cut partition (not just the value) from the tree or during its construction.

## Validation

The crate includes:
*   Unit tests for individual components.
*   Integration tests based on academic examples (e.g., small, known graphs).
*   Property-based tests using `proptest` to verify key properties of the Gomory-Hu tree (e.g., cut equivalence, tree structure) over a wide range of randomly generated graphs.

## Contributing

Contributions are welcome! Please feel free to submit issues, bug reports, or pull requests.
For major changes, please open an issue first to discuss the proposed changes.

## License

This project is licensed under the MIT license (`LICENSE-MIT` or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
