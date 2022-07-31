use pyo3::prelude::*;
use rand::Rng;

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[pyfunction]
/// Generate a list of n random numbers with each number bounded by some number
pub fn rand_list(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    if len < 1200 {
        Ok(serial_alloc(len, bound))
    } else {
        Ok(parallel_alloc(len, bound))
    }
}

#[pyfunction]
pub fn serial_rand_list(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    Ok(serial_alloc(len, bound))
}

fn serial_alloc(len: usize, bound: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    let mut v = vec![0; len];
    for item in v.iter_mut().take(len) {
        *item = rng.gen_range(0..bound);
    }

    v
}

#[pyfunction]
pub fn parallel_rand_list(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    Ok(parallel_alloc(len, bound))
}

fn parallel_alloc(len: usize, bound: usize) -> Vec<usize> {
    let core_count = 12;
    let cap = len / core_count;

    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));
    let mut workers = Vec::with_capacity(core_count);
    for _ in 0..12 {
        let sender = Arc::clone(&sender);

        workers.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut lv = vec![0; cap];
            for item in lv.iter_mut().take(cap) {
                *item = rng.gen_range(0..bound);
            }

            sender.lock().unwrap().send(lv).unwrap();
        }));
    }

    let mut v = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    let remain = len % core_count;
    for _ in 0..remain {
        v.push(rng.gen_range(0..bound));
    }

    for _ in 0..core_count {
        let mut rv = receiver.recv().unwrap();
        v.append(&mut rv);
    }

    v
}
