use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // 创建一个通道用于演示
    let (tx, mut rx) = mpsc::channel(10);
    
    // 创建一个共享的异步互斥锁
    let counter = Arc::new(Mutex::new(0));
    
    // 生产者任务
    let producer = tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });
    
    // 消费者任务 - 使用while let循环处理异步流
    let counter_clone = counter.clone();
    let consumer = tokio::spawn(async move {
        // 这里是关键模式: while let 循环条件中的.await
        while let Some(value) = rx.recv().await {
            println!("接收到值: {}", value);
            
            // 在循环体内使用异步锁
            let mut lock = counter_clone.lock().await;
            *lock += value;
            
            // 重要：在进行其他异步操作前释放锁
            drop(lock);
            
            // 这里可以进行其他异步操作
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });
    
    // 等待任务完成
    producer.await.unwrap();
    consumer.await.unwrap();
    
    // 打印结果
    let final_count = *counter.lock().await;
    println!("最终计数: {}", final_count);
}
