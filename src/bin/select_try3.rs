use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use std::io;
use std::net::SocketAddr;

// async fn race(
//     data: &[u8],
//     addr1: SocketAddr,
//     addr2: SocketAddr
// ) -> io::Result<()> {
//     tokio::select! {
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr1).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         Ok(_) = async {
//             let mut socket = TcpStream::connect(addr2).await?;
//             socket.write_all(data).await?;
//             Ok::<_, io::Error>(())
//         } => {}
//         else => {}
//     };

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     let (tx1, rx1) = oneshot::channel();
//     let (tx2, rx2) = oneshot::channel();

//     let mut out = String::new();

//     tokio::spawn(async move {
//     });

//     tokio::select! {
//         _ = rx1 => {
//             out.push_str("rx1 completed");
//         }
//         _ = rx2 => {
//             out.push_str("rx2 completed");
//         }
//     }

//     println!("{}", out);
// }

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send("value1").await.unwrap();
        tx2.send("value2").await.unwrap();
        tx3.send("value3").await.unwrap();
    });

    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {}", msg);
    }

    println!("All channels have been closed.");
}