# Rust Notes

## 一、基础语法

- Rust 编辑器
- Rust 创建工程
- Rust 程序的编译与执行
- Rust 代码规范

### 1. 下载安装

MacOS / Linux / WSL：

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> Windows 原生平台安装请自查，首先确定版本位数：，在安装过程中，它会询问你是想安装 GNU 工具链的版本还是 MSVC 工具链的版本。安装 GNU 工具链版本的话，不需要额外安装其他软件包。而安装 MSVC 工具链的话，需要先安装微软的 Visual Studio 依赖。若暂时不想在本地安装，可使用网页版：[Rust 试验场](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

### 2. 编辑器 / IDE

在 VS Code 中需要安装 rust-analyzer 插件才会有自动提示等功能：

![img](./imgs/b29a9f442ba5f55e1041e1b8b527a7d7.png)

其他一些常用的 Rust 代码编辑器还有 VIM、NeoVIM、IDEA、Clion 等。JetBrains 最近推出了 Rust 专用的 IDE：RustRover。

Rust 编译器套件安装好之后，会提供一些工具：

![image-20240228114934687](./imgs/image-20240228114934687.png)

![image-20240228115053947](./imgs/image-20240228115053947.png)

### 3. 创建工程

使用 Cargo 命令行工具来创建一个 Rust 工程 helloworld。

打开终端，输入：

```shell
cargo new --bin helloworld 
```

显示：

```shell
Created binary (application) `helloworld` package
```

新工程目录组织结构：

```shell
helloworld
    ├── Cargo.toml
    └── src
        └── main.rs
```

第一层是一个 src 目录和一个 Cargo.toml 配置文件。src 是放置源代码的地方，而 Cargo.toml 是这个工程的配置文件：

```
[package]
name = "helloworld"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

Cargo.toml 中包含 package 等基本信息，里面有名字、版本和采用的 Rust 版次。Rust 3 年发行一个版次，目前有 2015、2018 和 2021 版次，最新的是 2021 版次。可以执行`rustc -V`来查看Rust 版本：

```shell
rustc 1.76.0 (07dca489a 2024-02-04)
```

### 4. Hello, World

```rust
fn main() {
    println!("Hello, world!");
}
```

这段代码会使终端输出`"Hello, world!"`的字符串，使用`cargo build`编译：

```shell
$ cargo build
   Compiling helloworld v0.1.0 (/home/mike/works/classes/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 1.57s
```

使用`cargo run`命令可以直接运行程序：

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/helloworld`
Hello, world!
```

### 5. 基础语法

Rust 基础语法主要包括基础类型、复合类型、控制流、函数与模块几个方面。

#### 基础类型

##### 赋值语句

Rust使用`let`关键字定义变量以及初始化：

```rust
fn main() {
  let a: u32 = 1;
}
```

Rust 中类型写在变量名的后面，例子里变量 a 的类型是 u32, 也就是无符号 32 位整数，赋值为 1，Rust 保证定义的变量在第一次使用之前一定被初始化过。

##### 数字类型

与一些动态语言不同，Rust 中的数字类型是区分位数的。

###### 整数

![image-20240228132107640](./imgs/image-20240228132107640.png)

其中，isize 和 usize 的位数与具体 CPU 架构位数有关。CPU 是 64 位的，它们就是 64 位的，CPU 是 32 位的，它们就是 32 位的。这些整数类型可以在写字面量的时候作为后缀跟在后面，来直接指定值的类型，比如`let a = 10u32`就定义了一个变量a，初始化成无符号32位整型，值为10。

###### 整数字面量的辅助写法

```
十进制字面量 98_222，使用下划线按三位数字一组隔开
十六进制字面量 0xff，使用0x开头
8进制字面量 0o77，使用0o（小写字母o）开头
二进制字面量 0b1111_0000，使用0b开头，按4位数字一组隔开
字符的字节表示 b'A'，对一个ASCII字符，在其前面加b前缀，直接得到此字符的ASCII码值
```

各种形式的辅助写法是为了提高程序员写整数字面量的效率，同时更清晰，更不容易犯错。

###### 浮点数

浮点数有两种类型：f32 和 f64，分别代表 32 位浮点数类型和 64 位浮点数类型。它们也可以跟在字面量的后面，用来指定浮点数值的类型，比如 let a = 10.0f32; 就定义了一个变量 a，初始化成 32 位浮点数类型，值为 10.0。

###### 布尔类型

```rust
let a = true;
let b: bool = false;
```

###### 字符

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; 
    let heart_eyed_cat = '😻';
    let t = '中';
}
```

Rust 的 char 类型存的是[Unicode散列值](https://unicode.org/glossary/#unicode_scalar_value)。可以表示各种符号，另外在 Rust 中，char 类型在内存中总是占用4个字节大小，这一点与C语言和其他语言中的 char 类型不一样。

**注意，Rust 中的 String 不能通过下标去访问。**

```rust
let hello = String::from("你好");
let a = hello[0];  // 你可能想把“你”字取出来，但实际上这样是错误的
```

> String 存储的 Unicode 序列的 UTF8 编码，而 UTF8 编码是变长编码。这样即使能访问成功，也只能取出一个字符的 UTF8 编码的第一个字节，它很可能是没有意义的。因此 Rust 直接对 String 禁止了这个索引操作。

###### 转义字符

与 C 语言一样，Rust 中转义符号也是反斜杠 \ ，可用来转义各种字符：

```rust
fn main() {
    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);
    
    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);
    
    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);
    
    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);
    
    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);
}
```

除此之外，Rust 还支持通过 \x 输入等值的 ASCII 字符，以及通过 \u{} 输入等值的 Unicode 字符：

```rust
fn main() {
    // 使用 \x 输入等值的ASCII字符（最高7位）
    let byte_escape = "I'm saying hello \x7f";
    println!("{}", byte_escape);
    
    // 使用 \u{} 输入等值的Unicode字符（最高24位）
    let byte_escape = "I'm saying hello \u{0065}";
    println!("{}", byte_escape);
}
```

###### 禁止转义字符

如果不希望字符串被转义，在 Rust 中可以使用` r"" `或` r#""# `把字符串字面量套起来：

```rust
fn main() {
    // 字符串字面量前面加r，表示不转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    
    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    
    // 如果遇到字面量里面有#号的情况，可以在r后面，加任意多的前后配对的#号，
    // 只要能帮助Rust编译器识别就行
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
```

> Rust 中的字符串字面量都支持换行写，但会把换行符包含进去。

###### 字节串

如果字符串字面量中用不到 Unicode 字符，只需要 ASCII 字符集，对于这种情况，Rust 还有一种更紧凑的表示法：字节串。用 b 开头，双引号括起来，比如 b"this is a byte string"。这时候字符串的类型已不是字符串，而是字节的数组 [u8; N]，N 为字节数：

```rust
fn main() {
    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);
    
    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);
}
```

> 字节串很有用，特别是在做系统级编程或网络协议开发的时候，经常会用到。

###### 数组

Rust 中的数组是 array 类型，用于存储同一类型的多个值。数组表示成[T; N]，由中括号括起来，中间用分号隔开，分号前面表示类型，分号后面表示数组长度：

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
}
```

Rust 中的数组是固定长度的，也就是说在编译阶段就能知道它占用的字节数，并且在运行阶段，不能改变它的长度（尺寸）。

Rust 中区分固定尺寸数组和动态数组，之所以做这种区分是因为 Rust 语言在设计时就要求适应不同的场合，要有足够的韧性能在不同的场景中都达到最好的性能。因为固定尺寸的数据类型是可以直接放栈上的，创建和回收都比在堆上动态分配的动态数组性能要好。

**是否能在编译期计算出某个数据类型在运行过程中占用内存空间的大小**，这个指标很重要，Rust 的类型系统就是按这个对类型进行分类的。

###### 动态数组

Rust 中的动态数组类型是 Vec（Vector），也就是向量，中文翻译成动态数组。它用来存储同一类型的多个值，容量可在程序运行的过程中动态地扩大或缩小，因此叫做动态数组：

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

动态数组可以用下标进行索引访问，比如：

```rust
fn main() {
    let s1 = String::from("superman 1");
    let s2 = String::from("superman 2");
    let s3 = String::from("superman 3");
    let s4 = String::from("superman 4");
    
    let v = vec![s1, s2, s3, s4];

    println!("{:?}", v[0]);
}
// 输出 
"superman 1"
```

**如果下标越界，代码可以通过编译，但运行时会出错，并且会导致主线程的崩溃。**

###### 哈希表

哈希表是一种常见的结构，用于存储 Key-Value 映射关系，基本在各种语言中都有内置提供。Rust 中的哈希表类型为 HashMap。对一个 HashMap 结构来说，Key 要求是同一种类型，比如是字符串就统一用字符串，是数字就统一用数字。Value 也是一样，要求是同一种类型。Key 和 Value 的类型不需要相同：

```rust
fn main() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores["Blue"]);
}
```

##### 复合类型

复合类型可以包含多种基础类型，是一种将类型进行有效组织的方式，提供了一级一级搭建更高层类型的能力。Rust 中的复合类型包括元组、结构体、枚举等。

###### 元组

元组是一个固定（元素）长度的列表，每个元素类型可以不一样。用小括号括起来，元素之间用逗号隔开。例如：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

元组元素的访问：

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    
    // 元组使用.运算符访问其元素，下标从0开始，注意语法
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

与数组的相同点是，它们都是固定元素个数的，在运行时不可伸缩。与数组的不同点是，元组的每个元素的类型可以不一样。元组在 Rust 中很有用，因为它可以用于函数的**返回值**，相当于把多个想返回的值捆绑在一起，**一次性返回**。

当没有任何元素的时候，元组退化成 ()，就叫做 unit 类型，是 Rust 中一个非常重要的基础类型和值，unit 类型唯一的值实例就是 ()，与其类型本身的表示相同。比如一个函数没有返回值的时候，它实际默认返回的是这个 unit 值。

###### 结构体

结构体也是一种复合类型，它由若干字段组成，每个字段的类型可以不一样，Rust 中使用 struct 关键字来定义结构体。比如下面的代码就定义了一个 User 类型：

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    age: u64,
}
```

