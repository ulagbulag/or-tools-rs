use std::{ffi::c_int, marker::PhantomData, mem::transmute};

use libffi::high::Closure2;

use crate::sat::cp_model::IntVar;

use super::{
    routing_index_manager::RoutingIndexManager,
    routing_parameters::{RoutingModelParameters, RoutingSearchParameters},
    Solver,
};

// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/constraint_solver/routing.h"
}}

cpp_class!(
    #[doc(hidden)]
    unsafe struct RoutingModelInner as "operations_research::RoutingModel"
);

/// The vehicle routing library lets one model and solve generic vehicle routing
/// problems ranging from the Traveling Salesman Problem to more complex
/// problems such as the Capacitated Vehicle Routing Problem with Time Windows.
///
/// The objective of a vehicle routing problem is to build routes covering a set
/// of nodes minimizing the overall cost of the routes (usually proportional to
/// the sum of the lengths of each segment of the routes) while respecting some
/// problem-specific constraints (such as the length of a route). A route is
/// equivalent to a path connecting nodes, starting/ending at specific
/// starting/ending nodes.
///
/// The term "vehicle routing" is historical and the category of problems solved
/// is not limited to the routing of vehicles: any problem involving finding
/// routes visiting a given number of nodes optimally falls under this category
/// of problems, such as finding the optimal sequence in a playlist.
/// The literature around vehicle routing problems is extremely dense but one
/// can find some basic introductions in the following links:
/// - http://en.wikipedia.org/wiki/Travelling_salesman_problem
/// - http://www.tsp.gatech.edu/history/index.html
/// - http://en.wikipedia.org/wiki/Vehicle_routing_problem
///
/// The vehicle routing library is a vertical layer above the constraint
/// programming library (ortools/constraint_programming:cp).
/// One has access to all underlying constrained variables of the vehicle
/// routing model which can therefore be enriched by adding any constraint
/// available in the constraint programming library.
///
/// There are two sets of variables available:
/// - path variables:
///   * "next(i)" variables representing the immediate successor of the node
///     corresponding to i; use IndexToNode() to get the node corresponding to
///     a "next" variable value; note that node indices are strongly typed
///     integers (cf. ortools/base/int_type.h);
///   * "vehicle(i)" variables representing the vehicle route to which the
///     node corresponding to i belongs;
///   * "active(i)" boolean variables, true if the node corresponding to i is
///     visited and false if not; this can be false when nodes are either
///     optional or part of a disjunction;
///   * The following relationships hold for all i:
///      active(i) == 0 <=> next(i) == i <=> vehicle(i) == -1,
///      next(i) == j => vehicle(j) == vehicle(i).
/// - dimension variables, used when one is accumulating quantities along
///   routes, such as weight or volume carried, distance or time:
///   * "cumul(i,d)" variables representing the quantity of dimension d when
///     arriving at the node corresponding to i;
///   * "transit(i,d)" variables representing the quantity of dimension d added
///     after visiting the node corresponding to i.
///   * The following relationship holds for all (i,d):
///       next(i) == j => cumul(j,d) == cumul(i,d) + transit(i,d).
/// Solving the vehicle routing problems is mainly done using approximate
/// methods (namely local search,
/// cf. http://en.wikipedia.org/wiki/Local_search_(optimization) ), potentially
/// combined with exact techniques based on dynamic programming and exhaustive
/// tree search.
// TODO(user): Add a section on costs (vehicle arc costs, span costs,
//                disjunctions costs).
//
/// Advanced tips: Flags are available to tune the search used to solve routing
/// problems. Here is a quick overview of the ones one might want to modify:
/// - Limiting the search for solutions:
///   * routing_solution_limit (default: kint64max): stop the search after
///     finding 'routing_solution_limit' improving solutions;
///   * routing_time_limit (default: kint64max): stop the search after
///     'routing_time_limit' milliseconds;
/// - Customizing search:
///   * routing_first_solution (default: select the first node with an unbound
///     successor and connect it to the first available node): selects the
///     heuristic to build a first solution which will then be improved by local
///     search; possible values are GlobalCheapestArc (iteratively connect two
///     nodes which produce the cheapest route segment), LocalCheapestArc
///     (select the first node with an unbound successor and connect it to the
///     node which produces the cheapest route segment), PathCheapestArc
///     (starting from a route "start" node, connect it to the node which
///     produces the cheapest route segment, then extend the route by iterating
///     on the last node added to the route).
///   * Local search neighborhoods:
///     - routing_no_lns (default: false): forbids the use of Large Neighborhood
///       Search (LNS); LNS can find good solutions but is usually very slow.
///       Refer to the description of PATHLNS in the LocalSearchOperators enum
///       in constraint_solver.h for more information.
///     - routing_no_tsp (default: true): forbids the use of exact methods to
///       solve "sub"-traveling salesman problems (TSPs) of the current model
///       (such as sub-parts of a route, or one route in a multiple route
///       problem). Uses dynamic programming to solve such TSPs with a maximum
///       size (in number of nodes) up to cp_local_search_tsp_opt_size (flag
///       with a default value of 13 nodes). It is not activated by default
///       because it can slow down the search.
///   * Meta-heuristics: used to guide the search out of local minima found by
///     local search. Note that, in general, a search with metaheuristics
///     activated never stops, therefore one must specify a search limit.
///     Several types of metaheuristics are provided:
///     - routing_guided_local_search (default: false): activates guided local
///       search (cf. http://en.wikipedia.org/wiki/Guided_Local_Search);
///       this is generally the most efficient metaheuristic for vehicle
///       routing;
///     - routing_simulated_annealing (default: false): activates simulated
///       annealing (cf. http://en.wikipedia.org/wiki/Simulated_annealing);
///     - routing_tabu_search (default: false): activates tabu search (cf.
///       http://en.wikipedia.org/wiki/Tabu_search).
///
/// Code sample:
/// Here is a simple example solving a traveling salesman problem given a cost
/// function callback (returns the cost of a route segment):
///
/// - Define a custom distance/cost function from an index to another; in this
///   example just returns the sum of the indices:
///
///     int64_t MyDistance(int64_t from, int64_t to) {
///       return from + to;
///     }
///
/// - Create a routing model for a given problem size (int number of nodes) and
///   number of routes (here, 1):
///
///     RoutingIndexManager manager(...number of nodes..., 1);
///     RoutingModel routing(manager);
///
/// - Set the cost function by registering an std::function<int64_t(int64_t,
/// int64_t)> in the model and passing its index as the vehicle cost.
///
///    const int cost = routing.RegisterTransitCallback(MyDistance);
///    routing.SetArcCostEvaluatorOfAllVehicles(cost);
///
/// - Find a solution using Solve(), returns a solution if any (owned by
///   routing):
///
///    const Assignment* solution = routing.Solve();
///    CHECK(solution != nullptr);
///
/// - Inspect the solution cost and route (only one route here):
///
///    LOG(INFO) << "Cost " << solution->ObjectiveValue();
///    const int route_number = 0;
///    for (int64_t node = routing.Start(route_number);
///         !routing.IsEnd(node);
///         node = solution->Value(routing.NextVar(node))) {
///      LOG(INFO) << manager.IndexToNode(node);
///    }
///
///
/// Keywords: Vehicle Routing, Traveling Salesman Problem, TSP, VRP, CVRPTW,
/// PDP.
pub struct RoutingModel<'manager> {
    /// Lifetime limiter for graph
    _manager: PhantomData<&'manager ()>,
    /// Original model
    inner: Box<RoutingModelInner>,

    // Owned parameters
    /// Owned transit callback
    transit_callbacks: Vec<Closure2<'manager, i64, i64, i64>>,
}

