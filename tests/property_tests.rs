use proptest::prelude::*;
use proptest::collection::vec as prop_vec;
use gomory_hu_tree::{
    AdjacencyListFlowGraph,
    DinicSolver,
    gusfield_tree,
    MaxFlowError,
    MaxFlowSolver, // Trait for solver.max_flow_min_cut
};
use gomory_hu_tree::flow::{FlowGraph, OriginalGraphView}; // Traits for graph methods
use std::collections::{VecDeque, HashSet};

const EPSILON: f64 = 1e-9; // For float comparisons

// --- Helper: Graph Connectivity and Pathfinding ---
fn has_path_in_adj_list_graph(graph: &AdjacencyListFlowGraph, s: usize, t: usize) -> bool {
    if s == t { return true; }
    let n = graph.vertex_count(); // Needs FlowGraph in scope
    if s >= n || t >= n { return false; }

    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    queue.push_back(s);
    visited[s] = true;

    while let Some(u) = queue.pop_front() {
        if u == t { return true; }
        for (from, to, capacity) in graph.all_edges() { // Needs OriginalGraphView in scope
            if from == u && capacity > EPSILON && !visited[to] {
                if to < n {
                    visited[to] = true;
                    queue.push_back(to);
                }
            }
        }
    }
    false
}

// --- Proptest Strategies ---
fn arbitrary_graph_strategy(min_nodes: usize, max_nodes: usize) -> impl Strategy<Value = AdjacencyListFlowGraph> {
    (min_nodes..=max_nodes).prop_flat_map(move |num_nodes| {
        if num_nodes == 0 {
            return Just(AdjacencyListFlowGraph::new()).boxed();
        }
        let num_edge_tuples_to_generate = num_nodes * 2;

        (
            Just(num_nodes),
            prop_vec((0..num_nodes, 0..num_nodes, 1.0..100.0f64), 0..=num_edge_tuples_to_generate)
        ).prop_map(|(n, edges_tuples)| {
            let mut graph = AdjacencyListFlowGraph::new();
            for _ in 0..n { graph.add_node(()); }

            let mut actual_edges = HashSet::new();

            for (u, v, cap) in edges_tuples {
                if u != v {
                    if !actual_edges.contains(&(u,v)) {
                        graph.add_edge(u, v, cap);
                        actual_edges.insert((u,v));
                    }
                    if !actual_edges.contains(&(v,u)) {
                        graph.add_edge(v, u, cap);
                        actual_edges.insert((v,u));
                    }
                }
            }
            graph
        })
        .boxed()
    })
}

fn arbitrary_connected_graph_strategy(min_nodes: usize, max_nodes: usize) -> impl Strategy<Value = AdjacencyListFlowGraph> {
    arbitrary_graph_strategy(min_nodes.max(1), max_nodes)
        .prop_map(|mut graph| {
            let n = graph.vertex_count(); // Needs FlowGraph in scope
            if n <= 1 { return graph; }
            for i in 0..(n - 1) {
                 graph.add_edge(i, i + 1, 1.0);
                 graph.add_edge(i + 1, i, 1.0);
            }
            graph
        })
}

fn distinct_vertex_pair_strategy(num_nodes: usize) -> impl Strategy<Value = (usize, usize)> {
    if num_nodes < 2 {
        return Just((0,0)).boxed();
    }
    (0..num_nodes, 0..num_nodes)
        .prop_filter("Vertices s and t must be distinct", |(s, t)| s != t)
        .boxed()
}

