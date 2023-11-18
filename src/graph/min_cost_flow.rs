use std::marker::PhantomData;

use super::ebert_graph::{ArcIndex, CostValue, FlowQuantity, NodeIndex, StarGraph};

// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/graph/min_cost_flow.h"
}}

cpp_class!(
    #[doc(hidden)]
    unsafe struct MinCostFlowInner as "operations_research::MinCostFlow"
);

/// Default MinCostFlow instance that uses StarGraph.
/// New clients should use SimpleMinCostFlow if they can.
pub struct MinCostFlow<'graph> {
    /// Lifetime limiter for graph
    _graph: PhantomData<&'graph ()>,
    /// Original solver
    inner: Box<MinCostFlowInner>,
}

impl<'graph> MinCostFlow<'graph> {
    /// Creates a new `MinCostFlow` struct.
    pub fn new(graph: &'graph StarGraph) -> Self {
        Self {
            _graph: PhantomData,
            inner: unsafe {
                cpp!([
                    graph as "const operations_research::StarGraph*"
                ] -> Box<MinCostFlowInner> as "operations_research::MinCostFlow*"
                    {
                        return new operations_research::MinCostFlow(graph);
                    }
                )
            },
        }
    }

    /// Returns the graph associated to the current object.
    pub fn graph(&self) -> &StarGraph {
        let inner = self.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*"
            ] -> &StarGraph as "const operations_research::StarGraph*"
                {
                    return inner->graph();
                }
            )
        }
    }

    /// Sets the supply corresponding to node. A demand is modeled as a negative
    /// supply.
    pub fn set_node_supply(&mut self, node: NodeIndex, supply: FlowQuantity) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*",
                node as "operations_research::NodeIndex",
                supply as "operations_research::FlowQuantity"
            ]
                {
                    return inner->SetNodeSupply(node, supply);
                }
            )
        }
    }

    /// Sets the unit cost for the given arc.
    pub fn set_arc_unit_cost(&mut self, arc: ArcIndex, unit_cost: CostValue) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex",
                unit_cost as "operations_research::CostValue"
            ]
                {
                    return inner->SetArcUnitCost(arc, unit_cost);
                }
            )
        }
    }

    /// Sets the capacity for the given arc.
    pub fn set_arc_capacity(&mut self, arc: ArcIndex, new_capacity: FlowQuantity) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex",
                new_capacity as "operations_research::FlowQuantity"
            ]
                {
                    return inner->SetArcCapacity(arc, new_capacity);
                }
            )
        }
    }

    /// Sets the flow for the given arc. Note that new_flow must be smaller than
    /// the capacity of the arc.
    pub fn set_arc_flow(&mut self, arc: ArcIndex, new_flow: FlowQuantity) {
        let inner = self.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex",
                new_flow as "operations_research::FlowQuantity"
            ]
                {
                    return inner->SetArcFlow(arc, new_flow);
                }
            )
        }
    }

    /// Return output if a min-cost flow could be found.
    pub fn solve(&mut self) -> Option<MinCostFlowOutput<'graph, '_>> {
        let inner = self.inner.as_mut();

        let solved = unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*"
            ] -> bool as "bool"
                {
                    return inner->Solve();
                }
            )
        };

        if solved {
            Some(MinCostFlowOutput { solver: self })
        } else {
            None
        }
    }
}

/// Successful output of MinCostFlow.
pub struct MinCostFlowOutput<'graph, 'solver> {
    /// Callee solver
    solver: &'solver mut MinCostFlow<'graph>,
}

impl<'graph, 'solver> MinCostFlowOutput<'graph, 'solver> {
    /// Returns the cost of the minimum-cost flow found by the algorithm. This
    /// works in O(num_arcs). This will only work if the last solve() call was
    /// successful and returned true, otherwise it will return 0. Note that the
    /// computation might overflow, in which case we will cap the cost at
    /// CostValue::MAX.
    pub fn get_optimal_cost(&mut self) -> CostValue {
        let inner = self.solver.inner.as_mut();

        unsafe {
            cpp!([
                inner as "operations_research::MinCostFlow*"
            ] -> CostValue as "operations_research::CostValue"
                {
                    return inner->GetOptimalCost();
                }
            )
        }
    }

