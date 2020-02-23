use std::time::Duration;
use async_std::task;
use std::thread;

// async-std = {version = "1.5.0", features = ["attributes"]}
#[async_std::main]
async fn main() {
    println!("{}", count_up_async(1).await); // handler駆動開始 n:mモデル
    println!("{}", count_up_async(1).await); // handler2駆動開始 n:mモデル
    println!("Main thread finish!");
}

async fn count_up_async(sleep_time: u64) -> String {
    let mut async_tasks = vec![];
    for counter in 1..10{
        // 実行せずに、ひたすら async_tasksにtaskを詰める
        async_tasks.push(
            task::spawn(async move {
                println!("counter is {} {:?} {:?}",
                         counter, task::current().id(), thread::current().id());
                task::sleep(Duration::from_secs(sleep_time)).await;
            }));
    }
    // 最後に一括で取り出す
    for async_task in async_tasks { async_task.await };
    "Finish".to_string()
}