impl<'manager> RoutingModel<'manager> {
    /// Constructor taking an index manager. The version which does not take
    /// RoutingModelParameters is equivalent to passing
    /// DefaultRoutingModelParameters().
    pub fn new(
        index_manager: &'manager RoutingIndexManager,
        parameters: Option<RoutingModelParameters>,
    ) -> Self {
        Self {
            _manager: PhantomData,
            inner: match parameters {
                Some(parameters) => unsafe {
                    cpp!([
                        index_manager as "const operations_research::RoutingIndexManager*",
                        parameters as "operations_research::RoutingModelParameters"
                    ] -> Box<RoutingModelInner> as "operations_research::RoutingModel*"
                        {
                            return new operations_research::RoutingModel(
                                *index_manager,
                                parameters
                            );
                        }
                    )
                },
                None => unsafe {
                    cpp!([
                        index_manager as "const operations_research::RoutingIndexManager*"
                    ] -> Box<RoutingModelInner> as "operations_research::RoutingModel*"
                        {
                            return new operations_research::RoutingModel(
                                *index_manager
                            );
                        }
                    )
                },
            },
            transit_callbacks: Vec::default(),
        }
    }

    pub fn register_transit_callback<'a, F>(&'a mut self, callback: &'a F) -> c_int
    where
        F: 'a + Fn(i64, i64) -> i64,
    {
        let inner = self.inner.as_mut();

        let closure: Closure2<'manager, i64, i64, i64> =
            unsafe { transmute(Closure2::new(callback)) };
        let &f_ptr = closure.code_ptr();
        self.transit_callbacks.push(closure);

        unsafe {
            cpp!([
                inner as "operations_research::RoutingModel*",
                f_ptr as "const void*"
            ] -> c_int as "int"
                {
                    int64_t (*f)(int64_t, int64_t) = (int64_t (*)(int64_t, int64_t))f_ptr;

                    const operations_research::RoutingTransitCallback2 callback = [f](
                        const int64_t from_index,
                        const int64_t to_index
                    ) -> int64_t {
                        return f(from_index, to_index);
                    };
                    return inner->RegisterTransitCallback(callback);
                }
            )
        }
    }

