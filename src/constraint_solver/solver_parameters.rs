// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/constraint_solver/solver_parameters.pb.h"
}}

cpp_class!(
    /// Solver parameters.
    pub unsafe struct ConstraintSolverParameters as "operations_research::ConstraintSolverParameters"
);
