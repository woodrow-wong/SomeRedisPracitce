use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, oneshot};
use std::{io};
#[tokio::main]
async fn main(){
    // let (tx, rx) = oneshot::channel();

    // // 生成一个任务，用于向 oneshot 发送一条消息
    // tokio::spawn(async move {
    //     tx.send("done").unwrap();
    // });

    // tokio::select! {
    //     socket = TcpStream::connect("localhost:3465") => {
    //         println!("Socket connected {:?}", socket);
    //     }
    //     msg = rx => {
    //         println!("received message first {:?}", msg);
    //     }
    // }
    // tokio::spawn(async move {
    //     tx.send(()).unwrap();
    // });

    // let mut listener = TcpListener::bind("localhost:3465").await?;

    // tokio::select! {
    //     res = async {
    //         loop {
    //             let (socket, _) = listener.accept().await?;
    //             tokio::spawn(async move { process(socket) });
    //         }

    //         // 给予 Rust 类型暗示
    //         Ok::<_, io::Error>(())
    //     } => {
    //         res?;
    //     }
    //     _ = rx => {
    //         println!("terminating accept loop");
    //     }
    // }

    // Ok(())
    let (mut tx1, mut rx1) = mpsc::channel(128);
    let (mut tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        // 用 tx1 和 tx2 干一些不为人知的事
        tx1.send("I want to learn").await.unwrap();
        tx2.send("I want to study").await.unwrap();
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("Both channels closed");
        }
    }
}