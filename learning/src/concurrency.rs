/// Traits
/// Send - allow to send the data between threads (channels)
/// Sync - we can access the type safely from different threads
/// Manually implementing Sync and Send is not safe

pub mod concurrent_work_with_threads;
pub mod sending_data_between_threads;
pub mod parallelism_with_common_state;
