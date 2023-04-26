use pyo3::prelude::*;
use rand::Rng;

use crate::list::helper;
use pyo3::exceptions::{PyChildProcessError, PyException};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[pyfunction(bound = 256)]
/// Generate a list of random numbers, with a set bound for the random numbers
pub fn list_bounded(len: usize, bound: usize) -> PyResult<Vec<usize>> {
    let core_count = helper::available_cores();
    let cap = len / core_count;

    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));
    let mut workers = Vec::with_capacity(core_count);
    for _ in 0..core_count {
        let sender = Arc::clone(&sender);

        workers.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut lv = vec![0; cap];
            for item in lv.iter_mut().take(cap) {
                *item = rng.gen_range(0..bound);
            }

            return match sender.lock() {
                Ok(mtx) => match mtx.send(lv) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(PyException::new_err(err.to_string())),
                },
                Err(err) => Err(PyException::new_err(err.to_string())),
            };
        }));
    }

    let mut v = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    let remain = len % core_count;
    for _ in 0..remain {
        v.push(rng.gen_range(0..bound));
    }

    for _ in 0..core_count {
        let mut rv = match receiver.recv() {
            Ok(v) => Ok(v),
            Err(err) => Err(PyChildProcessError::new_err(err.to_string())),
        }?;
        v.append(&mut rv);
    }

    Ok(v)
}

#[pyfunction]
/// Generate a list of random numbers, with no bound on the random numbers
pub fn list(len: usize) -> PyResult<Vec<usize>> {
    let core_count = helper::available_cores();
    let cap = len / core_count;

    let (sender, receiver) = mpsc::channel();
    let sender = Arc::new(Mutex::new(sender));
    let mut workers = Vec::with_capacity(core_count);
    for _ in 0..core_count {
        let sender = Arc::clone(&sender);

        workers.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();

            let mut lv = vec![0; cap];
            for item in lv.iter_mut().take(cap) {
                *item = rng.gen();
            }

            return match sender.lock() {
                Ok(mtx) => match mtx.send(lv) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(PyException::new_err(err.to_string())),
                },
                Err(err) => Err(PyException::new_err(err.to_string())),
            };
        }));
    }

    let mut v = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    let remain = len % core_count;
    for _ in 0..remain {
        v.push(rng.gen());
    }

    for _ in 0..core_count {
        let mut rv = match receiver.recv() {
            Ok(v) => Ok(v),
            Err(err) => Err(PyChildProcessError::new_err(err.to_string())),
        }?;
        v.append(&mut rv);
    }

    Ok(v)
}
