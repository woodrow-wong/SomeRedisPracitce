use std::fmt::Display;

// 添加一个带Drop trait的结构体来观察变量生命周期结束时的行为
struct WithDrop(String);

impl Drop for WithDrop {
    fn drop(&mut self) {
        println!("正在销毁: {}", self.0);
    }
}

fn main() {
  let r1;
  let r2;
  {
    static STATIC_EXAMPLE: i32 = 42;
    r1 = &STATIC_EXAMPLE;
    let x = "&'static str";
    r2 = x;
    // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
  }

  println!("&'static i32: {}", r1); // -> 42
  println!("&'static str: {}", r2); // -> &'static str

//   演示非静态生命周期的引用
//   let r3;
//   let r4;
//   {
//     let local_var = 10; // 局部变量，非static
//     let local_str = String::from("hello"); // 局部String变量，非static
    
//     r3 = &local_var; // 编译错误：`local_var` 不活得够久
//     r4 = &local_str; // 编译错误：`local_str` 不活得够久
    
//     // local_var和local_str在这里离开作用域并被释放
//   }
  
//   // 到这里 r3 和 r4 将成为悬垂引用，Rust 编译器会阻止这种情况
//   println!("{}, {}", r3, r4); // 无法编译

  // 演示变量生命周期的开始时间
  {
    // 变量a的生命周期从这里开始，虽然它还没有被赋值
    let a;
    
    // 在这段代码中，a已经存在，但未初始化，不能使用
    // println!("{}", a); // 编译错误：使用了可能未初始化的变量
    
    // 这里只是给a赋值，而不是开始a的生命周期
    a = 10;
    
    println!("a: {}", a); // 现在可以使用a了
    
    // 变量b的生命周期也是从声明开始，同时进行了初始化
    let b = 20;
    
    // 变量遮蔽(shadowing)演示：这里创建了一个新的变量b，旧的b被遮蔽
    let b = b + 1; // 新的b的生命周期从这里开始
    
    println!("b: {}", b);
    
    // a和b的生命周期在这个作用域结束时终止
  }

  // 演示变量遮蔽(shadowing)时旧变量的生命周期
  {
    // 创建一个简单变量并持有其引用
    let x = 10;
    let ref_to_first_x = &x;
    
    println!("原始x的值: {}", x);
    
    // 遮蔽变量x
    let x = 20;
    
    // 我们仍然可以通过先前的引用访问原始的x
    println!("通过引用访问旧x: {}", ref_to_first_x);  // 输出: 10
    println!("新x的值: {}", x);  // 输出: 20
    
    // 使用带Drop特性的类型来观察生命周期
    let data = WithDrop("第一个数据".to_string());
    let _ref_to_data = &data;
    
    // 遮蔽data变量
    let data = WithDrop("第二个数据".to_string());
    
    println!("作用域即将结束...");
    // 在作用域结束时，两个data变量都会被drop，按照相反的声明顺序
    // 先销毁"第二个数据"，再销毁"第一个数据"
  }
  println!("作用域已结束");

  {
    let s1 = "Hello World By s1!".to_string();

    // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
    // 充分说明这个约束是多么的弱。。
    static_bound(&s1);

    // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
    // r3 = &s1;

    // s1 在这里被 drop
  }
  // println!("{}", r3);

  // 演示 T: 'static 与 &'static T 的区别
  {
    println!("\n--- T: 'static 与 &'static T 的区别 ---");
    
    // &'static T 示例: 一个具有静态生命周期的引用
    // 静态引用指向的数据必须在整个程序运行期间都有效
    let static_str: &'static str = "我是字符串字面量，有静态生命周期";
    static STATIC_NUM: i32 = 100;
    let static_num_ref: &'static i32 = &STATIC_NUM;
    
    println!("&'static 示例 - 指向静态数据的引用:");
    println!("  静态字符串: {}", static_str);
    println!("  静态数字: {}", static_num_ref);
    
    // T: 'static 示例：类型T满足static约束
    // 两类情况：
    // 1. 不包含任何引用的类型(如i32、String、Vec等)
    // 2. 只包含'static引用的类型
    
    // 不包含引用的类型（拥有自身数据）
    let owned_string: String = "我是一个拥有自己数据的String".to_string(); // String: 'static
    let owned_vector: Vec<i32> = vec![1, 2, 3];  // Vec<i32>: 'static
    
    // 编译器接受这些调用，因为这些类型满足T: 'static约束
    static_bound(&owned_string);
    // static_bound(&owned_vector);
    static_bound(&10); // 对i32字面量的引用也满足T: 'static
    
    // 不能创建指向这些堆数据的静态引用
    // let illegal_static_ref: &'static String = &owned_string; // 编译错误
    
    println!("\nT: 'static 约束比 &'static T 宽松得多：");
    println!("  - &'static T：引用必须指向静态数据");
    println!("  - T: 'static：T要么自己拥有数据(如String)，要么只包含静态引用");
  }

  {
    let s1 = "Hello World By s1!".to_string();
    // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
    // 充分说明这个约束是多么的弱。。
    static_bound(&s1);

    // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
    // r3 = &s1;

    // s1 在这里被 drop
  }
  // println!("{}", r3);

  // 生命周期的本质：针对引用而非变量或值
  {
    println!("\n--- 生命周期的本质 ---");
    
    // 创建一个String值，它由变量owned_string拥有
    let owned_string = String::from("这是一个String值");
    
    // 创建一个指向owned_string的引用
    let string_ref = &owned_string;
    
    // 在Rust中:
    // 1. owned_string(变量)有自己的作用域
    // 2. String值有自己的生命周期，由owned_string控制
    // 3. string_ref(引用)有自己的生命周期，必须比被引用的值短
    
    println!("变量和值的关系:");
    println!("  - 变量(owned_string)：拥有值，控制值的生命周期");
    println!("  - 值(String内容)：随拥有它的变量创建和销毁");
    println!("  - 引用(string_ref)：生命周期不能超过被引用的值");
    
    // // 生命周期参数在函数中的实际含义
    // let result = demonstrate_lifetime(&owned_string);
    // println!("函数返回的引用: {}", result);
    
    // 嵌套作用域示例
    {
      let inner_value = 42;
      // 可以创建内部值的引用
      let inner_ref = &inner_value;
      println!("内部引用: {}", inner_ref);
      
      // 不能从函数返回inner_value的引用，因为inner_value的生命周期太短
      // let invalid_ref = return_inner_ref(&inner_value);
    }
    
    println!("\n生命周期的核心概念:");
    println!("  1. 生命周期主要是针对引用的概念，而非变量或值本身");
    println!("  2. 生命周期注解(如'a)表示引用的有效范围");
    println!("  3. 编译器通过生命周期确保引用不会比它们引用的数据存活更长");
    println!("  4. 变量有作用域，值有存在时间，引用有生命周期");
  }

  // 引用的本质解释
  {
    println!("\n--- 引用的本质 ---");
    
    // 在Rust中，引用本身是一个值，这个值存储的是另一个值的内存地址
    let original = 42;
    
    // &original 创建了一个引用值，这个值代表original的内存位置
    // ref_var 是一个变量，它存储了这个引用值
    let ref_var = &original;
    
    println!("引用的基本概念:");
    println!("  - 原始值: {}", original);
    println!("  - 通过引用访问: {}", *ref_var);
    
    // 引用作为值的证明
    // 1. 引用可以被复制到另一个变量
    let another_ref = ref_var;  // another_ref也指向original
    
    // 2. 引用可以作为函数参数传递
    print_ref(ref_var);
    
    // 3. 引用可以从函数返回
    let returned_ref = return_same_ref(ref_var);
    println!("  - 函数返回的引用: {}", *returned_ref);
    
    // 4. 引用可以存储在数据结构中
    let ref_vec = vec![&original, &original];
    println!("  - 存储在向量中的引用: {} {}", *ref_vec[0], *ref_vec[1]);
    
    // 从概念上讲:
    // - 引用是一种特殊的值，它"指向"其他值
    // - 引用本身可以被存储在变量中，就像其他值一样
    // - 变量(如ref_var)持有引用值，而不是直接"成为"引用
    
    println!("\n引用的本质总结:");
    println!("  1. 引用是一个值，这个值存储了其他值的内存地址");
    println!("  2. 引用值可以被存储在变量中，就像整数等其他值一样");
    println!("  3. 引用本身也有类型，如&i32、&mut String等");
    println!("  4. 生命周期注解(如'a)描述引用值的有效范围，不是变量的有效范围");
  }

  // T: 'static 约束的本质解释
  {
    println!("\n--- T: 'static 约束的本质 ---");
    
    // T: 'static 中的T确实是泛型类型参数
    // 但这个约束的含义不是"T的引用必须是静态的"
    // 而是"T这个类型本身满足'static约束"
    
    println!("T: 'static 的真正含义:");
    println!("  1. T不包含任何非静态引用");
    println!("  2. 也就是说，T要么是自持有数据的类型，要么只包含'static引用");
    
    // 示例1: 自持有类型(owned types)自然满足T: 'static
    let num: i32 = 42;           // i32: 'static √
    let text: String = "hello".into(); // String: 'static √
    let vec: Vec<i32> = vec![1, 2, 3]; // Vec<i32>: 'static √
    
    // 这些类型都可以传递给要求T: 'static的函数
    explain_static_bound(&num);
    explain_static_bound(&text);
    // explain_static_bound(&vec);
    
    // 示例2: 包含'static引用的类型也满足T: 'static
    let static_str_ref: &'static str = "静态字符串";  // &'static str: 'static √
    explain_static_bound(&static_str_ref);
    
    // 示例3: 包含非静态引用的类型不满足T: 'static
    {
      let local_string = String::from("局部字符串");
      let string_ref = &local_string;  // 注意: string_ref类型是&String，有非静态生命周期
      
      // 错误: string_ref类型包含非静态引用，不满足T: 'static
      // explain_static_bound(&string_ref); // 如果取消注释会编译失败
      
      // 但可以传递local_string本身，因为String是自持有类型
      explain_static_bound(&local_string);
    }
    
    println!("\n总结:");
    println!("  - T: 'static是对类型T的约束，而不是对引用的约束");
    println!("  - 这个约束要求T不包含任何可能在程序运行期间失效的引用");
    println!("  - 所有不包含引用的类型(如String)都自动满足T: 'static");
  }

  // 引用与变量的生命周期
  {
    println!("\n--- 引用生命周期与变量生命周期的关系 ---");
    
    // 示例1: 基本的引用生命周期
    {
      let value = 42;  // 变量value的生命周期从这里开始
      
      // 引用的生命周期从创建时开始
      let ref_a = &value;  // 引用生命周期开始，变量ref_a的生命周期也开始
      
      println!("引用值: {}", ref_a);
      
      // 在这里，引用仍在被使用，所以它的生命周期继续存在
      
    } // 作用域结束时:
      // 1. 变量ref_a的生命周期结束(因为其作用域结束)
      // 2. 引用&value的生命周期结束(最后一次使用后)
      // 3. 变量value的生命周期结束(因为其作用域结束)
    
    // 示例2: 引用可以在其最后使用之后结束生命周期，即使变量仍在作用域内
    {
      let value = 100;
      
      let ref_value = &value;  // 引用生命周期开始
      println!("使用引用: {}", ref_value);  // 引用的最后一次使用
      
      // 在这行之后，编译器可以认为引用的生命周期已经结束
      // 尽管持有引用的变量ref_value仍在作用域内
      
      // 因为引用生命周期结束，现在可以获取value的可变引用
      // (如果引用仍然活跃，这会违反借用规则)
    //   let _mutable_ref = &mut *(&value as *const i32 as *mut i32);
      
      println!("引用变量在作用域内，但它持有的引用已不再活跃");
    } // 变量ref_value的生命周期在这里结束
    
    // 示例3: 持有引用的变量与引用本身
    {
      let outer_val = String::from("外部值");
      
      // 创建一个嵌套作用域
      {
        let inner_val = String::from("内部值");
        
        // 两个引用变量，分别指向不同生命周期的值
        let ref_to_outer = &outer_val; // 引用指向outer_val
        let ref_to_inner = &inner_val; // 引用指向inner_val
        
        println!("内部作用域中的引用:");
        println!("  - 外部引用: {}", ref_to_outer);
        println!("  - 内部引用: {}", ref_to_inner);
        
        // 此时两个变量(ref_to_outer和ref_to_inner)的生命周期相同
        // 但它们持有的引用的生命周期不同
      } // 内部作用域结束:
        // 1. ref_to_inner变量生命周期结束
        // 2. inner_val生命周期结束
        // 3. ref_to_inner持有的引用生命周期必须在此处之前结束
        // 4. ref_to_outer变量生命周期结束
        // 5. ref_to_outer持有的引用可以继续存在，因为它引用的值仍然有效
      
      // 可以创建一个新引用变量，指向仍然有效的outer_val
      let another_ref = &outer_val;
      println!("外部作用域中的引用: {}", another_ref);
    } // outer_val的生命周期在这里结束
    
    println!("\n引用与变量生命周期的关键区别:");
    println!("  1. 变量的生命周期是由其作用域决定的");
    println!("  2. 引用的生命周期从创建开始，到最后一次使用结束");
    println!("  3. 引用的生命周期必须包含在被引用值的生命周期内");
    println!("  4. 持有引用的变量的生命周期和引用本身的生命周期是不同的概念");
    println!("  5. 编译器可能会缩短引用的生命周期(非词法生命周期)");
  }

  // 引用生命周期与变量作用域的关系
  {
    println!("\n--- 引用生命周期与变量作用域的关系 ---");
    
    println!("核心原则澄清:");
    println!("  1. 引用的生命周期必须小于或等于它所指向变量的作用域");
    println!("  2. 持有引用的变量的作用域与被引用变量的作用域没有强制关系");
    
    // 示例1: 引用的生命周期必须小于或等于被引用变量的作用域
    {
      // 演示正确的关系
      let outer_value = String::from("外部值");  // 外部变量有更长的作用域
      
      {
        let ref_to_outer = &outer_value;  // 引用指向外部变量，这是安全的
        println!("  内部引用访问外部值: {}", ref_to_outer);
      } // ref_to_outer的作用域结束，但outer_value仍然有效
      
      // 相反方向是不允许的:
      // 不能让引用指向生命周期比它短的变量
    //   let outer_ref;
      {
        let inner_value = String::from("内部值");
        // 以下代码如果取消注释将导致编译错误
        // outer_ref = &inner_value; // 错误: inner_value的作用域太短
        
        // 下面这种用法是合法的，因为引用仅在内部作用域使用
        let inner_ref = &inner_value;
        println!("  内部引用: {}", inner_ref);
      } // inner_value在这里被释放
      
      // 如果上面的赋值被允许，这里将使用指向已释放内存的引用
      // println!("  悬垂引用: {}", outer_ref); // 这就是Rust要防止的！
    }
    
    // 示例2: 持有引用的变量作用域与引用生命周期
    {
      // 创建一个持有引用的变量，但不使用这个引用
      let outer_val = 42;
      let ref_holder = &outer_val;  // ref_holder持有对outer_val的引用
      
      // 在Rust中，引用的生命周期是自引用创建到最后一次使用
      // 而不是简单地等同于持有它的变量的作用域
      
      println!("  持有引用的变量: {:p}", ref_holder);  // 使用引用
      
      // 之后，尽管ref_holder变量仍在作用域内，但其引用可能已被编译器
      // 认为生命周期结束（因为不再使用）
      
      // 非词法作用域（Non-Lexical Lifetimes, NLL）允许更精确地跟踪引用的实际使用
    //   let _mutable = &mut *(&outer_val as *const i32 as *mut i32); // 可以创建可变引用
    }
    
    println!("\n结论:");
    println!("  - 引用的生命周期不能超过被引用变量的作用域（这是安全保证）");
    println!("  - 持有引用的变量可以有任意作用域，但其中的引用仍受上述规则约束");
    println!("  - Rust的借用检查器强制执行这些规则，防止悬垂引用");
  }

  // 普通值的存在时间与变量作用域的关系
  {
    println!("\n--- 普通值的存在时间与变量作用域的关系 ---");
    
    println!("基本原则：");
    println!("  1. 普通值的存在时间通常等于拥有它的变量的作用域");
    println!("  2. 但有重要例外：所有权转移会改变这一关系");
    
    // 示例1：基本情况 - 值的存在时间等于变量作用域
    {
      let value = 42;  // 值42的存在时间开始于变量value的创建
      println!("  基本整数值: {}", value);
    } // 值42的存在时间在这里结束，随变量value一起
    
    // 示例2：所有权转移 - 值的存在时间可能超过原变量作用域
    {
      let original_owner;
      {
        let temp_string = String::from("我的所有权将被转移");
        println!("  临时作用域中的字符串: {}", temp_string);
        
        original_owner = temp_string;  // 所有权从temp_string转移到original_owner
        // temp_string不再拥有这个字符串值，但该值继续存在
        
        // println!("{}", temp_string);  // 错误：使用了移动后的值
      } // temp_string的作用域结束，但字符串值仍然存在！
      
      // 字符串值仍然存在，因为它现在归original_owner所有
      println!("  原作用域结束后，值仍存在: {}", original_owner);
    } // 此时original_owner作用域结束，字符串值才被释放
    
    // 示例3：移动语义对存在时间的影响
    {
      let mut v = Vec::new();
      v.push(String::from("第一个字符串"));
      
      let first_string = v.remove(0);  // 所有权从Vec转移到first_string
      
      println!("  从集合中移出的值: {}", first_string);
      // 现在字符串值的存在时间由first_string控制，而不是v
    }
    
    // 示例4：Rc/Arc共享所有权 - 值的存在时间取决于最后一个所有者
    {
      use std::rc::Rc;
      
      let value_rc;
      {
        // 创建一个引用计数的共享值
        let first_owner = Rc::new(String::from("共享所有权的值"));
        
        // 增加引用计数，创建第二个所有者
        value_rc = Rc::clone(&first_owner);
        
        println!("  内部作用域中: {} (引用计数: {})", 
                 first_owner, Rc::strong_count(&first_owner));
      } // first_owner离开作用域，但值继续存在，因为value_rc仍然持有它
      
      println!("  外部作用域中: {} (引用计数: {})", 
               value_rc, Rc::strong_count(&value_rc));
    } // value_rc离开作用域，引用计数为0，值被销毁
    
    println!("\n重要区别：");
    println!("  - 对于栈上的简单值：存在时间通常等于变量作用域");
    println!("  - 对于堆上的复杂值：存在时间等于最后一个拥有者的作用域");
    println!("  - 移动语义使得值的存在时间可以超过原始变量的作用域");
    println!("  - 共享所有权时，值的存在时间取决于最后一个所有者何时被销毁");
  }

  // Copy类型值的存在时间特性
  {
    println!("\n--- Copy类型值的存在时间特性 ---");
    
    println!("对于实现了Copy trait的类型：");
    println!("  1. 每次赋值或传递都会创建独立副本");
    println!("  2. 每个副本的存在时间严格等于拥有它的变量的作用域");
    
    // 示例1：基本Copy类型的行为
    {
      let x = 42;  // i32类型实现了Copy
      let y = x;   // x的值被复制到y
      
      println!("  复制后两个变量独立存在: x={}, y={}", x, y);  // x仍然可用
      
      {
        let z = x;  // 在内部作用域再次复制
        println!("  内部作用域中的副本: z={}", z);
      } // z的副本在这里结束存在
      
      // x和y的副本继续存在
      println!("  内部作用域结束后: x={}, y={}", x, y);
    } // x和y的副本在这里结束存在
    
    // 示例2：对比非Copy类型
    {
      // 非Copy类型（String）
      let s1 = String::from("非Copy类型");
      let s2 = s1;  // 所有权转移，而不是复制
      
      // println!("  s1: {}", s1);  // 错误：s1的值已被移动
      println!("  s2接管所有权: {}", s2);
      
      // Copy类型（i32）作为对比
      let n1 = 100;
      let n2 = n1;  // 值被复制
      
      println!("  赋值后两个数字都可用: n1={}, n2={}", n1, n2);
    }
    
    // 示例3：在函数调用中的行为
    {
      let num = 50;
    //   takes_copy(num);  // num的值被复制
      println!("  函数调用后原值仍可用: {}", num);  // num仍然可用
      
      let text = String::from("会被移动的文本");
      // takes_ownership(text);  // text的所有权被转移
      // println!("  {}", text);  // 错误：text的值已被移动
      
      // 使用引用避免所有权转移
    //   takes_reference(&text);
      println!("  使用引用避免所有权转移: {}", text);  // text仍然可用
    }
    
    println!("\n结论:");
    println!("  - 对于Copy类型，值的存在时间确实等于变量作用域");
    println!("  - 这是因为每个变量都持有自己的独立副本");
    println!("  - 不存在所有权转移的概念，只有值复制");
    println!("  - 典型的Copy类型包括：整数、浮点数、布尔值、字符、固定大小数组等");
  }
}