// --- Property Tests ---
proptest! {
    #![proptest_config(ProptestConfig {
        cases: 25,
        .. ProptestConfig::default()
    })]

    #[test]
    fn tree_has_correct_number_of_edges(
        graph in arbitrary_connected_graph_strategy(1, 20)
    ) {
        let n = graph.vertex_count(); // Needs FlowGraph in scope
        let solver = DinicSolver::new();
        match gusfield_tree(&graph, &solver) {
            Ok(tree) => {
                if n == 1 {
                    prop_assert_eq!(tree.edge_count(), 0);
                } else {
                    prop_assert_eq!(tree.edge_count(), n - 1, "Tree for {}-node graph has {} edges, expected {}", n, tree.edge_count(), n-1);
                }
            }
            Err(e) => {
                prop_assert!(false, "gusfield_tree failed for {}-node graph: {:?}", n, e);
            }
        }
    }

    #[test]
    fn cut_property_holds(
        graph_data in (2usize..=10usize).prop_flat_map(|num_nodes| { // Fixed range to usize
            (
                Just(num_nodes),
                arbitrary_connected_graph_strategy(num_nodes, num_nodes),
                distinct_vertex_pair_strategy(num_nodes)
            )
        })
    ) {
        let (num_nodes, graph, (s, t)) = graph_data;
        if s == t {
            return Ok(());
        }
        let solver = DinicSolver::new();
        let original_min_cut_val = match solver.max_flow_min_cut(&graph, s, t) { // Needs MaxFlowSolver in scope
            Ok((flow, _min_cut_partition)) => flow,
            Err(MaxFlowError::SourceEqualsSink(_)) => f64::INFINITY,
            Err(MaxFlowError::EmptyGraph) => 0.0,
            Err(e) => {
                 prop_assert!(false, "Reference max_flow_min_cut failed for s={}, t={} on {}-node graph: {:?}", s, t, num_nodes, e);
                return Ok(());
            }
        };
        match gusfield_tree(&graph, &solver) {
            Ok(gh_tree) => {
                let tree_cut_val = gh_tree.min_cut_value(s, t);
                if original_min_cut_val.is_infinite() {
                    prop_assert!(tree_cut_val.is_infinite(), "Original cut is Inf, tree cut is {}", tree_cut_val);
                } else if tree_cut_val.is_infinite() {
                    prop_assert!(original_min_cut_val.is_infinite(), "Tree cut is Inf, original cut is {}", original_min_cut_val);
                } else {
                     prop_assert!(
                        (tree_cut_val - original_min_cut_val).abs() < EPSILON,
                        "Cut property failed for s={}, t={}. Tree: {}, Original: {}. Graph has {} nodes.",
                        s, t, tree_cut_val, original_min_cut_val, num_nodes
                    );
                }
            }
            Err(e) => {
                prop_assert!(false, "gusfield_tree construction failed for {}-node graph: {:?}", num_nodes, e);
            }
        }
    }

    #[test]
    fn tree_connectivity_matches_graph(
        graph_data in (0usize..=10usize).prop_flat_map(|num_nodes| { // Fixed range to usize
            (
                Just(num_nodes),
                arbitrary_graph_strategy(num_nodes, num_nodes),
                if num_nodes == 0 {
                    (Just(0), Just(0)).boxed()
                } else {
                    (0..num_nodes, 0..num_nodes).boxed()
                }
            )
        })
    ) {
        let (num_nodes, graph, (s, t)) = graph_data;
        if num_nodes == 0 {
            return Ok(());
        }
        if s == t { return Ok(()); }
        let graph_connected_st = has_path_in_adj_list_graph(&graph, s, t);
        let solver = DinicSolver::new();
        match gusfield_tree(&graph, &solver) {
            Ok(gh_tree) => {
                let tree_min_cut_val = gh_tree.min_cut_value(s, t);
                let tree_implies_connected = tree_min_cut_val > EPSILON;
                prop_assert_eq!(
                    tree_implies_connected, graph_connected_st,
                    "Connectivity mismatch for s={}, t={}. Tree says connected: {}, Graph says connected: {}. Min-cut val: {}. Graph nodes: {}",
                    s, t, tree_implies_connected, graph_connected_st, tree_min_cut_val, num_nodes
                );
            }
            Err(e) => {
                prop_assert!(false, "gusfield_tree construction failed for connectivity test on {}-node graph: {:?}", num_nodes, e);
            }
        }
    }
}