    // Model creation

    /// Methods to add dimensions to routes; dimensions represent quantities
    /// accumulated at nodes along the routes. They represent quantities such as
    /// weights or volumes carried along the route, or distance or times.
    /// Quantities at a node are represented by "cumul" variables and the increase
    /// or decrease of quantities between nodes are represented by "transit"
    /// variables. These variables are linked as follows:
    /// if j == next(i), cumul(j) = cumul(i) + transit(i, j) + slack(i)
    /// where slack is a positive slack variable (can represent waiting times for
    /// a time dimension).
    /// Setting the value of fix_start_cumul_to_zero to true will force the
    /// "cumul" variable of the start node of all vehicles to be equal to 0.

    /// Creates a dimension where the transit variable is constrained to be
    /// equal to evaluator(i, next(i)); 'slack_max' is the upper bound of the
    /// slack variable and 'capacity' is the upper bound of the cumul variables.
    /// 'name' is the name used to reference the dimension; this name is used to
    /// get cumul and transit variables from the routing model.
    /// Returns false if a dimension with the same name has already been created
    /// (and doesn't create the new dimension).
    /// Takes ownership of the callback 'evaluator'.
    pub fn add_dimension(
        &mut self,
        evaluator_index: c_int,
        slack_max: i64,
        capacity: i64,
        fix_start_cumul_to_zero: bool,
        name: &str,
    ) -> bool {
        let inner = self.inner.as_mut();

        let name_ptr = name.as_ptr();
        let name_len = name.len();

        unsafe {
            cpp!([
                inner as "operations_research::RoutingModel*",
                evaluator_index as "int",
                slack_max as "int64_t",
                capacity as "int64_t",
                fix_start_cumul_to_zero as "bool",
                name_ptr as "const char*",
                name_len as "size_t"
            ] -> bool as "bool"
                {
                    std::string name = std::string(name_ptr, name_len);

                    return inner->AddDimension(
                        evaluator_index,
                        slack_max,
                        capacity,
                        fix_start_cumul_to_zero,
                        name
                    );
                }
            )
        }
    }

