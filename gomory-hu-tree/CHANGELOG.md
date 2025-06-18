# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - YYYY-MM-DD <!-- TODO: Set actual release date before publishing -->
### Added
*   Initial implementation of Gomory-Hu tree construction using Gusfield's algorithm (`gusfield_tree`).
*   Dinic's algorithm for max-flow calculations (`DinicSolver`), implementing the `MaxFlowSolver` trait.
*   Basic `AdjacencyListFlowGraph` for directed graph representation, using `petgraph` internally. Implements `FlowGraph` and `OriginalGraphView` traits.
*   Core data structures `GomoryHuTree` and `TreeEdge`.
*   API for `GomoryHuTree::min_cut_value(s, t)` to query min-cut values (currently O(N) via BFS).
*   `GomoryHuTree::to_dot()` for exporting the tree structure to DOT format.
*   Error types `GomoryHuError` and `MaxFlowError`.
*   Unit tests and integration tests for academic graph examples.
*   Property-based tests using `proptest` for `gusfield_tree` properties and cut value correctness.
*   Criterion benchmarks for tree construction and query performance on various graph types (random dense/sparse, grid).
*   Comprehensive crate documentation in `src/lib.rs` and a detailed `README.md`.
*   Standard MIT and Apache-2.0 license files.
*   Basic CI setup using GitHub Actions (checks formatting, clippy, build, and tests).
*   Initial `CHANGELOG.md`.
