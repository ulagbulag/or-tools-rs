use super::routing_enums::FirstSolutionStrategy;

// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/constraint_solver/routing_parameters.h"
}}

cpp_class!(
    /// Parameters which have to be set when creating a RoutingModel.
    pub unsafe struct RoutingModelParameters as "operations_research::RoutingModelParameters"
);

impl RoutingModelParameters {
    /// Create a default parameters
    pub fn new() -> Self {
        unsafe {
            cpp!([
            ] -> RoutingModelParameters as "operations_research::RoutingModelParameters"
                {
                    return operations_research::DefaultRoutingModelParameters();
                }
            )
        }
    }
}

cpp_class!(
    /// Parameters defining the search used to solve vehicle routing problems.
    ///
    /// If a parameter is unset (or, equivalently, set to its default value),
    /// then the routing library will pick its preferred value for that parameter
    /// automatically: this should be the case for most parameters.
    /// To see those "default" parameters, call GetDefaultRoutingSearchParameters().
    /// Next ID: 56
    pub unsafe struct RoutingSearchParameters as "operations_research::RoutingSearchParameters"
);

impl RoutingSearchParameters {
    /// Create a default parameters
    pub fn new() -> Self {
        unsafe {
            cpp!([
            ] -> RoutingSearchParameters as "operations_research::RoutingSearchParameters"
                {
                    return operations_research::DefaultRoutingSearchParameters();
                }
            )
        }
    }

    /// Set a parameter: `first_solution_strategy`
    pub fn set_first_solution_strategy(&mut self, value: FirstSolutionStrategy) {
        unsafe {
            cpp!([
                self as "operations_research::RoutingSearchParameters*",
                value as "operations_research::FirstSolutionStrategy_Value"
            ]
                {
                    return self->set_first_solution_strategy(value);
                }
            )
        }
    }
}
