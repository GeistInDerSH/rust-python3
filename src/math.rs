use pyo3::prelude::*;

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
pub fn square_and_multiply(base: u64, pow: u64, m: u64) -> PyResult<u64> {
    let mut n = base;
    let bit_pos = bit_count(pow);

    for i in (0..bit_pos - 1).rev() {
        let bit = (pow >> i) & 1;
        n *= if bit == 1 { base * n } else { n };
        n %= m;
    }

    Ok(n)
}
