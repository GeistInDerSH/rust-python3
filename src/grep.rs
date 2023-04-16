use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use pyo3::{exceptions::PyUnicodeError, prelude::*};
use regex::Regex;

#[pyfunction]
pub fn grep(regex: String, file_name: String) -> PyResult<Vec<String>> {
    let file = File::open(file_name)?;
    let regex = match Regex::new(&regex) {
        Ok(r) => Ok(r),
        Err(err) => Err(PyUnicodeError::new_err(err.to_string())),
    }?;

    Ok(BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| regex.find(line).is_some())
        .collect())
}

#[pyfunction]
pub fn grep_files(regex: String, files: Vec<String>) -> PyResult<Vec<String>> {
    Ok(files
        .into_iter()
        .collect::<HashSet<String>>()
        .iter()
        .map(|file| grep(regex.clone(), file.to_string()))
        .filter(|r| r.is_ok())
        .flat_map(|r| r.unwrap())
        .collect())
}

#[pyfunction]
pub fn grep_files_mapped(
    regex: String,
    files: Vec<String>,
) -> PyResult<HashMap<String, Vec<String>>> {
    Ok(files
        .into_iter()
        .collect::<HashSet<String>>() // Get the unique values
        .iter()
        .map(|file| (file.clone(), grep(regex.clone(), file.to_string())))
        .filter(|(_, r)| r.is_ok())
        .map(|(v, r)| (v, r.unwrap()))
        .collect::<HashMap<String, Vec<String>>>())
}

pub fn register(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "grep")?;
    m.add_function(wrap_pyfunction!(grep_files, m)?)?;
    m.add_function(wrap_pyfunction!(grep_files_mapped, m)?)?;
    m.add_function(wrap_pyfunction!(grep, m)?)?;
    Ok(m)
}
