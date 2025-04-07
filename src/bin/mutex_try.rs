use tokio::sync::Mutex; // 使用tokio的异步Mutex

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    // 异步获取锁
    let mut lock = mutex.lock().await;
    *lock += 1;
    
    // 方案1: 在进行异步操作前释放锁
    drop(lock); // 显式释放锁
    do_something_async().await;
    
    // 方案2: 缩小锁的作用域
    // {
    //     let mut lock = mutex.lock().await;
    //     *lock += 1;
    //     // 锁在这里被自动释放
    // }
    // do_something_async().await;
} 

async fn do_something_async() {
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Doing something asynchronously!");
}

#[tokio::main] // 添加tokio入口点宏
async fn main() {
    let mutex = Mutex::new(0);

    // spawn 了一个新的任务来处理这个连接
    tokio::spawn(async move {
        increment_and_do_stuff(&mutex).await;
    }).await.unwrap(); // 等待任务完成
}