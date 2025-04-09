use std::{pin::Pin, task::{Context, Poll}};
use tokio::sync::oneshot;
use std::future::Future;  // 需要添加这个导入

async fn some_operation() -> &'static str {
    // 在这里执行一些操作...
    "operation result"  // 返回静态字符串字面量，不再创建String
}

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,  // 恢复原来的类型
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1 completed first with {:?}", val);
            return Poll::Ready(());
        }

        if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2 completed first with {:?}", val);
            return Poll::Ready(());
        }

        Poll::Pending
    }
}


#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        // 这里实际上编译器为我们隐式地插入了`move`关键字
        // 等价于 tokio::spawn(async move { ... })
        
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // 收到了发送端发来的关闭信号
                // `select` 即将结束，此时，正在进行的 `some_operation()` 任务会被取消，任务自动完成，
                // tx1 被释放
                // 这是因为 select! 宏会在一个分支完成时取消其他所有分支的异步操作
            }
        }
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    // tokio::select! {
    //     val = rx1 => {
    //         println!("rx1 completed first with {:?}", val);
    //     }
    //     val = rx2 => {
    //         println!("rx2 completed first with {:?}", val);
    //     }
    // }

    MySelect {
        rx1,
        rx2,
    }.await;

    // 任何一个 select 分支结束后，都会继续执行接下来的代码
}

// 在这里解释 tokio::select! 宏和 val 的含义:
// 
// tokio::select! 宏的工作原理:
// 1. 它同时等待多个异步操作，只要其中一个完成，就执行对应的分支代码
// 2. 当一个分支执行后，其他分支的异步操作会被自动取消
// 3. 每个分支的格式为: 模式 = 异步表达式 => 要执行的代码
//
// val 的含义:
// - val 是一个变量名，用来接收异步操作完成后的返回值
// - 例如 `val = some_operation() => { ... }` 中，val 将接收 some_operation() 的返回值
// - 如果不需要使用返回值，可以用 _ 代替，如 `_ = tx1.closed() => { ... }`
//
// 在第一个 select! 中:
//   - val 接收 some_operation() 返回的 String
//   - 第二个分支使用 _ 忽略了返回值
//
// 在第二个 select! 中:
//   - val 接收 rx1 或 rx2 的结果，类型是 Result<T, oneshot::error::RecvError>