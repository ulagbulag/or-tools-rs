use std::marker::PhantomData;

// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/graph/max_flow.h"
}}

cpp_class!(
    #[doc(hidden)]
    unsafe struct MaxFlowInner as "operations_research::MaxFlow"
);

/// Default instance MaxFlow that uses StarGraph. Note that we cannot just use a
/// typedef because of dependent code expecting MaxFlow to be a real class.
pub struct MaxFlow<'graph> {
    /// Lifetime limiter for graph
    _graph: PhantomData<&'graph ()>,
    /// Original solver
    inner: Box<MaxFlowInner>,
}

impl<'graph> MaxFlow<'graph> {
    /// Creates a new `MaxFlow` struct.
    pub fn new(
        graph: &'graph super::StarGraph,
        source: super::NodeIndex,
        target: super::NodeIndex,
    ) -> Self {
        Self {
            _graph: PhantomData,
            inner: unsafe {
                cpp!([
                    graph as "const operations_research::StarGraph*",
                    source as "operations_research::NodeIndex",
                    target as "operations_research::NodeIndex"
                ] -> Box<MaxFlowInner> as "operations_research::MaxFlow*"
                    {
                        return new operations_research::MaxFlow(graph, source, target);
                    }
                )
            },
        }
    }

    /// Change the capacity of an arc.
    pub fn set_arc_capacity(&mut self, arc: super::ArcIndex, capacity: super::FlowQuantity) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MaxFlow*",
                arc as "operations_research::ArcIndex",
                capacity as "operations_research::FlowQuantity"
            ]
                {
                    return inner->SetArcCapacity(arc, capacity);
                }
            )
        }
    }

    // Sets the flow for arc.
    pub fn set_arc_flow(&mut self, arc: super::ArcIndex, flow: super::FlowQuantity) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MaxFlow*",
                arc as "operations_research::ArcIndex",
                flow as "operations_research::FlowQuantity"
            ]
                {
                    return inner->SetArcFlow(arc, flow);
                }
            )
        }
    }

    /// Returns output if a maximum flow was solved.
    pub fn solve(&mut self) -> Option<MaxFlowOutput<'graph, '_>> {
        let inner = self.inner.as_mut();

        let solved = unsafe {
            cpp!([
                inner as "operations_research::MaxFlow*"
            ] -> bool as "bool"
                {
                    return inner->Solve();
                }
            )
        };

        if solved {
            Some(MaxFlowOutput { solver: self })
        } else {
            None
        }
    }
}

/// Successful output of MaxFlow.
pub struct MaxFlowOutput<'graph, 'solver> {
    /// Callee solver
    solver: &'solver MaxFlow<'graph>,
}

impl<'graph, 'solver> MaxFlowOutput<'graph, 'solver> {
    /// Returns the total flow found by the algorithm.
    pub fn get_optimal_flow(&self) -> super::FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MaxFlow*"
            ] -> super::FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->GetOptimalFlow();
                }
            )
        }
    }

    /// Returns the flow on arc using the equations given in the comment on
    /// residual_arc_capacity_.
    pub fn flow(&self, arc: super::ArcIndex) -> super::FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MaxFlow*",
                arc as "operations_research::ArcIndex"
            ] -> super::FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->Flow(arc);
                }
            )
        }
    }

    /// Returns the capacity of arc using the equations given in the comment on
    /// residual_arc_capacity_.
    pub fn capacity(&self, arc: super::ArcIndex) -> super::FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MaxFlow*",
                arc as "operations_research::ArcIndex"
            ] -> super::FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->Capacity(arc);
                }
            )
        }
    }

    /// Returns the status. `NotSolved` is returned if
    /// the problem has been modified in such a way that
    /// the previous solution becomes invalid.
    pub fn status(&self) -> MaxFlowStatus {
        let inner = self.solver.inner.as_ref();

        let status = unsafe {
            cpp!([
                inner as "const operations_research::MaxFlow*"
            ] -> u8 as "uint8_t"
                {
                    switch (inner->status()) {
                        case operations_research::MaxFlowStatusClass::Status::NOT_SOLVED:
                            return 0;
                        case operations_research::MaxFlowStatusClass::Status::OPTIMAL:
                            return 1;
                        case operations_research::MaxFlowStatusClass::Status::INT_OVERFLOW:
                            return 2;
                        case operations_research::MaxFlowStatusClass::Status::BAD_INPUT:
                            return 3;
                        case operations_research::MaxFlowStatusClass::Status::BAD_RESULT:
                            return 4;
                        default:
                            return 4;
                    }
                }
            )
        };

        match status {
            0 => MaxFlowStatus::NotSolved,
            1 => MaxFlowStatus::Optimal,
            2 => MaxFlowStatus::IntOverflow,
            3 => MaxFlowStatus::BadInput,
            4 | 5.. => MaxFlowStatus::BadResult,
        }
    }
}

/// Solves the problem (finds the maximum flow from the given source to the
/// given sink), and returns the problem status.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MaxFlowStatus {
    /// The problem was not solved, or its data were edited.
    NotSolved,
    /// solve() was called and found an optimal solution.
    Optimal,
    /// There is a feasible flow > max possible flow.
    IntOverflow,
    /// The input is inconsistent.
    BadInput,
    /// There was an error.
    BadResult,
}
