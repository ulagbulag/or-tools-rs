use std::{
    ffi::c_int,
    ops::{Index, IndexMut},
};

// IMPORT CXX LIBRARY
cpp! {{
    #include <vector>

    #include "ortools/constraint_solver/routing_index_manager.h"
}}

cpp_class!(
    /// Manager for any NodeIndex <-> variable index conversion. The routing solver
    /// uses variable indices internally and through its API. These variable indices
    /// are tricky to manage directly because one Node can correspond to a multitude
    /// of variables, depending on the number of times they appear in the model, and
    /// if they're used as start and/or end points. This class aims to simplify
    /// variable index usage, allowing users to use NodeIndex instead.
    ///
    /// Usage:
    /// ```ignore
    /// let starts_ends = &[];  /// These are NodeIndex.
    /// let manager = RoutingIndexManager::new(10, 4, starts_ends);  // 10 nodes, 4 vehicles.
    /// let model = RoutingModel::new(manager);
    /// ```
    ///
    /// Then, use `manager.node_to_index(node)` whenever model requires a variable
    /// index.
    ///
    /// Note: the mapping between node indices and variables indices is subject to
    /// change so no assumption should be made on it. The only guarantee is that
    /// indices range between 0 and n-1, where n = number of vehicles * 2 (for start
    /// and end nodes) + number of non-start or end nodes.
    ///
    pub unsafe struct RoutingIndexManager as "operations_research::RoutingIndexManager"
);

impl RoutingIndexManager {
    /// Creates a NodeIndex to variable index mapping for a problem containing
    /// `num_nodes`, `num_vehicles` and the given starts and ends for each
    /// vehicle. If used, any start/end arrays have to have exactly `num_vehicles`
    /// elements.
    pub fn new(
        num_nodes: c_int,
        num_vehicles: c_int,
        plan: RoutingIndexManagerVehiclePlan,
    ) -> Self {
        match plan {
            RoutingIndexManagerVehiclePlan::Depot(depot) => unsafe {
                cpp!([
                    num_nodes as "int",
                    num_vehicles as "int",
                    depot as "operations_research::RoutingNodeIndex"
                ] -> RoutingIndexManager as "operations_research::RoutingIndexManager"
                    {
                        return operations_research::RoutingIndexManager(
                            num_nodes,
                            num_vehicles,
                            depot
                        );
                    }
                )
            },
            RoutingIndexManagerVehiclePlan::Map { starts, ends } => unsafe {
                cpp!([
                    num_nodes as "int",
                    num_vehicles as "int",
                    starts as "std::vector<operations_research::RoutingNodeIndex>",
                    ends as "std::vector<operations_research::RoutingNodeIndex>"
                ] -> RoutingIndexManager as "operations_research::RoutingIndexManager"
                    {
                        return operations_research::RoutingIndexManager(
                            num_nodes,
                            num_vehicles,
                            starts,
                            ends
                        );
                    }
                )
            },
            RoutingIndexManagerVehiclePlan::Pair(starts_ends) => unsafe {
                cpp!([
                    num_nodes as "int",
                    num_vehicles as "int",
                    starts_ends as "std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>"
                ] -> RoutingIndexManager as "operations_research::RoutingIndexManager"
                    {
                        return operations_research::RoutingIndexManager(
                            num_nodes,
                            num_vehicles,
                            starts_ends
                        );
                    }
                )
            },
        }
    }

    /// Returns the number of nodes in the manager.
    pub fn num_nodes(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*"
            ] -> c_int as "int"
                {
                    return self->num_nodes();
                }
            )
        }
    }

    /// Returns the number of vehicles in the manager.
    pub fn num_vehicles(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*"
            ] -> c_int as "int"
                {
                    return self->num_vehicles();
                }
            )
        }
    }

    /// Returns the number of indices mapped to nodes.
    pub fn num_indices(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*"
            ] -> c_int as "int"
                {
                    return self->num_indices();
                }
            )
        }
    }

    /// Returns start index of the given vehicle.
    pub fn get_start_index(&self, vehicle: c_int) -> i64 {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*",
                vehicle as "int"
            ] -> i64 as "int64_t"
                {
                    return self->GetStartIndex(vehicle);
                }
            )
        }
    }

    /// Returns end index of the given vehicle.
    pub fn get_end_index(&self, vehicle: c_int) -> i64 {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*",
                vehicle as "int"
            ] -> i64 as "int64_t"
                {
                    return self->GetEndIndex(vehicle);
                }
            )
        }
    }

    /// Returns the index of a node. A node can correspond to multiple indices if
    /// it's a start or end node. As of 03/2020, kUnassigned will be returned for
    /// all end nodes. If a node appears more than once as a start node, the index
    /// of the first node in the list of start nodes is returned.
    pub fn node_to_index(&self, node: &RoutingNodeIndex) -> i64 {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*",
                node as "const operations_research::RoutingNodeIndex*"
            ] -> i64 as "int64_t"
                {
                    return self->NodeToIndex(*node);
                }
            )
        }
    }

    /// Returns the node corresponding to an index. A node may appear more than
    /// once if it is used as the start or the end node of multiple vehicles.
    pub fn index_to_node(&self, index: i64) -> RoutingNodeIndex {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*",
                index as "int64_t"
            ] -> RoutingNodeIndex as "operations_research::RoutingNodeIndex"
                {
                    return self->IndexToNode(index);
                }
            )
        }
    }

    /// TODO(user) Add unit tests for NodesToIndices and IndicesToNodes.
    ///
    /// TODO(user): Remove when removal of NodeIndex from RoutingModel is
    /// complete.
    pub fn num_unique_depots(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingIndexManager*"
            ] -> c_int as "int"
                {
                    return self->num_unique_depots();
                }
            )
        }
    }
}