    /// Returns a dimension from its name. Returns nullptr if the dimension does
    /// not exist.
    pub fn get_mutable_dimension(&self, dimension_name: &str) -> Option<&mut RoutingDimension> {
        let inner = self.inner.as_ref();

        let dimension_name_ptr = dimension_name.as_ptr();
        let dimension_name_len = dimension_name.len();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                dimension_name_ptr as "const char*",
                dimension_name_len as "size_t"
            ] -> *mut RoutingDimension as "operations_research::RoutingDimension*"
                {
                    std::string dimension_name = std::string(dimension_name_ptr, dimension_name_len);

                    return inner->GetMutableDimension(dimension_name);
                }
            )
            .as_mut()
        }
    }

    /// Notifies that index1 and index2 form a pair of nodes which should belong
    /// to the same route. This methods helps the search find better solutions,
    /// especially in the local search phase.
    /// It should be called each time you have an equality constraint linking
    /// the vehicle variables of two node (including for instance pickup and
    /// delivery problems):
    ///     Solver* const solver = routing.solver();
    ///     int64_t index1 = manager.NodeToIndex(node1);
    ///     int64_t index2 = manager.NodeToIndex(node2);
    ///     solver->AddConstraint(solver->MakeEquality(
    ///         routing.VehicleVar(index1),
    ///         routing.VehicleVar(index2)));
    ///     routing.AddPickupAndDelivery(index1, index2);
    ///
    // TODO(user): Remove this when model introspection detects linked nodes.
    pub fn add_pickup_and_delivery(&self, pickup: i64, delivery: i64) {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "operations_research::RoutingModel*",
                pickup as "int64_t",
                delivery as "int64_t"
            ]
                {
                    return inner->AddPickupAndDelivery(
                        pickup,
                        delivery
                    );
                }
            )
        }
    }

    /// Sets the cost function of the model such that the cost of a segment of a
    /// route between node 'from' and 'to' is evaluator(from, to), whatever the
    /// route or vehicle performing the route.
    pub fn set_arc_cost_evaluator_of_all_vehicles(&mut self, evaluator_index: c_int) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::RoutingModel*",
                evaluator_index as "int"
            ]
                {
                    return inner->SetArcCostEvaluatorOfAllVehicles(evaluator_index);
                }
            )
        }
    }

    /// Solves the current routing model with the given parameters. If 'solutions'
    /// is specified, it will contain the k best solutions found during the search
    /// (from worst to best, including the one returned by this method), where k
    /// corresponds to the 'number_of_solutions_to_collect' in
    /// 'search_parameters'. Note that the Assignment returned by the method and
    /// the ones in solutions are owned by the underlying solver and should not be
    /// deleted.
    pub fn solve_with_parameters(&self, search_parameters: &RoutingSearchParameters) -> Assignment {
        let inner = self.inner.as_ref();

        Assignment {
            inner: unsafe {
                cpp!([
                    inner as "operations_research::RoutingModel*",
                    search_parameters as "const operations_research::RoutingSearchParameters*"
                ] -> &super::Assignment as "const operations_research::Assignment*"
                    {
                        return inner->SolveWithParameters(*search_parameters);
                    }
                )
            },
            model: self,
        }
    }

    // Model inspection.

    /// Returns the variable index of the starting node of a vehicle route.
    pub fn start(&self, vehicle: c_int) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                vehicle as "int"
            ] -> i64 as "int64_t"
                {
                    return inner->Start(vehicle);
                }
            )
        }
    }

    /// Returns the variable index of the ending node of a vehicle route.
    pub fn end(&self, vehicle: c_int) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                vehicle as "int"
            ] -> i64 as "int64_t"
                {
                    return inner->End(vehicle);
                }
            )
        }
    }

    /// Returns true if 'index' represents the first node of a route.
    pub fn is_start(&self, index: i64) -> bool {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> bool as "bool"
                {
                    return inner->IsStart(index);
                }
            )
        }
    }

    /// Returns true if 'index' represents the last node of a route.
    pub fn is_end(&self, index: i64) -> bool {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> bool as "bool"
                {
                    return inner->IsEnd(index);
                }
            )
        }
    }

    /// Returns the vehicle of the given start/end index, and None if the given
    /// index is not a vehicle start/end.
    pub fn vehicle_index(&self, index: i64) -> Option<c_int> {
        let inner = self.inner.as_ref();

        let vehicle = unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> c_int as "int"
                {
                    return inner->VehicleIndex(index);
                }
            )
        };

        if vehicle == -1 {
            None
        } else {
            Some(vehicle)
        }
    }

    /// Returns the next variable of the node corresponding to index. Note that
    /// NextVar(index) == index is equivalent to ActiveVar(index) == 0.
    pub fn next_var(&self, index: i64) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->NextVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Returns the active variable of the node corresponding to index.
    pub fn active_var(&self, index: i64) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->ActiveVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Returns the active variable of the node corresponding to index.
    pub fn active_vehicle_var(&self, vehicle: c_int) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                vehicle as "int"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->ActiveVehicleVar(vehicle);
                }
            )
            .as_ref()
        }
    }

    /// Returns the variable specifying whether or not the given vehicle route is
    /// considered for costs and constraints. It will be equal to 1 iff the route
    /// of the vehicle is not empty OR vehicle_used_when_empty_[vehicle] is true.
    pub fn active_route_considered_var(&self, vehicle: c_int) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                vehicle as "int"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->VehicleRouteConsideredVar(vehicle);
                }
            )
            .as_ref()
        }
    }

    /// Returns the vehicle variable of the node corresponding to index. Note that
    /// VehicleVar(index) == -1 is equivalent to ActiveVar(index) == 0.
    pub fn vehicle_var(&self, index: i64) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->VehicleVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Returns the resource variable for the given vehicle index in the given
    /// resource group. If a vehicle doesn't require a resource from the
    /// corresponding resource group, then ResourceVar(v, r_g) == -1.
    pub fn resource_var(&self, vehicle: c_int, resource_group: c_int) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                vehicle as "int",
                resource_group as "int"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->ResourceVar(vehicle, resource_group);
                }
            )
            .as_ref()
        }
    }

    /// Returns the global cost variable which is being minimized.
    pub fn cost_var(&self) -> Option<&IntVar> {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return inner->CostVar();
                }
            )
            .as_ref()
        }
    }

    /// Returns the cost of the transit arc between two nodes for a given vehicle.
    /// Input are variable indices of node. This returns 0 if vehicle < 0.
    pub fn get_arc_cost_for_vehicle(&self, from_index: i64, to_index: i64, vehicle: i64) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                from_index as "int64_t",
                to_index as "int64_t",
                vehicle as "int64_t"
            ] -> i64 as "int64_t"
                {
                    return inner->GetArcCostForVehicle(
                        from_index,
                        to_index,
                        vehicle
                    );
                }
            )
        }
    }

    /// Whether costs are homogeneous across all vehicles.
    pub fn costs_are_homogeneous_across_vehicles(&self) -> bool {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*"
            ] -> bool as "bool"
                {
                    return inner->CostsAreHomogeneousAcrossVehicles();
                }
            )
        }
    }

    /// Returns the cost of the segment between two nodes supposing all vehicle
    /// costs are the same (returns the cost for the first vehicle otherwise).
    pub fn get_homogeneous_cost(&self, from_index: i64, to_index: i64) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                from_index as "int64_t",
                to_index as "int64_t"
            ] -> i64 as "int64_t"
                {
                    return inner->GetHomogeneousCost(
                        from_index,
                        to_index
                    );
                }
            )
        }
    }

    /// Returns the cost of the arc in the context of the first solution strategy.
    /// This is typically a simplification of the actual cost; see the .cc.
    pub fn get_arc_cost_for_first_solution(&self, from_index: i64, to_index: i64) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                from_index as "int64_t",
                to_index as "int64_t"
            ] -> i64 as "int64_t"
                {
                    return inner->GetArcCostForFirstSolution(
                        from_index,
                        to_index
                    );
                }
            )
        }
    }

    /// Returns the cost of the segment between two nodes for a given cost
    /// class. Input are variable indices of nodes and the cost class.
    /// Unlike GetArcCostForVehicle(), if cost_class is kNoCost, then the
    /// returned cost won't necessarily be zero: only some of the components
    /// of the cost that depend on the cost class will be omited. See the code
    /// for details.
    pub fn get_arc_cost_for_class(
        &self,
        from_index: i64,
        to_index: i64,
        cost_class_index: i64, // CostClassIndex
    ) -> i64 {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::RoutingModel*",
                from_index as "int64_t",
                to_index as "int64_t",
                cost_class_index as "int64_t"
            ] -> i64 as "int64_t"
                {
                    return inner->GetArcCostForClass(
                        from_index,
                        to_index,
                        cost_class_index
                    );
                }
            )
        }
    }

    /// Returns the underlying constraint solver. Can be used to add extra
    /// constraints and/or modify search algorithms.
    pub fn solver(&self) -> &mut Solver {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "operations_research::RoutingModel*"
            ] -> &mut Solver as "operations_research::Solver*"
                {
                    return inner->solver();
                }
            )
        }
    }
}

