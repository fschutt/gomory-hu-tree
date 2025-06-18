use gomory_hu_tree::{
    gusfield_tree, // Gomory-Hu tree algorithm
    // GomoryHuTree,        // Not directly used for assertions here, but result is this type
    // TreeEdge,            // Not directly used
    AdjacencyListFlowGraph, // Concrete graph implementation
    DinicSolver,            // Max-flow solver
};

// Helper for float comparisons
const EPSILON: f64 = 1e-9;

#[test]
fn test_triangle_graph() {
    // Interpreted as a line graph 0-1-2 to match report's expected output of all pairwise min-cuts = 10.0
    // Nodes 0, 1, 2
    // Edges (0,1) cap 10, (1,2) cap 10
    let mut graph = AdjacencyListFlowGraph::new();
    let _ = graph.add_node(()); // Node 0
    let _ = graph.add_node(()); // Node 1
    let _ = graph.add_node(()); // Node 2

    graph.add_edge(0, 1, 10.0);
    graph.add_edge(1, 0, 10.0); // Reverse edge for undirected capacity

    graph.add_edge(1, 2, 10.0);
    graph.add_edge(2, 1, 10.0); // Reverse edge for undirected capacity

    let solver = DinicSolver::new();
    let gh_tree = gusfield_tree(&graph, &solver)
        .expect("Gusfield tree construction failed for line graph (triangle test)");

    // For line graph 0-1-2 with capacities 10:
    // min_cut(0,1) = 10.
    // min_cut(1,2) = 10.
    // min_cut(0,2) = min(cap(0,1), cap(1,2)) = 10.
    assert!((gh_tree.min_cut_value(0, 1) - 10.0).abs() < EPSILON);
    assert!((gh_tree.min_cut_value(1, 2) - 10.0).abs() < EPSILON);
    assert!((gh_tree.min_cut_value(0, 2) - 10.0).abs() < EPSILON);
}

#[test]
fn test_star_graph() {
    // Center node 0, spokes 1, 2, 3, 4, 5
    // Edge (0,i) has capacity i. (Using 1-based index for capacity for simplicity of spoke id)
    let mut graph = AdjacencyListFlowGraph::new();
    let _center = graph.add_node(()); // Node 0 (center)
    let num_spokes = 5;
    let mut spoke_nodes_indices = Vec::new(); // Store actual graph indices of spoke nodes

    for _i in 0..num_spokes {
        // Add spoke nodes first
        spoke_nodes_indices.push(graph.add_node(()));
    }

    for (i, spoke_node_graph_idx) in spoke_nodes_indices.iter().enumerate().take(num_spokes) {
        // Then add edges
        let capacity = (i + 1) as f64; // Capacity for spoke i+1 is (i+1).0
        graph.add_edge(0, *spoke_node_graph_idx, capacity);
        graph.add_edge(*spoke_node_graph_idx, 0, capacity); // Undirected
    }

    let solver = DinicSolver::new();
    let gh_tree =
        gusfield_tree(&graph, &solver).expect("Gusfield tree construction failed for star graph");

    // Min-cut between any two spokes s_i, s_j should be min(cap(0,s_i), cap(0,s_j))
    // Spoke nodes are indexed 1 through 5 in terms of problem description,
    // corresponding to spoke_nodes_indices[0] through spoke_nodes_indices[4]
    for i in 0..num_spokes {
        for j in (i + 1)..num_spokes {
            let node_idx_i = spoke_nodes_indices[i]; // Actual graph node index for spoke (i+1)
            let node_idx_j = spoke_nodes_indices[j]; // Actual graph node index for spoke (j+1)

            let cap_i = (i + 1) as f64; // Capacity of edge (0, node_idx_i)
            let cap_j = (j + 1) as f64; // Capacity of edge (0, node_idx_j)

            let expected_min_cut = cap_i.min(cap_j);
            let actual_min_cut = gh_tree.min_cut_value(node_idx_i, node_idx_j);

            assert!(
                (actual_min_cut - expected_min_cut).abs() < EPSILON,
                "Min-cut between spoke {} (node {}) and spoke {} (node {}) was {}, expected {}",
                i + 1,
                node_idx_i,
                j + 1,
                node_idx_j,
                actual_min_cut,
                expected_min_cut
            );
        }
    }

    // Min-cut between center (0) and any spoke s_k (node index spoke_nodes_indices[k-1]) should be cap(0,s_k)
    for (i, spoke_node_graph_idx) in spoke_nodes_indices.iter().enumerate().take(num_spokes) {
        let expected_min_cut = (i + 1) as f64; // Capacity of edge (0, spoke_node_graph_idx)
        let actual_min_cut = gh_tree.min_cut_value(0, *spoke_node_graph_idx); // 0 is center node index
        assert!(
            (actual_min_cut - expected_min_cut).abs() < EPSILON,
            "Min-cut between center (0) and spoke {} (node {}) was {}, expected {}",
            i + 1,
            spoke_node_graph_idx,
            actual_min_cut,
            expected_min_cut
        );
    }
}
