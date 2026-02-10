use tokio::sync::mpsc;

enum Progress {
    Started(u32),
    Completed(u32, String),
}

#[tokio::main]
async fn main() {
    let tasks: Vec<u32> = (0..10).collect();
    let (tx, mut rx) = mpsc::channel(32);

    let monitor = tokio::spawn(async move {
        let mut in_progress = 0u32;
        let mut completed = 0u32;

        while let Some(msg) = rx.recv().await {
            match msg {
                Progress::Started(id) => {
                    in_progress += 1;
                    println!(
                        "Started task {} (in progress: {}, completed: {})",
                        id, in_progress, completed
                    );
                }
                Progress::Completed(id, result) => {
                    in_progress -= 1;
                    completed += 1;
                    println!(
                        "Finished task {}: {} (in progress: {}, completed: {})",
                        id, result, in_progress, completed
                    );
                }
            }
        }

        println!("All {} tasks complete", completed);
    });

    let mut handles = Vec::new();

    for id in tasks {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            tx.send(Progress::Started(id)).await.unwrap();

            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            let result = format!("Done by task {}", id);

            tx.send(Progress::Completed(id, result)).await.unwrap();
        }));
    }

    drop(tx);

    for handle in handles {
        handle.await.unwrap();
    }

    monitor.await.unwrap();
}