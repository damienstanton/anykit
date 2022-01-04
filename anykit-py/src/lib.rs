use anykit::math;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: i32, b: i32) -> PyResult<String> {
    Ok((math::add(a, b)).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn anykit_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}