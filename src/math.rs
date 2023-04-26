use pyo3::prelude::*;

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

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
fn shortest_ascending_subsequence(seq: Vec<usize>) -> PyResult<Vec<usize>> {
    let mut sub_seq = Vec::new();

    for val in seq {
        if sub_seq.is_empty() || sub_seq[sub_seq.len() - 1] < val {
            sub_seq.push(val);
        }
    }

    Ok(sub_seq)
}

#[pyfunction]
fn fib(n: usize) -> PyResult<BigUint> {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = std::mem::replace(&mut f1, f2);
    }

    Ok(f0)
}

#[pyfunction]
pub fn fast_fib(n: usize) -> BigUint {
    match n {
        0 => Zero::zero(),
        1 => One::one(),
        _ => {
            let two = 2.to_biguint().unwrap();
            let k = n / 2;
            let f1 = fast_fib(k);
            let f2 = fast_fib(k - 1);

            match n % 4 {
                0 | 2 => &f1 * (&f1 + two * f2),
                1 => (&two * &f1 + &f2) * (&two * f1 - f2) + two,
                _ => (&two * &f1 + &f2) * (&two * f1 - f2) - two,
            }
        }
    }
}

pub fn register(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "math")?;
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    m.add_function(wrap_pyfunction!(fast_fib, m)?)?;
    m.add_function(wrap_pyfunction!(shortest_ascending_subsequence, m)?)?;
    m.add_function(wrap_pyfunction!(square_and_multiply, m)?)?;
    Ok(m)
}