结构体类型的实例化：

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        age: 1,
    };
}
```

###### 枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

枚举类型里面的选项叫做此枚举的变体（variants）。

变体是其所属枚举类型的一部分，枚举使用变体进行枚举类型的实例化，比如：

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

可以看到，枚举类型也是一种复合类型。但是与结构体不同，结构体类型是里面的所有字段（所有类型）同时起作用，来产生一个具体的实例，而枚举类型是其中的一个变体起作用，来产生一个具体实例。学术上，通常把枚举叫作**和类型**（sum type），把结构体叫作**积类型**（product type）。

**枚举类型是 Rust 中最强大的复合类型**，在后面的课程中我们会看到，枚举就像一个载体，可以携带任何类型。

##### 控制流

###### 分支语句

Rust 中使用 if else 来构造分支：

```rust
fn main() {
    let number = 6;
    // 判断数字number能被4，3，2中的哪一个数字整除
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

与其他 C 系语言不同，Rust 中 if 后面的条件表达式不推荐用（）包裹起来，因为 Rust 设计者认为那个是不必要的，是多余的语法噪音。

另外，`if else`支持表达式返回：

```rust
fn main() {
    let x = 1;
    // 在这里，if else 返回了值
    let y = if x == 0 {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        100
    } else {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        101
    };
    println!("y is {}", y);
}
```

像上面这样的代码，其实已经实现了类似于 C 语言中的三目运算符这样的设计，在 Rust 中，不需要额外提供那样的特殊语法。

###### 循环语句

Rust 中有三种循环语句，分别是 loop、while、for。

- loop 用于无条件（无限）循环：

  ```rust
  fn main() {
      let mut counter = 0;
      
      // 这里，接收从循环体中返回的值，对result进行初始化
      let result = loop {
          counter += 1;
          if counter == 10 {
              // 使用break跳出循环，同时带一个返回值回去
              break counter * 2;
          }
      };
  
      println!("The result is {result}");
  }
  ```

  这种**返回一个值到外面对一个变量初始化的方式**，是 Rust 中的习惯用法，这能让代码更紧凑。

- while 循环为条件判断循环。当后面的条件为真的时候，执行循环体里面的代码。和前面的 if 语句一样，Rust 中的 while 后面的条件表达式不推荐用（）包裹起来。比如：

  ```rust
  fn main() {
      let mut number = 3;
  
      while number != 0 {
          println!("{number}!");
          number -= 1;
      }
  
      println!("LIFTOFF!!!");
  }
  ```

- for 循环在 Rust 中，基本上只用于迭代器（暂时可以想象成对数组，动态数组等）的遍历。Rust 中没有 C 语言那种 for 循环风格的语法支持，因为那被认为是一种不好的设计，以下是对一个数组进行遍历：

  ```rust
  fn main() {
      let a = [10, 20, 30, 40, 50];
  
      for element in a {
          println!("the value is: {element}");
      }
  }
  ```

  上面代码对动态数组 Vec 的遍历阻止了越界的可能性，因此用这种方式访问 Vec 比用下标索引的方式访问更加安全。

  对于循环的场景，Rust 还提供了一个便捷的语法来生成遍历区间：` ..`（两个点）：

  ```rust
  fn main() {
      // 左闭右开区间
      for number in 1..4 {
          println!("{number}");
      }
      println!("--");
      // 左闭右闭区间
      for number in 1..=4 {
          println!("{number}");
      }
      println!("--");
      // 反向
      for number in (1..4).rev() {
          println!("{number}");
      }
  }
  // 输出
  1
  2
  3
  --
  1
  2
  3
  4
  --
  3
  2
  1
  ```

  打印字符：

  ```rust
  fn main() {
  for ch in 'a'..='z' {
          println!("{ch}");
      }
  }
  // 输出：
  a
  b
  c
  d
  e
  f
  g
  h
  i
  j
  k
  l
  m
  n
  o
  p
  q
  r
  s
  t
  u
  v
  w
  x
  y
  z
  ```

##### 函数与模块

###### 函数

函数基本上是所有编程语言的标配，在 Rust 中也不例外，它是一种基本的代码复用方法。在 Rust 中使用 fn 关键字来定义一个函数。比如：

```rust
fn print_a_b(a: i32, b: char) {
    println!("The value of a b is: {a}{b}");
}

fn main() {
    print_a_b(5, 'h');
}
```

函数对于几乎所有语言都非常重要，实际上各种编程语言在实现时，都是以函数作为基本单元来组织栈上的内存分配和回收的，这个基本的内存单元就是所谓的栈帧（frame）。

###### 闭包（Closure）

闭包是另一种风格的函数。它使用两个竖线符号` ||` 定义，而不是用` fn xxx() `来定义：

```rust
fn main() {
    // // 标准的函数定义
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }

    // // 闭包的定义，请注意形式对比
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };

    // // 闭包的定义2，省略了类型标注
    // let add_one_v3 = |x|             { x + 1 };

    // // 闭包的定义3，花括号也省略了
    // let add_one_v4 = |x|              x + 1  ;
    
    let add_one = |x| x + 1; 
    let a_vec: Vec<i32> = vec![1,2,3,4,5];
    let vec2: Vec<_> = a_vec.iter().map(add_one).collect();
    println!("{vec2:?}");
}
```

闭包与函数的一个显著不同就是，**闭包可以捕获函数中的局部变量使用**，而函数不行。比如，下面示例中的闭包 add_v2 捕获了 main 函数中的局部变量 a 来使用，但是函数 add_v1 就没有这个能力：

```rust
fn main() {
    let a = 10u32;             // 局部变量
    
    fn  add_v1   (x: u32) -> u32 { x + a }    // 定义一个内部函数
    let add_v2 = |x: u32| -> u32 { x + a };   // 定义一个闭包
    
    let result1 = add_v1(20);  // 调用函数
    let result2 = add_v2(20);  // 调用闭包
    println!("{}", result2);
}
```

这样会编译错误，并提示错误：

```shell
error[E0434]: can't capture dynamic environment in a fn item
 --> src/main.rs:4:40
  |
4 |     fn  add_v1   (x: u32) -> u32 { x + a }    // 定义一个内部函数
  |                                        ^
  |
  = help: use the `|| { ... }` closure form instead
```

闭包之所以能够省略类型参数等信息，主要是其定义在某个函数体内部，从闭包的内容和上下文环境中能够分析出来那些类型信息。

###### 模块

代码量多了后，分成不同的文件模块书写是非常自然的事情，这个需求需要从编程语言层级去做一定的支持才行，Rust 也提供了相应的方案。

分文件和目录组织代码理解起来其实很简单，主要的知识点在于目录的组织结构上。比如下面示例：

```shell
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden              // 子目录
    │   └── vegetables.rs
    ├── garden.rs           // 与子目录同名的.rs文件，表示这个模块的入口
    └── main.rs
```

第五行代码到第七行代码组成 garden 模块，在 garden.rs 中，使用 mod vegetables; 导入 vegetables 子模块。在 main.rs 中，用同样的方式导入 garden 模块：

```shell
mod garden;
```

整个代码结构就这样一层一层地组织起来了，另一种文件的组织形式来自 2015 版：

```shell
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden          // 子目录
    │   └── mod.rs      // 子目录中有一个固定文件名 mod.rs，表示这个模块的入口
    │   └── vegetables.rs
    └── main.rs
```

同上，由第五行到第七行代码组成 garden 模块，在 main.rs 中导入它使用。

##### 测试

Rust 语言中自带单元测试和集成测试方案，假设在 src/lib.rs 或 src/main.rs 下有一段代码：

```shell
fn foo() -> u32 { 10u32 }

#[cfg(test)]            // 这里配置测试模块
mod tests {
    use crate::foo;
     
    #[test]             // 具体的单元测试用例
    fn it_works() {
        let result = foo();           // 调用被测试的函数或功能
        assert_eq!(result, 10u32);    // 断言
    }
}
```

在项目目录下运行 cargo test，会输出类似如下结果：

```shell
running 1 test
test tests::it_works ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

##### 配置文件 Cargo.toml

`Cargo.toml`是 Rust 语言包和依赖管理器 Cargo 的配置文件，由官方定义约定。

写 Rust 代码基本都会按照这种约定来使用它，对所在工程进行配置。npm 依赖黑洞是指 Node.js 的包依赖太多太琐碎了，Rust 也类似，但Cargo 工具已经帮我们搞定了包依赖相关方方面面的麻烦事。为了应对这种复杂性，Cargo 工具的提供了非常多的特性，配置起来也相对比较复杂。相关文档：[Cargo配置属性](https://doc.rust-lang.org/cargo/)
