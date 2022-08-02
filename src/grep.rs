use std::fs::File;
use std::io::{prelude::*, BufReader};

use pyo3::prelude::*;
use regex::Regex;

#[pyfunction]
pub fn grep(regex: String, file_name: String) -> PyResult<Vec<String>> {
    let file = File::open(file_name)?;
    let regex = Regex::new(&regex).unwrap();

    Ok(BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| regex.find(line).is_some())
        .collect())
}

#[pyfunction]
pub fn grep_files(regex: String, files: Vec<String>) -> PyResult<Vec<String>> {
    let mut v = Vec::new();

    for file in files {
        let mut temp = grep(regex.clone(), file).unwrap();
        v.append(&mut temp);
    }

    Ok(v)
}

pub fn register(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "grep")?;
    m.add_function(wrap_pyfunction!(grep_files, m)?)?;
    m.add_function(wrap_pyfunction!(grep, m)?)?;
    Ok(m)
}
