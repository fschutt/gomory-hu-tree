use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use gomory_hu_tree::{
    AdjacencyListFlowGraph, // Concrete graph implementation
    DinicSolver,            // Max-flow solver
    gusfield_tree,          // Gomory-Hu tree algorithm
    // GomoryHuTree,        // Type is inferred or not strictly needed for benchmark logic
    // MaxFlowError,        // Not directly used in benchmark setup logic much
};
use fastrand; // For random graph generation and query pair selection

// --- Graph Generation Helpers ---

// Generates a random graph with `num_nodes` and a target `edge_density`.
// `edge_density` = 1.0 means a complete graph.
// `edge_density` = 0.0 means no edges beyond what might be needed for connectivity (if forced).
fn generate_random_graph(num_nodes: usize, edge_density: f64) -> AdjacencyListFlowGraph {
    let mut graph = AdjacencyListFlowGraph::new();
    if num_nodes == 0 {
        return graph;
    }
    for _ in 0..num_nodes {
        graph.add_node(());
    }

    for u in 0..num_nodes {
        for v in 0..num_nodes {
            if u == v {
                continue;
            }
            if u < v && fastrand::f64() < edge_density {
                let capacity = fastrand::f64() * 99.0 + 1.0; // Capacity between 1 and 100
                graph.add_edge(u, v, capacity);
                graph.add_edge(v, u, capacity);
            }
        }
    }
    graph
}

// Generates a 2D grid graph.
fn generate_grid_graph(side_length: usize) -> AdjacencyListFlowGraph {
    let mut graph = AdjacencyListFlowGraph::new();
    if side_length == 0 {
        return graph;
    }
    let num_nodes = side_length * side_length;
    for _ in 0..num_nodes {
        graph.add_node(());
    }

    for r in 0..side_length {
        for c in 0..side_length {
            let u = r * side_length + c;
            if c + 1 < side_length {
                let v_right = r * side_length + (c + 1);
                let capacity = fastrand::f64() * 99.0 + 1.0;
                graph.add_edge(u, v_right, capacity);
                graph.add_edge(v_right, u, capacity);
            }
            if r + 1 < side_length {
                let v_bottom = (r + 1) * side_length + c;
                let capacity = fastrand::f64() * 99.0 + 1.0;
                graph.add_edge(u, v_bottom, capacity);
                graph.add_edge(v_bottom, u, capacity);
            }
        }
    }
    graph
}


// --- Benchmarks ---

fn gomory_hu_construction_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Gomory-Hu Construction");

    let dense_graph_sizes = [10, 20, 30];
    let sparse_graph_sizes = [10, 30, 50, 70];
    let grid_side_lengths = [3, 5, 7];

    for &size in dense_graph_sizes.iter() {
        group.throughput(Throughput::Elements(size as u64));
        let dense_graph = generate_random_graph(size, 0.7);
        group.bench_with_input(
            BenchmarkId::new("Dense Random", size),
            &dense_graph,
            |b, g| {
                let solver = DinicSolver::new();
                b.iter(|| gusfield_tree(g, &solver).unwrap())
            }
        );
    }

    for &size in sparse_graph_sizes.iter() {
        group.throughput(Throughput::Elements(size as u64));
        let sparse_graph = generate_random_graph(size, 0.1);
        group.bench_with_input(
            BenchmarkId::new("Sparse Random", size),
            &sparse_graph,
            |b, g| {
                let solver = DinicSolver::new();
                b.iter(|| gusfield_tree(g, &solver).unwrap())
            }
        );
    }

    for &side_len in grid_side_lengths.iter() {
        let num_nodes = side_len * side_len;
        group.throughput(Throughput::Elements(num_nodes as u64));
        let grid_graph = generate_grid_graph(side_len);
        group.bench_with_input(
            BenchmarkId::new("Grid Graph", num_nodes),
            &grid_graph,
            |b, g| {
                let solver = DinicSolver::new();
                b.iter(|| gusfield_tree(g, &solver).unwrap())
            }
        );
    }

    group.finish();
}

fn min_cut_query_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Min-Cut Queries from Gomory-Hu Tree");

    let query_graph_sizes = [30, 50, 100];
    const NUM_QUERIES_PER_TREE: usize = 100;

    for &size in query_graph_sizes.iter() {
        let graph = generate_random_graph(size, 0.2);
        let solver = DinicSolver::new();
        let tree = gusfield_tree(&graph, &solver).unwrap();

        group.throughput(Throughput::Elements(NUM_QUERIES_PER_TREE as u64));
        group.bench_with_input(
            BenchmarkId::new("Random Queries", size),
            &tree,
            |b, gh_tree| {
                b.iter(|| {
                    for _ in 0..NUM_QUERIES_PER_TREE {
                        if gh_tree.vertex_count() < 2 { continue; }
                        let s = fastrand::usize(0..gh_tree.vertex_count());
                        let t = fastrand::usize(0..gh_tree.vertex_count());
                        if s != t {
                            criterion::black_box(gh_tree.min_cut_value(s, t));
                        } else {
                             criterion::black_box(gh_tree.min_cut_value(s, s));
                        }
                    }
                })
            }
        );
    }

    group.finish();
}

criterion_group!(benches, gomory_hu_construction_benchmarks, min_cut_query_benchmarks);
criterion_main!(benches);
