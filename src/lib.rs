#![allow(non_upper_case_globals)] // derived from: cpp crate
#![recursion_limit = "256"]

#[macro_use]
extern crate cpp;

pub mod constraint_solver;
pub mod graph;
pub mod sat;
pub mod utils;
