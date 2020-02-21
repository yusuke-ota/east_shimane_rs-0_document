use std::time::Duration;
use async_std::task;

// async-std = { version = "1.5.0", features = ["attributes"] }
#[async_std::main]
async fn main() {
    let handle = count_up_async(1);
    let handle2 = count_up_async(2);
    // println!("{}", handle.await); // handler駆動開始 1:1モデル
    // println!("{}", handle2.await); // handler2駆動開始 1:1モデル
    futures::join!(handle, handle2); // 並行駆動開始 n:1モデル
    println!("Main thread finish!");
}

async fn count_up_async(sleep_time: u64) -> String {
    for counter in 1..10{
        println!("counter is {}", counter);
        println!("{:?}", task::current().id());
        task::sleep(Duration::from_secs(sleep_time)).await;
    }
    "Finish".to_string()
}
