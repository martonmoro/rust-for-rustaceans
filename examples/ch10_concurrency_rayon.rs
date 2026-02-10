use rayon::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Progress {
    Started(u32),
    Completed(u32, String),
    AllDone,
}

fn main () {
    let tasks: Vec<u32> = (1..=32).collect();
    let (tx, rx) = mpsc::channel::<Progress>();

    let monitor = thread::spawn(move || {
        let mut in_progress = 0u32;
        let mut completed = 0u32;

        for msg in rx {
            match msg {
                Progress::Started(id) => {
                    in_progress += 1;
                    println!("Task {} started. In progress: {}", id, in_progress);
                }
                Progress::Completed(id, result) => {
                    in_progress -= 1;
                    completed += 1;
                    println!("Finished task {}: {} (in progress: {}, completed: {})", id, result, in_progress, completed);
                }
                Progress::AllDone => {
                    println!("All {} tasks completed!", completed);
                    break;
                }
            }
        }
    });

    tasks.par_iter().for_each(|&id| {
        tx.send(Progress::Started(id)).unwrap();

        thread::sleep(Duration::from_millis(500));
        let result = format!("Done by {:?}", thread::current().id());

        tx.send(Progress::Completed(id, result)).unwrap();
    });

    tx.send(Progress::AllDone).unwrap();
    monitor.join().unwrap();
}