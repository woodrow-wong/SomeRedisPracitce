use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        // 关闭通道，确保值可以被干净地释放。shutdown()标记写入完成并关闭连接
        wr.shutdown().await ? ;
        // 有时，我们需要给予 Rust 一些类型暗示，它才能正确的推导出类型
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }
        // 删除打印整个缓冲区的行
        
        // 打印实际接收到的内容（以字节形式）
        println!("GOT bytes: {:?}", &buf[..n]);
        
        // 将接收到的字节转换为字符串并打印
        println!("GOT string: {}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}