cpp_class!(
    #[doc(hidden)]
    pub unsafe struct RoutingDimension as "operations_research::RoutingDimension"
);

impl RoutingDimension {
    /// Get the cumul variable for the given node (given as i64 var index).
    pub fn cumul_var(&self, index: i64) -> Option<&IntVar> {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingDimension*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return self->CumulVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Get the transit variable for the given node (given as i64 var index).
    pub fn transit_var(&self, index: i64) -> Option<&IntVar> {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingDimension*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return self->TransitVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Get the fixled transit variable for the given node (given as i64 var index).
    pub fn fixed_transit_var(&self, index: i64) -> Option<&IntVar> {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingDimension*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return self->FixedTransitVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Get the slack variable for the given node (given as i64 var index).
    pub fn slack_var(&self, index: i64) -> Option<&IntVar> {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingDimension*",
                index as "int64_t"
            ] -> *const IntVar as "operations_research::IntVar*"
                {
                    return self->SlackVar(index);
                }
            )
            .as_ref()
        }
    }

    /// Sets a cost proportional to the **global** dimension span, that is the
    /// difference between the largest value of route end cumul variables and
    /// the smallest value of route start cumul variables.
    /// In other words:
    /// global_span_cost =
    ///   coefficient * (Max(dimension end value) - Min(dimension start value)).
    pub fn set_global_span_cost_coefficient(&mut self, coefficient: i64) {
        unsafe {
            cpp!([
                self as "operations_research::RoutingDimension*",
                coefficient as "int64_t"
            ]
                {
                    return self->SetGlobalSpanCostCoefficient(coefficient);
                }
            )
        }
    }
}

