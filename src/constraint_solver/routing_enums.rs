// // IMPORT CXX LIBRARY
// cpp! {{
//     #include "ortools/constraint_solver/routing_enums.h"
// }}

// cpp_class!(
//     /// Parameters which have to be set when creating a RoutingModel.
//     pub unsafe struct FirstSolutionStrategy as "operations_research::FirstSolutionStrategy"
// );

// impl RoutingModelParameters {
//     /// Create a default parameters
//     pub fn new() -> Self {
//         unsafe {
//             cpp!([
//             ] -> RoutingModelParameters as "operations_research::RoutingModelParameters"
//                 {
//                     return operations_research::DefaultRoutingModelParameters();
//                 }
//             )
//         }
//     }
// }

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum FirstSolutionStrategy {
    /// See the homonymous value in LocalSearchMetaheuristic.
    Unset = 0,

    /// Lets the solver detect which strategy to use according to the model being
    /// solved.
    Automatic = 15,

    // --- Path addition heuristics ---
    /// Starting from a route "start" node, connect it to the node which produces
    /// the cheapest route segment, then extend the route by iterating on the
    /// last node added to the route.
    PathCheapestArc = 3,
    /// Same as PATH_CHEAPEST_ARC, but arcs are evaluated with a comparison-based
    /// selector which will favor the most constrained arc first. To assign a
    /// selector to the routing model, see
    /// RoutingModel::ArcIsMoreConstrainedThanArc() in routing.h for details.
    PathMostConstrainedArc = 4,
    /// Same as PATH_CHEAPEST_ARC, except that arc costs are evaluated using the
    /// function passed to RoutingModel::SetFirstSolutionEvaluator()
    /// (cf. routing.h).
    EvaluatorStrategy = 5,
    /// Savings algorithm (Clarke & Wright).
    /// Reference: Clarke, G. & Wright, J.W.:
    /// "Scheduling of Vehicles from a Central Depot to a Number of Delivery
    /// Points", Operations Research, Vol. 12, 1964, pp. 568-581
    Savings = 10,
    /// Sweep algorithm (Wren & Holliday).
    /// Reference: Anthony Wren & Alan Holliday: Computer Scheduling of Vehicles
    /// from One or More Depots to a Number of Delivery Points Operational
    /// Research Quarterly (1970-1977),
    /// Vol. 23, No. 3 (Sep., 1972), pp. 333-344
    Sweep = 11,
    /// Christofides algorithm (actually a variant of the Christofides algorithm
    /// using a maximal matching instead of a maximum matching, which does
    /// not guarantee the 3/2 factor of the approximation on a metric travelling
    /// salesman). Works on generic vehicle routing models by extending a route
    /// until no nodes can be inserted on it.
    /// Reference: Nicos Christofides, Worst-case analysis of a new heuristic for
    /// the travelling salesman problem, Report 388, Graduate School of
    /// Industrial Administration, CMU, 1976.
    Christofides = 13,

    // --- Path insertion heuristics ---
    /// Make all nodes inactive. Only finds a solution if nodes are optional (are
    /// element of a disjunction constraint with a finite penalty cost).
    AllUnperformed = 6,
    /// Iteratively build a solution by inserting the cheapest node at its
    /// cheapest position; the cost of insertion is based on the global cost
    /// function of the routing model. As of 2/2012, only works on models with
    /// optional nodes (with finite penalty costs).
    BestInsertion = 7,
    /// Iteratively build a solution by inserting the cheapest node at its
    /// cheapest position; the cost of insertion is based on the arc cost
    /// function. Is faster than BEST_INSERTION.
    ParallelCheapestInsertion = 8,
    /// Iteratively build a solution by constructing routes sequentially, for
    /// each route inserting the cheapest node at its cheapest position until the
    /// route is completed; the cost of insertion is based on the arc cost
    /// function. Is faster than `ParallelCheapestInsertion`.
    SequentialCheapestInsertion = 14,
    /// Iteratively build a solution by inserting each node at its cheapest
    /// position; the cost of insertion is based on the arc cost function.
    /// Differs from `ParallelCheapestInsertion` by the node selected for
    /// insertion; here nodes are considered in decreasing order of distance to
    /// the start/ends of the routes, i.e. farthest nodes are inserted first.
    /// Is faster than `SequentialCheapestInsertion`.
    LocalCheapestInsertion = 9,
    /// Same as `LOCAL_CHEAPEST_INSERTION` except that the cost of insertion is
    /// based on the routing model cost function instead of arc costs only.
    LocalCheapestCostInsertion = 16,

    // --- Variable-based heuristics ---
    /// Iteratively connect two nodes which produce the cheapest route segment.
    GlobalCheapestArc = 1,
    /// Select the first node with an unbound successor and connect it to the
    /// node which produces the cheapest route segment.
    LocalCheapestArc = 2,
    /// Select the first node with an unbound successor and connect it to the
    /// first available node.
    /// This is equivalent to the CHOOSE_FIRST_UNBOUND strategy combined with
    /// ASSIGN_MIN_VALUE (cf. constraint_solver.h).
    FirstUnboundMinValue = 12,
}