    /// Returns the flow on the given arc using the equations given in the
    /// comment on residual_arc_capacity_.
    pub fn flow(&self, arc: ArcIndex) -> FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex"
            ] -> FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->Flow(arc);
                }
            )
        }
    }

    /// Returns the capacity of the given arc.
    pub fn capacity(&self, arc: ArcIndex) -> FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex"
            ] -> FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->Capacity(arc);
                }
            )
        }
    }

    /// Returns the unscaled cost for the given arc.
    pub fn unit_cost(&self, arc: ArcIndex) -> CostValue {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                arc as "operations_research::ArcIndex"
            ] -> CostValue as "operations_research::CostValue"
                {
                    return inner->UnitCost(arc);
                }
            )
        }
    }

    /// Returns the supply at a given node. Demands are modelled as negative
    /// supplies.
    pub fn supply(&self, node: NodeIndex) -> FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                node as "operations_research::NodeIndex"
            ] -> FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->Supply(node);
                }
            )
        }
    }

    /// Returns the initial supply at a given node.
    pub fn initial_supply(&self, node: NodeIndex) -> FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                node as "operations_research::NodeIndex"
            ] -> FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->InitialSupply(node);
                }
            )
        }
    }

    /// Returns the largest supply (if > 0) or largest demand in absolute value
    /// (if < 0) admissible at node. If the problem is not feasible, some of these
    /// values will be smaller (in absolute value) than the initial supplies
    /// and demand given as input.
    pub fn feasible_supply(&self, node: NodeIndex) -> FlowQuantity {
        let inner = self.solver.inner.as_ref();

        unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*",
                node as "operations_research::NodeIndex"
            ] -> FlowQuantity as "operations_research::FlowQuantity"
                {
                    return inner->FeasibleSupply(node);
                }
            )
        }
    }

    /// Returns the status. `NotSolved` is returned if
    /// the problem has been modified in such a way that
    /// the previous solution becomes invalid.
    pub fn status(&self) -> MinCostFlowStatus {
        let inner = self.solver.inner.as_ref();

        let status = unsafe {
            cpp!([
                inner as "const operations_research::MinCostFlow*"
            ] -> u8 as "uint8_t"
                {
                    switch (inner->status()) {
                        case operations_research::MinCostFlowBase::Status::NOT_SOLVED:
                            return 0;
                        case operations_research::MinCostFlowBase::Status::OPTIMAL:
                            return 1;
                        case operations_research::MinCostFlowBase::Status::FEASIBLE:
                            return 2;
                        case operations_research::MinCostFlowBase::Status::INFEASIBLE:
                            return 3;
                        case operations_research::MinCostFlowBase::Status::UNBALANCED:
                            return 4;
                        case operations_research::MinCostFlowBase::Status::BAD_COST_RANGE:
                            return 5;
                        case operations_research::MinCostFlowBase::Status::BAD_RESULT:
                            return 6;
                        default:
                            return 6;
                    }
                }
            )
        };

        match status {
            0 => MinCostFlowStatus::NotSolved,
            1 => MinCostFlowStatus::Optimal,
            2 => MinCostFlowStatus::Feasible,
            3 => MinCostFlowStatus::Infeasible,
            4 => MinCostFlowStatus::Unbalanced,
            5 => MinCostFlowStatus::BadCostRange,
            6 => MinCostFlowStatus::BadResult,
            7.. => unreachable!(),
        }
    }
}

/// Solves the problem (finds the maximum flow from the given source to the
/// given sink), and returns the problem status.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MinCostFlowStatus {
    /// The problem was not solved, or its data were edited.
    NotSolved,
    /// solve() was called and found an optimal solution.
    Optimal,
    #[doc(hidden)]
    Feasible,
    #[doc(hidden)]
    Infeasible,
    #[doc(hidden)]
    Unbalanced,
    /// This is returned when an integer overflow occurred during the algorithm
    /// execution.
    ///
    /// Some details on how to deal with this:
    /// - The sum of all incoming/outgoing capacity at each node should not
    ///   overflow. TODO(user): this is not always properly checked and probably
    ///   deserve a different return status.
    /// - Since we scale cost, each arc unit cost times (num_nodes + 1) should
    ///   not overflow. We detect that at the beginning of the Solve().
    /// - This is however not sufficient as the node potential depends on the
    ///   minimum cost of sending 1 unit of flow through the residual graph. If
    ///   the maximum arc unit cost is smaller than kint64max / (2 * n ^ 2) then
    ///   one shouldn't run into any overflow. But in pratice this bound is quite
    ///   loose. So it is possible to try with higher cost, and the algorithm
    ///   will detect if an overflow actually happen and return BAD_COST_RANGE,
    ///   so we can retry with smaller cost.
    ///
    /// And two remarks:
    /// - Note that the complexity of the algorithm depends on the maximum cost,
    ///   so it is usually a good idea to use unit cost that are as small as
    ///   possible.
    /// - Even if there is no overflow, note that the total cost can easily not
    ///   fit on an int64_t since it is the product of the unit cost times the
    ///   actual amount of flow sent. This is easy to detect since the final
    ///   optimal cost will be set to kint64max. It is also relatively easy to
    ///   deal with since we will still have the proper flow on each arc. It is
    ///   thus possible to recompute the total cost in double or using
    ///   absl::int128 if the need arise.
    BadCostRange,
    /// There was an error.
    BadResult,
}
