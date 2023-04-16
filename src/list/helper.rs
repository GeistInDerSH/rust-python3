use std::thread::available_parallelism;

#[inline]
pub(super) fn available_cores() -> usize {
    available_parallelism().unwrap().get()
}
