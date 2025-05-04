// Cargo dependency:
// tokio = { version = "1.37.0", features = ["macros", "rt-multi-thread"] }

use tokio::time::{Duration, sleep};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let handle = tokio::runtime::Handle::current();
    assert_eq!(
        tokio::runtime::RuntimeFlavor::MultiThread,
        handle.runtime_flavor()
    );
    println!("Handle: {:?}", handle);
    println!("Main: start {}", ct_id());
    let task1 = handle.spawn(async {
        println!("Task 1: start {}", ct_id());
        let handle = tokio::runtime::Handle::current();
        let task2 = handle.spawn(async {
            println!("Task 2: start {}", ct_id());
            sleep(Duration::from_secs(1)).await;
            println!("Task 2: end {}", ct_id());
        });
        sleep(Duration::from_secs(2)).await;
        println!("Task 1: sleeped {}", ct_id());
        task2.await.unwrap();
        println!("Task 1: end {}", ct_id());
    });

    sleep(Duration::from_secs(3)).await;

    println!("Main: sleeped {}", ct_id());
    task1.await.unwrap();
    println!("Main: end {}", ct_id());
}

fn ct_id() -> String {
    let t = std::thread::current();
    format!("'{}' {:?}", t.name().unwrap_or_default(), t.id())
}