/// Successful output of Routing.
pub struct Assignment<'manager, 'model> {
    inner: &'model super::Assignment,
    model: &'model RoutingModel<'manager>,
}

// Assignment inspection
impl<'manager, 'model> Assignment<'manager, 'model> {
    #[doc(hidden)]
    pub fn value(&self, var: &IntVar) -> i64 {
        let assignment = self.inner;

        unsafe {
            cpp!([
                assignment as "const operations_research::Assignment*",
                var as "const operations_research::IntVar*"
            ] -> i64 as "int64_t"
                {
                    return assignment->Value(var);
                }
            )
        }
    }

    /// Returns the variable index of the node directly after the node
    /// corresponding to `index` in `assignment`.
    pub fn next(&self, index: i64) -> i64 {
        let assignment = self.inner;
        let model = self.model.inner.as_ref();

        unsafe {
            cpp!([
                model as "const operations_research::RoutingModel*",
                assignment as "const operations_research::Assignment*",
                index as "int64_t"
            ] -> i64 as "int64_t"
                {
                    return model->Next(*assignment, index);
                }
            )
        }
    }

    /// Returns true if the route of `vehicle` is non empty in `assignment`.
    pub fn is_vehicle_used(&self, vehicle: c_int) -> bool {
        let assignment = self.inner;
        let model = self.model.inner.as_ref();

        unsafe {
            cpp!([
                model as "const operations_research::RoutingModel*",
                assignment as "const operations_research::Assignment*",
                vehicle as "int"
            ] -> bool as "bool"
                {
                    return model->IsVehicleUsed(*assignment, vehicle);
                }
            )
        }
    }
}
