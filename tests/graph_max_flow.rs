use itertools::Itertools;
use or_tools::graph::{
    ebert_graph::{ArcIndex, NodeIndex, StarGraph},
    max_flow::{MaxFlow, MaxFlowStatus},
};

#[test]
fn max_flow_simple() {
    // Add each arc.
    // Arc are stored as (begin_node, end_node, capacity)
    let arcs = &[
        (0, 1, 20),
        (0, 2, 30),
        (0, 3, 10),
        (1, 2, 40),
        (1, 4, 30),
        (2, 3, 10),
        (2, 4, 20),
        (3, 2, 5),
        (3, 4, 20),
    ];
    let num_nodes = arcs
        .iter()
        .flat_map(|(start, end, _)| [start, end])
        .unique()
        .count() as NodeIndex;
    let num_arcs = arcs.len() as ArcIndex;

    let mut graph = StarGraph::new(num_nodes, num_arcs);
    let arc_indices = arcs
        .iter()
        .map(|(start, end, _)| graph.add_arc(*start, *end))
        .collect_vec();

    let mut max_flow = MaxFlow::new(&graph, 0 /* node 0 */, num_nodes - 1 /* node 4 */);
    for (arc, (_, _, capacity)) in arc_indices.iter().zip(arcs) {
        max_flow.set_arc_capacity(*arc, *capacity);
    }

    println!(
        "Solving max flow with: {num_nodes} nodes, and {num_arcs} arcs.",
        num_nodes = graph.num_nodes(),
        num_arcs = graph.num_arcs(),
    );

    // Find the maximum flow between node 0 and node 4.
    let output = max_flow.solve().expect("failed to solve maximum flow");
    if output.status() != MaxFlowStatus::Optimal {
        eprintln!("Solving the max flow is not optimal!");
    }
    let total_flow = output.get_optimal_flow();
    println!("Maximum flow: {total_flow}");
    println!();
    println!(" Arc  : Flow / Capacity");
    for arc in arc_indices {
        println!(
            "{tail} -> {head}: {flow} / {capacity}",
            tail = graph.tail(arc),
            head = graph.head(arc),
            flow = output.flow(arc),
            capacity = output.capacity(arc),
        );
    }

    assert_eq!(total_flow, 60);
}
