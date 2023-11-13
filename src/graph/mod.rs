pub mod max_flow;

// IMPORT CXX LIBRARY
cpp! {{
    #include "ortools/base/init_google.h"
}}

cpp_class!(
    /// Standard instantiation of ForwardEbertGraph (named 'ForwardStarGraph') of
    /// EbertGraph (named 'StarGraph'); and relevant type shortcuts. Unless their use
    /// cases prevent them from doing so, users are encouraged to use StarGraph or
    /// ForwardStarGraph according to whether or not they require reverse arcs to be
    /// represented explicitly. Along with either graph representation, the other
    /// type shortcuts here will often come in handy.
    pub unsafe struct StarGraph as "operations_research::StarGraph"
);

impl StarGraph {
    /// Creates a new `StarGraph` struct.
    pub fn new(max_num_nodes: NodeIndex, max_num_arcs: ArcIndex) -> Self {
        unsafe {
            cpp!([
                max_num_nodes as "operations_research::NodeIndex",
                max_num_arcs as "operations_research::ArcIndex"
            ] -> StarGraph as "operations_research::StarGraph"
                {
                    return operations_research::StarGraph(max_num_nodes, max_num_arcs);
                }
            )
        }
    }

    /// Adds an arc to the graph and returns its index.
    /// Returns kNilArc if the arc could not be added.
    /// Note that for a given pair (tail, head) AddArc does not overwrite an
    /// already-existing arc between tail and head: Another arc is created
    /// instead. This makes it possible to handle multi-graphs.
    pub fn add_arc(&mut self, tail: NodeIndex, head: NodeIndex) -> ArcIndex {
        unsafe {
            cpp!([
                self as "operations_research::StarGraph*",
                tail as "operations_research::NodeIndex",
                head as "operations_research::NodeIndex"
            ] -> ArcIndex as "operations_research::ArcIndex"
                {
                    return self->AddArc(tail, head);
                }
            )
        }
    }

    /// Returns the head node of given arc in the graph.
    pub fn head(&self, arc: ArcIndex) -> NodeIndex {
        unsafe {
            cpp!([
                self as "const operations_research::StarGraph*",
                arc as "operations_research::ArcIndex"
            ] -> NodeIndex as "operations_research::NodeIndex"
                {
                    return self->Head(arc);
                }
            )
        }
    }

    /// Returns the tail node of given arc in the graph.
    pub fn tail(&self, arc: ArcIndex) -> NodeIndex {
        unsafe {
            cpp!([
                self as "const operations_research::StarGraph*",
                arc as "operations_research::ArcIndex"
            ] -> NodeIndex as "operations_research::NodeIndex"
                {
                    return self->Tail(arc);
                }
            )
        }
    }

    /// Returns the number of nodes in the graph.
    pub fn num_nodes(&self) -> NodeIndex {
        unsafe {
            cpp!([
                self as "const operations_research::StarGraph*"
            ] -> NodeIndex as "operations_research::NodeIndex"
                {
                    return self->num_nodes();
                }
            )
        }
    }

    /// Returns the number of nodes in the graph.
    pub fn num_arcs(&self) -> ArcIndex {
        unsafe {
            cpp!([
                self as "const operations_research::StarGraph*"
            ] -> ArcIndex as "operations_research::ArcIndex"
                {
                    return self->num_arcs();
                }
            )
        }
    }
}

#[doc(hidden)]
pub type NodeIndex = u32;
#[doc(hidden)]
pub type ArcIndex = u32;
#[doc(hidden)]
pub type FlowQuantity = u64;
#[doc(hidden)]
pub type CostValue = i64;
