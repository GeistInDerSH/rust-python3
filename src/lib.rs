use pyo3::prelude::*;

mod grep;
mod list;
mod math;

#[pyfunction]
/// int_sum generates the sum of a vector of integers
fn int_sum(vec: Vec<usize>) -> PyResult<usize> {
    Ok(vec.iter().sum())
}

#[pymodule]
fn rpython(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_sum, m)?)?;

    let list_submodule = list::register(_py)?;
    m.add_submodule(list_submodule)?;

    let grep_submodule = grep::register(_py)?;
    m.add_submodule(grep_submodule)?;

    let math_submodule = math::register(_py)?;
    m.add_submodule(math_submodule)?;

    Ok(())
}
