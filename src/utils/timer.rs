use std::time::{Duration, Instant};

pub(crate) fn calculate_elapsed_time(last_time_updated: &mut Instant) -> Duration {
    let current_time = Instant::now();
    let elapsed_time = current_time - *last_time_updated;
    *last_time_updated = current_time;
    elapsed_time
}
