use pyo3::prelude::*;
use rand::Rng;

#[pyfunction]
/// int_sum generates the sum of a vector of integers
fn int_sum(vec: Vec<usize>) -> PyResult<usize> {
    Ok(vec.iter().sum())
}

/// Count the number of bits in an integer
fn bit_count(n: u64) -> i64 {
    let mut count = 0;
    let mut n = n;

    while n != 0 {
        count += 1;
        n >>= 1;
    }

    count
}

#[pyfunction]
/// Raise the base to the power and take the mod of the number
fn square_and_multiply(base: u64, pow: u64, m: u64) -> PyResult<u64> {
    let mut n = base;
    let bit_pos = bit_count(pow);

    for i in (0..bit_pos - 1).rev() {
        let bit = (pow >> i) & 1;
        n *= if bit == 1 { base * n } else { n };
        n %= m;
    }

    Ok(n)
}

#[pyfunction]
/// Generate a list of n random numbers with each number bounded by some number
fn rand_list(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    let mut rng = rand::thread_rng();
    let v = (0..len)
        .map(|_| {
            let n: usize = rng.gen();
            n % bound
        })
        .collect::<Vec<usize>>();
    Ok(v)
}

#[pymodule]
fn rpython(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(int_sum, m)?)?;
    m.add_function(wrap_pyfunction!(square_and_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(rand_list, m)?)?;
    Ok(())
}
