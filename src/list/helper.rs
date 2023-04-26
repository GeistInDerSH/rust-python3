use std::thread::available_parallelism;

#[inline]
/// Get the number of available cores on the host system
pub(super) fn available_cores() -> usize {
    available_parallelism().unwrap().get()
}
