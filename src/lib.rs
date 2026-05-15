//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for CocoIndex,
//! exposing Python bindings via PyO3.

use pyo3::prelude::*;

pub mod indexing;
pub mod pipeline;
pub mod storage;
pub mod transforms;
pub mod utils;

/// Python module entry point.
///
/// Registers all Python-accessible classes and functions from the
/// Rust implementation.
#[pymodule]
fn _cocoindex_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register core indexing types
    m.add_class::<indexing::Index>()?;
    m.add_class::<indexing::IndexConfig>()?;

    // Register pipeline components
    m.add_class::<pipeline::Pipeline>()?;
    m.add_class::<pipeline::PipelineStep>()?;

    // Register storage backends
    m.add_class::<storage::StorageBackend>()?;

    // Register utility functions
    m.add_function(wrap_pyfunction!(utils::version, m)?)?;

    Ok(())
}