// 接受T: 'static约束的函数
fn static_bound<T: Display + 'static>(t: &T) {
  println!("  接收满足'static约束的值: {}", t);
}

// 新增: 专门解释T: 'static约束的函数
fn explain_static_bound<T: Display + 'static>(value: &T) {
  println!("  接收类型满足T: 'static约束的值: {} (类型: {})", value, std::any::type_name::<T>());
}

// 新增：接受&'static T的函数，展示区别
fn static_ref_only(t: &'static str) {
  println!("  接收静态引用: {}", t);
}

// // 展示生命周期参数实际含义的函数
// fn demonstrate_lifetime<'a>(s: &'a String) -> &'a str {
//   // 'a表示：返回的引用的生命周期不会超过参数s的生命周期
//   &s[0..5]
// }

// 这个函数尝试返回局部变量的引用，编译会失败
// fn return_inner_ref<'a>(x: &i32) -> &'a i32 {
//   let local = 10;
//   &local // 编译错误：返回对函数局部变量的引用
// }

// 接收引用的函数
fn print_ref(r: &i32) {
  println!("  - 函数收到的引用: {}", *r);
}

// 返回引用的函数
fn return_same_ref<'a>(r: &'a i32) -> &'a i32 {
  r // 返回接收到的同一个引用
}

// 演示生命周期注解的函数
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate() {
    if (item == b' ') {
      return &s[0..i];
    }
  }
  
  &s[..]
}