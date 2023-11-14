use itertools::Itertools;
use or_tools::graph::{
    ebert_graph::{ArcIndex, NodeIndex, StarGraph},
    min_cost_flow::{MinCostFlow, MinCostFlowStatus},
};

#[test]
fn min_cost_flow_simple() {
    // Define supply of each node.
    let supplies = &[(0, 20), (1, 0), (2, 0), (3, -5), (4, -15)];

    // Define each arc
    // Arc are stored as (begin_node, end_node, capacity, unit_cost)
    let arcs = &[
        (0, 1, 15, 4),
        (0, 2, 8, 4),
        (1, 2, 20, 2),
        (1, 3, 4, 2),
        (1, 4, 10, 6),
        (2, 3, 15, 1),
        (2, 4, 4, 3),
        (3, 4, 20, 2),
        (4, 2, 5, 3),
    ];
    let num_nodes = supplies.len() as NodeIndex;
    let num_arcs = arcs.len() as ArcIndex;

    let mut graph = StarGraph::new(num_nodes, num_arcs);
    let arc_indices = arcs
        .iter()
        .map(|(start, end, _, _)| graph.add_arc(*start, *end))
        .collect_vec();

    let mut min_cost_flow = MinCostFlow::new(&graph);
    for (arc, (_, _, capacity, unit_cost)) in arc_indices.iter().zip(arcs) {
        min_cost_flow.set_arc_capacity(*arc, *capacity);
        min_cost_flow.set_arc_unit_cost(*arc, *unit_cost);
    }
    for (node, supply) in supplies {
        min_cost_flow.set_node_supply(*node, *supply);
    }

    println!(
        "Solving min cost flow with: {num_nodes} nodes, and {num_arcs} arcs.",
        num_nodes = graph.num_nodes(),
        num_arcs = graph.num_arcs(),
    );

    // Find the minimum cost flow.
    let mut output = min_cost_flow
        .solve()
        .expect("failed to solve minimum cost flow");
    if output.status() != MinCostFlowStatus::Optimal {
        eprintln!("Solving the min cost flow is not optimal!");
    }
    let total_flow_cost = output.get_optimal_cost();
    println!("Minimum cost flow: {total_flow_cost}");
    println!();
    println!(" Arc  : Flow / Capacity / Cost");
    for arc in arc_indices.iter().copied() {
        println!(
            "{tail} -> {head}: {flow} / {capacity} / {cost}",
            tail = graph.tail(arc),
            head = graph.head(arc),
            flow = output.flow(arc),
            capacity = output.capacity(arc),
            cost = output.unit_cost(arc),
        );
    }

    let get_arc_cost = |tail, head| {
        arcs.iter()
            .zip(&arc_indices)
            .find(|((target_tail, target_head, _, _), _)| {
                tail == *target_tail && head == *target_head
            })
            .map(|(_, arc)| output.unit_cost(*arc))
            .unwrap_or_else(|| panic!("no such arc: ({tail}, {head})"))
    };

    assert_eq!(get_arc_cost(1, 4), 6);
    assert_eq!(get_arc_cost(2, 3), 1);
    assert_eq!(get_arc_cost(3, 4), 2);
    assert_eq!(get_arc_cost(4, 2), 3);
}
