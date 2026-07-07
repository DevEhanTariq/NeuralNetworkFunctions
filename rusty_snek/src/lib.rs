use pyo3::prelude::*;

#[pyfunction]
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

#[pyfunction]
fn neuron_calculator(a: Vec<f32>, b: Vec<f32>) ->  f32 {
    let mut z: f32 = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        z += x * y;
    }
    return z;
}

#[pyfunction]
fn idk(n: u128) {
    for i in 0..n {
        let a: f32 = 0.003*0.002; // i^6
    }
}

#[pymodule]
fn rusty_snek(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(neuron_calculator, m)?)?;

    m.add_function(wrap_pyfunction!(idk, m)?)?;

    Ok(())
}