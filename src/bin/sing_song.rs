use tokio::{self, join};
use tokio::{time};

#[tokio::main]
async fn main() {
    // sing().await; //里面有休眠3秒的逻辑，按道理休眠后，会异步调用dance()乃至say_to_world()都有可能
    // dance().await;

    // let s = say_to_world().await;
    // println!("{}", s)
    // 同时运行三个 future
    // 花费的时间约等于最久的那一个
    join!(sing(), dance(), async {
        let s = say_to_world().await;
        println!("{}", s)
    });
}

async fn get_song() -> String{
    println!("finish the song");
    String::from("song name abcd...")
}

async fn sing(){
    let s = get_song().await;
    time::sleep(time::Duration::from_secs(3)).await; // 这里进行了休眠3秒
    println!("it`s singing song: {}", s)
}

async fn dance(){
    println!("now is dancing...")
}

async fn say_to_world() -> String {
    String::from("world")
}


// 实际上 async/await 的用法你可能有点误解了。

// 你首先可以把整个 main 函数看作一个最大的 future。

// 其下有三个小的 future，分别是 sing、dance 和 print。对小的 future 使用 .await 意味着「父 future 执行时，等待子 future 完成」。

// 也就是说，这个时候实际上是 main 进行 .await，等待着一个子 future 完成，这时控制权是可以交给和 main 以外的 future 的，不过这个例子里面只有 main 这么一个最大的 future，所以表现出来就是一直到 sing 完成了，dance 才开始。

// 上面的言辞可能不是那么准确，但大概的意思是这样的。join! 宏就是相当于帮你同时运行几个 future，在它们之间进行调度。


// 感谢你的回复，我已经了解了，现在才看到你的回复。
// 很多文章或书本没有很深入去解释 .await 、 join!宏等等的机制，所以可能会有不少人在使用上会出现各种理解之外的情况。

// 像@LovesAsuna说的其实也可以理解，既然要异步并行了，直接点全都调度就好，当然很方便，但rust这样做，可能是为了保持异步使用的多样性来适用不同的场景需求。

// 我的问题其实还可以使用tokio::task 或者 tokio::spawn的函数进行封装调用，这样他们之间就会在任务等待时释放cpu资源，在多任务间并发调度了。不过相对.await，这样做就不会【等待完成】，执行是会执行但不知道什么时候执行，main也不会等你执行完才结束，可能你们没执行完我就先下班，你们也被迫下班了


// 这个 process 函数里的 match 臂，顺序是有影响的。因为前面的 db 需要推断泛型参数，而只有 Set 分支能推断出完整的 HashMap<String, Vec<u8>> 类型。如果先遇到 Get 分支就推断不全。语义上 match 各分支是没有先后的，类型推断也应当尽力而为，感觉这是语言的一个缺陷。

// 用的是 rust 1.67.1 版本。

// EDIT：搜了一下发现从 2015 年传承至今。rust-lang/rust#25165


// 这里hashmap的编译器类型推导很有意思。在match里面，如果对换了Set和Get会导致编译不通过：只有db.insert才可以推导出类型，而先Get的话，db.get不知道是个什么。 然而写代码容易会先写出先get后set的习惯

// 因为 Rust 在 match 表达式中会根据第一个分支推断返回类型，然后检查后续分支是否匹配，Frame::Bulk、Frame::Null 和 Frame::Simple 都是 Frame 的不同变体，属于枚举类型 Frame，所以按理说可以兼容。但 Rust 在 match 的类型推断时是从第一个分支开始的，如果第一个分支返回的 Frame 变体不够通用，就可能导致后续分支类型无法匹配。然而Rust 允许 Frame::Bulk、Frame::Null 作为 Frame 类型的变体匹配，所以不会报错
// 但是 Set 分支返回的 Frame::Simple 是一个具体的变体，Rust 无法推断出它是 Frame 的一个变体，所以会报错。
// 这就是为什么你在 match 中的顺序会影响编译的原因。为了避免这种问题，可以使用类型注解来明确指定返回类型，或者将所有分支的返回类型都设置为相同的变体。这样就可以避免 Rust 在推断类型时出现问题。   