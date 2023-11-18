use std::{
    ffi::c_int,
    time::{Duration, Instant},
};

use or_tools::constraint_solver::{
    routing::{RoutingModel, RoutingModelStatus},
    routing_enums::FirstSolutionStrategy,
    routing_index_manager::{
        RoutingIndexManager, RoutingIndexManagerVehiclePlan, RoutingNodeIndex,
    },
    routing_parameters::RoutingSearchParameters,
};

#[test]
fn vrp_pickup_delivery_simple() {
    // Instantiate the data problem.
    let distance_matrix = &[
        [
            0, 548, 776, 696, 582, 274, 502, 194, 308, 194, 536, 502, 388, 354, 468, 776, 662,
        ],
        [
            548, 0, 684, 308, 194, 502, 730, 354, 696, 742, 1084, 594, 480, 674, 1016, 868, 1210,
        ],
        [
            776, 684, 0, 992, 878, 502, 274, 810, 468, 742, 400, 1278, 1164, 1130, 788, 1552, 754,
        ],
        [
            696, 308, 992, 0, 114, 650, 878, 502, 844, 890, 1232, 514, 628, 822, 1164, 560, 1358,
        ],
        [
            582, 194, 878, 114, 0, 536, 764, 388, 730, 776, 1118, 400, 514, 708, 1050, 674, 1244,
        ],
        [
            274, 502, 502, 650, 536, 0, 228, 308, 194, 240, 582, 776, 662, 628, 514, 1050, 708,
        ],
        [
            502, 730, 274, 878, 764, 228, 0, 536, 194, 468, 354, 1004, 890, 856, 514, 1278, 480,
        ],
        [
            194, 354, 810, 502, 388, 308, 536, 0, 342, 388, 730, 468, 354, 320, 662, 742, 856,
        ],
        [
            308, 696, 468, 844, 730, 194, 194, 342, 0, 274, 388, 810, 696, 662, 320, 1084, 514,
        ],
        [
            194, 742, 742, 890, 776, 240, 468, 388, 274, 0, 342, 536, 422, 388, 274, 810, 468,
        ],
        [
            536, 1084, 400, 1232, 1118, 582, 354, 730, 388, 342, 0, 878, 764, 730, 388, 1152, 354,
        ],
        [
            502, 594, 1278, 514, 400, 776, 1004, 468, 810, 536, 878, 0, 114, 308, 650, 274, 844,
        ],
        [
            388, 480, 1164, 628, 514, 662, 890, 354, 696, 422, 764, 114, 0, 194, 536, 388, 730,
        ],
        [
            354, 674, 1130, 822, 708, 628, 856, 320, 662, 388, 730, 308, 194, 0, 342, 422, 536,
        ],
        [
            468, 1016, 788, 1164, 1050, 514, 514, 662, 320, 274, 388, 650, 536, 342, 0, 764, 194,
        ],
        [
            776, 868, 1552, 560, 674, 1050, 1278, 742, 1084, 810, 1152, 274, 388, 422, 764, 0, 798,
        ],
        [
            662, 1210, 754, 1358, 1244, 708, 480, 856, 514, 468, 354, 844, 730, 536, 194, 798, 0,
        ],
    ];

    let pickups_deliveries = &[
        [RoutingNodeIndex::new(1), RoutingNodeIndex::new(6)],
        [RoutingNodeIndex::new(2), RoutingNodeIndex::new(10)],
        [RoutingNodeIndex::new(4), RoutingNodeIndex::new(3)],
        [RoutingNodeIndex::new(5), RoutingNodeIndex::new(9)],
        [RoutingNodeIndex::new(7), RoutingNodeIndex::new(8)],
        [RoutingNodeIndex::new(15), RoutingNodeIndex::new(11)],
        [RoutingNodeIndex::new(13), RoutingNodeIndex::new(12)],
        [RoutingNodeIndex::new(16), RoutingNodeIndex::new(14)],
    ];

    let num_nodes = distance_matrix.len() as c_int;
    let num_vehicles = 4;
    let depot = RoutingNodeIndex::new(0);

    // Create Routing Index Manager
    let manager = RoutingIndexManager::new(
        num_nodes,
        num_vehicles,
        RoutingIndexManagerVehiclePlan::Depot(depot),
    );

    // Create Routing Model.
    let mut routing = RoutingModel::new(&manager, None);

    // Define cost of each arc.
    let transit_callback = |from_index, to_index| {
        let from_node = manager.index_to_node(from_index).value() as usize;
        let to_node = manager.index_to_node(to_index).value() as usize;
        distance_matrix[from_node][to_node]
    };
    let transit_callback_index = routing.register_transit_callback(&transit_callback);
    routing.set_arc_cost_evaluator_of_all_vehicles(transit_callback_index);

    // Add Distance constraint.
    routing.add_dimension(
        transit_callback_index, // transit callback
        0,                      // no slack
        3000,                   // vehicle maximum travel distance
        true,                   // start cumul to zero
        "Distance",
    );
    let distance_dimension = routing
        .get_mutable_dimension("Distance")
        .expect("failed to find dimension");
    distance_dimension.set_global_span_cost_coefficient(100);

    // Define Transportation Requests.
    let solver = routing.solver();
    for [pickup_node, delivery_node] in pickups_deliveries {
        let pickup_index = manager.node_to_index(pickup_node);
        let delivery_index = manager.node_to_index(delivery_node);
        routing.add_pickup_and_delivery(pickup_index, delivery_index);

        let pickup_var = routing
            .vehicle_var(pickup_index)
            .expect("failed to get vehicle");
        let delivery_var = routing
            .vehicle_var(delivery_index)
            .expect("failed to get vehicle");

        let constraint = solver.make_equality(pickup_var, delivery_var);
        solver.add_constraint(constraint);

        let constraint = solver.make_less_or_equal(
            distance_dimension
                .cumul_var(pickup_index)
                .expect("failed to get cumul var"),
            distance_dimension
                .cumul_var(delivery_index)
                .expect("failed to get cumul var"),
        );
        solver.add_constraint(constraint);
    }

    // Setting first solution heuristic.
    let mut search_parameters = RoutingSearchParameters::new();
    search_parameters.set_first_solution_strategy(FirstSolutionStrategy::ParallelCheapestInsertion);
    search_parameters.set_time_limit(Duration::from_secs(1_000));

    // Solve the problem.
    let instant = Instant::now();
    let solution = routing.solve_with_parameters(&search_parameters);
    let elapsed_ms = instant.elapsed().as_millis();

    assert!(solution.has_contents());

    // Check the status.
    let status = solution.status();
    if !matches!(
        status,
        RoutingModelStatus::RoutingSuccess
            | RoutingModelStatus::RoutingPartialSuccessLocalOptimumNotReached
    ) {
        panic!("failed to solve the routing problem: {status:?}");
    }

    // Print solution on console.
    let mut total_distance = 0;
    for vehicle_id in 0..num_vehicles {
        let mut index = routing.start(vehicle_id);
        println!("Route for Vehicle {index}:");

        let mut route_distance = 0;
        while !routing.is_end(index) {
            print!("{} -> ", manager.index_to_node(index).value());
            let previous_index = index;
            index = solution
                .value(routing.next_var(index).expect("failed to get next var"))
                .expect("failed to get value");
            route_distance +=
                routing.get_arc_cost_for_vehicle(previous_index, index, vehicle_id as i64);
        }
        println!("{}", manager.index_to_node(index).value());
        println!("Distance of the route: {route_distance}m");
        total_distance += route_distance;
    }
    println!("Total distance of all routes: {total_distance}m");
    println!();
    println!("Advanced usage:");
    println!("Problem solved in {elapsed_ms}ms");
}
