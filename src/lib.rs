#[macro_use] extern crate impl_ops;
mod coordinate;
mod problem;
mod solution;
mod marked;
mod grid;
mod client;
pub mod ffi;

pub use client::MapfmClientError;
pub use client::MapfBenchmarker;
pub use client::BenchmarkDescriptor;
pub use client::ProgressiveDescriptor;

pub use grid::Grid;
pub use marked::MarkedCoordinate;
pub use coordinate::Coordinate;
pub use solution::Solution;
pub use problem::Problem;
