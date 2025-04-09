use tokio::net::{TcpListener, TcpStream};
use tokio::sync::oneshot;
use std::{io};
#[tokio::main]
async fn main() ->io::Result<()>{
    let (tx, rx) = oneshot::channel();

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
    tokio::spawn(async move {
        tx.send(()).unwrap();
    });

    let mut listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            loop {
                let (socket, _) = listener.accept().await?;
                tokio::spawn(async move { process(socket) });
            }

            // 给予 Rust 类型暗示
            Ok::<_, io::Error>(())
        } => {
            res?;
        }
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}