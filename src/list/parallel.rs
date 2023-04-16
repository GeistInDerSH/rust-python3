use pyo3::prelude::*;
use rand::Rng;

use crate::list::helper;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[pyfunction(bound = 256)]
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

    Ok(v)
}

#[pyfunction]
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

            sender.lock().unwrap().send(lv).unwrap();
        }));
    }

    let mut v = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    let remain = len % core_count;
    for _ in 0..remain {
        v.push(rng.gen());
    }

    for _ in 0..core_count {
        let mut rv = receiver.recv().unwrap();
        v.append(&mut rv);
    }

    Ok(v)
}
