use pyo3::prelude::*;

#[pyfunction]
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

#[pymodule]
fn rusty_snek(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;

    Ok(())
}