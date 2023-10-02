// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, time};

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            let now = time::Instant::now();
            thread::sleep(Duration::from_millis(250));
            if now.elapsed() >= Duration::from_millis(250) {
                status_shared.lock().unwrap().jobs_completed += 1;
            }
        });
        handles.push(handle);
    }

    let status_log = Arc::clone(&status);
    for handle in handles {
        handle.join().unwrap();
        println!(
            "jobs completed {}",
            status_log.lock().unwrap().jobs_completed
        );
    }
}
