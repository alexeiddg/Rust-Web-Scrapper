use num_cpus;
use std::time::{Duration, Instant};

pub fn calculate_optimal_threads() -> usize {
    let num_cpus = num_cpus::get();
    println!("Number of CPUs: {}", num_cpus);

    // Hill climbing approach to determine optimal threads
    let mut threads = num_cpus;
    let mut best_time = Duration::MAX;

    for trial in 0..num_cpus {
        let current_threads = num_cpus - trial;
        let start = Instant::now();

        // Simulate workload
        let handles: Vec<_> = (0..current_threads).map(|_| {
            std::thread::spawn(|| {
                // Simulate work
                std::thread::sleep(Duration::from_millis(10));
            })
        }).collect();

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed = start.elapsed();
        println!("Trial with {} threads took {:?}", current_threads, elapsed);

        if elapsed < best_time {
            best_time = elapsed;
            threads = current_threads;
        } else {
            break;
        }
    }

    println!("Optimal number of threads determined: {}", threads);
    threads
}
