use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Task {
    id: u32,
    work: String,
}

struct WorkerPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Task>>,
}

struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

struct SharedResults {
    completed: HashMap<u32, String>,
    in_progress: u32,
    all_work_sent: bool,
}

impl WorkerPool {
    fn new(size: usize, results: Arc<Mutex<SharedResults>>) -> Self {
        let (sender, receiver) = mpsc::channel::<Task>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::new();

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let results = Arc::clone(&results);

            let handle = thread::spawn(move || {
                loop {
                    let task = {
                        let rx = receiver.lock().unwrap();
                        rx.recv()
                    };

                    match task {
                        Ok(task) => {
                            {
                                let mut r = results.lock().unwrap();
                                r.in_progress += 1;
                            }

                            println!("Worker {} processing task {}", id, task.id);
                            thread::sleep(Duration::from_millis(400));

                            {
                                let mut r = results.lock().unwrap();
                                r.completed.insert(task.id, format!("Done by {}", id));
                                r.in_progress -= 1;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });

            workers.push(Worker {
                id,
                handle: Some(handle),
            });
        }

        WorkerPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute(&self, task: Task) {
        self.sender.as_ref().unwrap().send(task).unwrap();
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        self.sender.take();

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                handle.join().unwrap();
            }
        }
    }
}

fn spawn_monitor(results: Arc<Mutex<SharedResults>>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(250));
            let r = results.lock().unwrap();
            println!(
                "[Monitor] Complete: {}, Working: {}",
                r.completed.len(),
                r.in_progress
            );

            if r.all_work_sent && r.in_progress == 0 {
                break;
            }
        }
    })
}

fn main() {
    let results = Arc::new(Mutex::new(SharedResults {
        completed: HashMap::new(),
        in_progress: 0,
        all_work_sent: false,
    }));

    let monitor = spawn_monitor(Arc::clone(&results));

    let pool = WorkerPool::new(3, Arc::clone(&results));

    for i in 0..8 {
        pool.execute(Task {
            id: i as u32,
            work: format!("Task-{}", i),
        });
    }

    {
        let mut r = results.lock().unwrap();
        r.all_work_sent = true;
    }

    drop(pool);
    monitor.join().unwrap();

    let final_results = results.lock().unwrap();
    println!("\nCompleted {} tasks", final_results.completed.len());
}
