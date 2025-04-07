use mini_redis::client;

#[tokio::main]
async fn main() {
    // 创建到服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // 生成两个任务，一个用于获取 key, 一个用于设置 key
    let t1 = tokio::spawn(async {
        let res = client.get("hello").await;
    });

    let t2 = tokio::spawn(async {
        client.set("foo", "bar".into()).await;
    });

    t1.await.unwrap();
    t2.await.unwrap();
}