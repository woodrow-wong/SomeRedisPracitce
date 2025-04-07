use std::{collections::HashMap, sync::{Arc, Mutex}};
use bytes::Bytes;
// 引入 `mini-redis` 的 `Connection` 和 `Frame` 结构体，
// 这两个结构体是 `mini-redis` 的核心，前者用于处理 TCP 连接，后者用于表示 Redis 的数据帧
// 这里的 `mini-redis` 是一个 Redis 的 Rust 实现，
// 你可以在 `Cargo.toml` 中添加依赖cargo add mini-redis
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
// 修改导入，区分命令枚举和结构体
use mini_redis::Command::{self,Set,Get};

// 引入 `Arc` 和 `Mutex`，用于在多个任务之间共享数据
type Db=Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    // 监听指定地址，等待 TCP 连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // 第二个被忽略的项中包含有新连接的 `IP` 和端口信息
        let (socket, _) = listener.accept().await.unwrap();
        // process(socket).await;

        let db = db.clone();
        println!("Accepted connection from {}", socket.peer_addr().unwrap());
        // 这里的 `tokio::spawn` 是一个异步函数，它会在后台运行一个新的任务
        // 为每一条连接都生成一个新的任务，
        // `socket` 的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
            // spawn 了一个新的任务来处理这个连接
            process(socket,db).await;
        });
        // 这里的 `process` 函数是一个异步函数，返回一个 `Future`，而不是直接返回一个值
    }
}


async fn process(socket: TcpStream,db: Db) {
    // `mini-redis` 提供的 `Connection` 结构体用于处理 TCP 连接

    // // 使用 hashmap 来存储 redis 的数据
    // let mut db = HashMap::new();

    // `mini-redis` 提供的便利函数，使用返回的 `connection` 可以用于从 socket 中读取数据并解析为数据帧
    let mut connection = Connection::new(socket);

    // 使用 `read_frame` 方法从连接获取一个数据帧：一条redis命令 + 相应的数据
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // // 值被存储为 `Vec<u8>` 的形式
                // db.insert(cmd.key().to_string(), cmd.value().to_vec());
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`， 该类型会在后面章节讲解，
                    // 此时，你只要知道 `&Vec<u8>` 可以使用 `into()` 方法转换成 `Bytes` 类型
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 将请求响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
}