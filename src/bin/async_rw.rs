use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // // 使用String读取整个文件内容，正确处理所有Unicode字符
    // let mut file = File::open("foo.txt").await?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).await?;
    
    // println!("文件内容: {}", contents);
    // println!("字符数量: {}", contents.chars().count());
    // println!("字节数量: {}", contents.as_bytes().len());
    
    // // 演示UTF-8中字符与字节的对应关系
    // if let Some(first_char) = contents.chars().next() {
    //     let bytes = first_char.to_string().as_bytes().to_vec();
    //     println!("第一个字符 '{}' 由 {} 个字节组成: {:?}", first_char, bytes.len(), bytes);
    // }
    // let mut f = File::open("foo.txt").await?;
    // let mut buffer = Vec::new();

    // let mut file = File::create("foo.txt").await?;

    // let n = file.write(b"some bytes").await?;

    // println!("Wrote the first {} bytes of 'some bytes'.", n);
    // Ok(())
    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await?;

    io::copy(&mut reader, &mut file).await?;
    Ok(())

}