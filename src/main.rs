use std::hint::black_box;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Config {
    reader_count: usize,
    writer_count: usize,
    reader_pause: u64,
    writer_pause: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reader_count: 0,
            writer_count: 0,

            reader_pause: 1,
            writer_pause: 1,
        }
    }
}

fn run_one_mutex(config: Config, iters: usize) -> Duration {
    let mut threads = vec![];
    let mutex = Arc::new(Mutex::new(0));

    let start_time = Instant::now();

    for _ in 0..config.reader_count {
        let mutex_clone = Arc::clone(&mutex);

        threads.push(thread::spawn(move || {
            for _ in 0..iters {
                let lock = mutex_clone.lock().unwrap();
                thread::sleep(Duration::from_millis(config.reader_pause));
                drop(black_box(lock));
            }
        }));
    }

    for _ in 0..config.writer_count {
        let mutex_clone = Arc::clone(&mutex);
        threads.push(thread::spawn(move || {
            for _ in 0..iters {
                let lock = mutex_clone.lock().unwrap();
                thread::sleep(Duration::from_millis(config.writer_pause));
                drop(black_box(lock));
            }
        }));
    }

    for thread_to_join in threads {
        thread_to_join.join().unwrap();
    }

    return Instant::now() - start_time;
}

fn run_one_rwlock(config: Config, iters: usize) -> Duration {
    let mut threads = vec![];
    let mutex = Arc::new(RwLock::new(0));

    let start_time = Instant::now();

    for _ in 0..config.reader_count {
        let mutex_clone = Arc::clone(&mutex);

        threads.push(thread::spawn(move || {
            for _ in 0..iters {
                let lock = mutex_clone.read().unwrap();
                thread::sleep(Duration::from_millis(config.reader_pause));
                drop(black_box(lock));
            }
        }));
    }

    for _ in 0..config.writer_count {
        let mutex_clone = Arc::clone(&mutex);
        threads.push(thread::spawn(move || {
            for _ in 0..iters {
                let lock = mutex_clone.write().unwrap();
                thread::sleep(Duration::from_millis(config.writer_pause));
                drop(black_box(lock));
            }
        }));
    }

    for thread_to_join in threads {
        thread_to_join.join().unwrap();
    }

    return Instant::now() - start_time;
}

fn main() {
    let iters = 5000;

    // Warmup
    /*let config = Config {
        reader_count: 5,
        writer_count: 5,
        ..Default::default()
    };
    println!("Performing Warmup...");
    black_box(run_one_mutex(config, iters));
    black_box(run_one_rwlock(config, iters));
    println!("Running Bench:");*/

    let available_threads = 12;
    let mut mutex_results = vec![];
    let mut rw_results = vec![];

    for dedicated_for_reading in 0..=available_threads {
        let dedicated_for_writing = available_threads - dedicated_for_reading;

        let config = Config {
            reader_count: dedicated_for_reading,
            writer_count: dedicated_for_writing,
            ..Default::default()
        };

        let mutex_res = run_one_mutex(config, iters);
        let rwlock_res = run_one_rwlock(config, iters);

        mutex_results.push(mutex_res.as_millis());
        rw_results.push(rwlock_res.as_millis());
        println!("{:02} readers @ {:04} ms | {:02} writers @ {:04}", config.reader_count, config.reader_pause, config.writer_count, config.writer_pause);
        println!("Mutex: {:04}", mutex_res.as_millis());
        println!("RW:    {:04}", rwlock_res.as_millis());
    }
    dbg!(mutex_results);
    dbg!(rw_results);
}