pub enum RoutingIndexManagerVehiclePlan {
    Depot(RoutingNodeIndex),
    Map {
        starts: RoutingNodeIndexVector,
        ends: RoutingNodeIndexVector,
    },
    Pair(RoutingNodeIndexPairVector),
}

cpp_class!(
    #[doc(hidden)]
    pub unsafe struct RoutingNodeIndexVector as "std::vector<operations_research::RoutingNodeIndex>"
);

impl FromIterator<RoutingNodeIndex> for RoutingNodeIndexVector {
    fn from_iter<T: IntoIterator<Item = RoutingNodeIndex>>(iter: T) -> Self {
        let mut vector = Self::default();
        iter.into_iter().for_each(|value| vector.push(value));
        vector
    }
}

impl Index<usize> for RoutingNodeIndexVector {
    type Output = (RoutingNodeIndex, RoutingNodeIndex);

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            cpp!([
                self as "const std::vector<operations_research::RoutingNodeIndex>*",
                index as "size_t"
            ] -> &(RoutingNodeIndex, RoutingNodeIndex) as "const operations_research::RoutingNodeIndex*"
                {
                    return self->data() + index;
                }
            )
        }
    }
}

impl IndexMut<usize> for RoutingNodeIndexVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            cpp!([
                self as "std::vector<operations_research::RoutingNodeIndex>*",
                index as "size_t"
            ] -> &mut (RoutingNodeIndex, RoutingNodeIndex) as "operations_research::RoutingNodeIndex*"
                {
                    return self->data() + index;
                }
            )
        }
    }
}

impl RoutingNodeIndexVector {
    pub fn len(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const std::vector<operations_research::RoutingNodeIndex>*"
            ] -> c_int as "int"
                {
                    return (int) self->size();
                }
            )
        }
    }

    pub fn push(&mut self, value: RoutingNodeIndex) {
        unsafe {
            cpp!([
                self as "std::vector<operations_research::RoutingNodeIndex>*",
                value as "operations_research::RoutingNodeIndex"
            ]
                {
                    return self->push_back(value);
                }
            )
        }
    }
}

cpp_class!(
    #[doc(hidden)]
    pub unsafe struct RoutingNodeIndexPairVector as "std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>"
);

impl FromIterator<(RoutingNodeIndex, RoutingNodeIndex)> for RoutingNodeIndexPairVector {
    fn from_iter<T: IntoIterator<Item = (RoutingNodeIndex, RoutingNodeIndex)>>(iter: T) -> Self {
        let mut vector = Self::default();
        iter.into_iter()
            .for_each(|(start, end)| vector.push(start, end));
        vector
    }
}

impl Index<usize> for RoutingNodeIndexPairVector {
    type Output = (RoutingNodeIndex, RoutingNodeIndex);

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            cpp!([
                self as "const std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>*",
                index as "size_t"
            ] -> &(RoutingNodeIndex, RoutingNodeIndex) as "const std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>*"
                {
                    return self->data() + index;
                }
            )
        }
    }
}

impl IndexMut<usize> for RoutingNodeIndexPairVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            cpp!([
                self as "std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>*",
                index as "size_t"
            ] -> &mut (RoutingNodeIndex, RoutingNodeIndex) as "std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>*"
                {
                    return self->data() + index;
                }
            )
        }
    }
}

impl RoutingNodeIndexPairVector {
    pub fn len(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>*"
            ] -> c_int as "int"
                {
                    return (int) self->size();
                }
            )
        }
    }

    pub fn push(&mut self, start: RoutingNodeIndex, end: RoutingNodeIndex) {
        let value = (start, end);

        unsafe {
            cpp!([
                self as "std::vector<std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>>*",
                value as "std::pair<operations_research::RoutingNodeIndex, operations_research::RoutingNodeIndex>"
            ]
                {
                    return self->push_back(value);
                }
            )
        }
    }
}

cpp_class!(
    #[doc(hidden)]
    pub unsafe struct RoutingNodeIndex as "operations_research::RoutingNodeIndex"
);

impl RoutingNodeIndex {
    pub fn new(value: c_int) -> Self {
        unsafe {
            cpp!([
                value as "int"
            ] -> RoutingNodeIndex as "operations_research::RoutingNodeIndex"
                {
                    return operations_research::RoutingNodeIndex(value);
                }
            )
        }
    }

    pub fn value(&self) -> c_int {
        unsafe {
            cpp!([
                self as "const operations_research::RoutingNodeIndex*"
            ] -> c_int as "int"
                {
                    return self->value();
                }
            )
        }
    }
}

#[doc(hidden)]
pub type RoutingCostClassIndex = c_int;
#[doc(hidden)]
pub type RoutingDimensionIndex = c_int;
#[doc(hidden)]
pub type RoutingDisjunctionIndex = c_int;
#[doc(hidden)]
pub type RoutingVehicleClassIndex = c_int;
