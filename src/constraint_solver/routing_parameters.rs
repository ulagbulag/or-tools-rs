use std::time::Duration;

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

    /// Set first solution strategies, used as starting point of local search.
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

    // Set limit to the time spent in the search.
    pub fn set_time_limit(&mut self, duration: Duration) {
        let seconds = duration.as_secs();
        let nanos = duration.subsec_nanos();

        unsafe {
            cpp!([
                self as "operations_research::RoutingSearchParameters*",
                seconds as "uint64_t",
                nanos as "uint32_t"
            ]
                {
                    auto time_limit = self->mutable_time_limit();
                    time_limit->set_seconds(seconds);
                    return time_limit->set_nanos(nanos);
                }
            )
        }
    }
}
