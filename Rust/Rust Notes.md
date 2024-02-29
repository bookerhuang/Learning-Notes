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

#### 控制流

##### 分支语句

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

##### 循环语句

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

#### 函数与模块

##### 函数

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

##### 闭包（Closure）

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

##### 模块

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

#### 测试

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

#### 配置文件 Cargo.toml

`Cargo.toml`是 Rust 语言包和依赖管理器 Cargo 的配置文件，由官方定义约定。

写 Rust 代码基本都会按照这种约定来使用它，对所在工程进行配置。npm 依赖黑洞是指 Node.js 的包依赖太多太琐碎了，Rust 也类似，为了应对这种复杂性，Cargo 工具的提供了非常多的特性，配置起来也相对比较复杂。相关文档：[Cargo配置属性](https://doc.rust-lang.org/cargo/)

---

## 二、所有权

Rust 语言里的值有两大类：一类是固定内存长度（简称固定尺寸）的值，比如 i32、u32、由固定尺寸的类型组成的结构体等；另一类是不固定内存长度（简称非固定尺寸）的值，比如字符串 String。这两种值的本质特征完全不一样。而**怎么处理这两种值的差异，往往是语言设计的差异性所在**。

C、C++、Java 这些语言就明确定义了数字类型会占用内存中的几个字节，比如 8 位，也就是一个字节，16 位，也就是两个字节。而 JavaScript 这种语言，就完全屏蔽了底层的细节，统一用一个 Number 表示数字。Python 则给出了 int 整数、float 浮点、complex 复数三种数字类型。

Rust 语言因为在设计时就定位为一门通用的编程语言（对标 C++），它的应用范围很广，从最底层的嵌入式开发、OS 开发，到最上层的 Web 应用开发，它都要兼顾。所以它的数字类型不可避免地就得暴露出具体的字节数，于是就有了 i8、i16、i32、i64 等类型。

一种类型如果具有固定尺寸，那么它就能够在编译期做更多的分析。实际上固定尺寸类型也可以用来管理非固定尺寸类型。具体来说，Rust 中的非固定尺寸类型就是靠指针或引用来指向，而指针或引用本身就是一种固定尺寸的类型。

### 1. 栈与堆

现代计算机会把内存划分为很多个区。比如，二进制代码的存放区、静态数据的存放区、栈、堆等。

**栈上的操作比堆高效**，因为栈上内存的分配和回收只需移动栈顶指针就行了。这就决定了分配和回收时都必须精确计算这个指针的增减量，因此栈上一般放固定尺寸的值。另一方面，栈的容量也是非常有限的，因此也**不适合放尺寸太大的值**，比如一个有 1000 万个元素的数组。

那么非固定尺寸的值怎么处理呢？在计算机体系架构里面，专门在内存中拿出一大块区域来存放这类值，这个区域就叫堆。

在一般的程序语言设计中，栈空间都会与函数关联起来。每一个函数的调用，都会对应一个帧，也叫做 frame 栈帧，就像图片栈空间里的方块 main、fn1、fn2 等。

一个函数被调用，就会分配一个新的帧，函数调用结束后，这个帧就会被自动释放掉。因此**栈帧是一个运行时的事物**。函数中的参数、局部变量之类的资源，都会放在这个帧里面，比如图里 fn2 中的局部变量 a，这个帧释放时，这些局部变量就会被一起回收掉。

![image-20240228165656584](./imgs/image-20240228165656584.png)

函数的调用会形成层级关系，因此栈空间中的帧可能会同时存在很多个，并且在它们之间也对应地形成层级关系。如上图所示，可能的函数调用关系为，main 函数中调用了函数 fn1，fn1 中调用了函数 fn2，fn2 中调用了函数 fn3，fn3 中调用了函数 fn4，fn4 调用了更深层次的其他函数。这样的话，在程序执行的某个时刻，main 函数、fn1、fn2、fn3、fn4 等对应的帧副本就同时存在于栈中了。

图中右边堆空间里面的一些小圈表示堆空间中资源，也就是被分配的内存。从图中可以看到，栈空间中函数帧的局部变量是可以引用这些堆上资源的。一个栈帧中的多个局部变量可以指向堆中的多个资源，如 fn3 中的 b 指向资源 A，c 指向资源 B；同时存在的多个栈帧中的局部变量还可以指向堆上的同一个资源，如图中的 a 和 b，c 和 d；堆上的资源也可以存在引用关系，如图中的 D 和 E。

如果一个资源没有被任何一个栈帧中的变量引用或间接引用，如图中的 C，那么它实际是一个被泄漏的资源，也就是发生了内存泄漏。被泄漏的资源会一直伴随程序的运行，直到程序自身的进程被停止时，才会一起被 OS 回收掉。

而计算机程序内存管理的复杂性，主要就在于**堆内存的管理比较复杂——既要高效，又要安全**。

### 2. 变量与可变性

在 Rust 中定义一个变量，使用`let variable = value`这种语法。比如`let x = 10u32`就定义了变量 x，10u32 是一个值，它被绑定到这个变量上，另外默认变量是不可变的。

Rust 默认这样做是为了减少一些很低级的 Bug，假如默认可以改的话，如果在一个代码量很大而且离定义变量很远的某个分支语句里面修改了这个变量的值，然后在后面某个函数调用里面又用到了它，结果导致程序行为与期望不符，所以Rust 禁用了这种方式。

但是下面这样做是可以的：

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;    // 注意这里，重新使用了 let 来定义新变量
    println!("The value of x is: {x}");
}
```

这种方式在 Rust 中叫做变量的 Shadowing。意思很好理解，就是定义了一个新的变量名，只不过这个变量名和老的相同。原来那个变量就被遮盖起来了，访问不到了，变量的 Shadow 甚至支持新的变量的类型和原来的不一样。

如果需要修改变量的值，需要在变量名前面加一个 mut ：

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
// 输出 
// The value of x is: 5
// The value of x is: 6
```

注意，值的改变只能在同一种类型中变化，在变量 x 定义的时候，就已经确定了变量 x 的类型为数字了，如果将其改为字符串，则会报错。

Rust 中变量的可变性是一种潜力，只要它有可能会变化，那么就可以称之为变量。而 Rust 给这种潜力加了一道开关，**当想让这个变量的可变性暴露出来的时候，应该在变量名前面明确地加个 mut 修饰符**。

可以看到，变量名加了 mut，多打了 4 个字符，这实际是在代码中留下了一种足迹。也就是说给了程序员一个信息，当自己或别的程序员在读到这个变量的定义时，他会知道，后面一定会修改这个变量，因为如果后面没修改它，Rust 编译器会提示把这个 mut 去掉。另外这种设计还有一个好处，那就是减少滥用概率。

值是有类型的，比如 10u32，它就是一个 u32 类型的数字。一旦一个变量绑定了一个值，或者说一个值被绑定到了一个变量上，那么这个变量就被指定为这种值的类型。比如`let x = 10u32` 编译器会自动推导出变量 x 的类型为 u32，完整的写法就是`let x: u32 = 10u32`。

此外还有一种方式，就是直接先指定变量的类型，然后把一个值绑定上去，比如`let x: u32 = 10`。这种方式更好，它能说明在写这句代码的时候就已经对它做了一个架构上的规划和设计，这种形式能帮助在编译阶段阻止一些错误。

比如输入下面这段代码：

```rust
fn main() {
    let a: u8 = 323232;
    println!("{a}");
}
```

编译器就会报错，**指出 u8 类型装不下这么大的一个数字**。

**所有的变量都应该具有明确的类型是 Rust 程序的基本设计**，当然其他语言中也有类型，不同语言对类型重视的程度不一样，这取决于语言自身的设计定位。

### 3. 所有权

一个简单的例子：

```rust
fn main() {
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");
}

// 打印：
// 10
// 10
```

如果赋值为字符串，输出将会发生错误：

```rust
fn main() {
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    println!("{s1}");
    println!("{s2}");
}
```

```shell
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `s1`
// 借用了移动后的值 `s1`
 --> src/main.rs:4:15
  |
2 |     let s1 = String::from("I am a superman.");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 移动发生了，因为 `s1` 的类型是 `String`，而这种类型并没有实现 `Copy` trait."。
3 |     let s2 = s1;
  |              -- value moved here
// 在这里值移动了。
4 |     println!("{s1}");
  |               ^^^^ value borrowed here after move
// 值在被移动后在这里被借用
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
// 如果性能成本可以接受的话，考虑克隆这个值
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

按修改建议进行修改：

```rust
fn main() {
    let s1 = String::from("I am a superman.");
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}");
}
```

可以输出预期的结果了：

```shell
I am a superman.
I am a superman.
```

在 Rust 中，字符串的行为与 u32 这种数字类型不一样。u32 这种类型是固定尺寸类型，而 String 是非固定尺寸类型。

一般来说，对于固定尺寸类型，会默认放在栈上；而非固定尺寸类型，会默认创建在堆上，成为堆上的一个资源，然后在栈上用一个局部变量来指向它，如代码中的 s1。

在将一个变量赋值给另一个变量的时候，不同语言对底层细节的处理不一样。Java 语言对于 int 这类固定尺寸类型，在复制给另一个变量的时候，会直接复制它的值。在面对 Object 这种复杂对象的时候，默认只会复制这个 Object 的引用给另一个变量。这个引用的值（内存地址）就存在栈上的局部变量里面，因为如果那个 Object 占用的内存很大，每一次重新赋值，就把那个对象重新拷贝一次，也就是完全克隆，是非常低效的。所以在用 Java 编程时，它实际上是隐藏了对象引用的复制这个细节。

回到 Rust，对于 u32 这种固定尺寸类型来说，Rust 与 Java 也是同样的处理，直接在栈上进行内容的拷贝。而对于字符串这种动态长度的类型来说，在变量的再赋值上，Rust 除了拷贝字符串的引用外，实际还做了更多事情，一个修改后的例子：

```rust
fn main() {
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    //println!("{s1}");
    println!("{s2}");
}
// 能正常打印
```

对比之后，可以发现` let s2 = s1; `语句执行后，s2 可以使用，而 s1 不能再使用了。也就是说，在 Rust 里面，s1 把内容`复制`给 s2 后，s2 可用，s1 不能用了。

从代码层面我们也可以说，s1 把值（资源）`移动`给了 s2。既然是移动了，那原来的变量就没有那个值了。请仔细体会这里与 Java 的不同之处。Java 默认做了引用的拷贝，并且新旧两个变量同时指向原来那个对象。**而 Rust 不一样，Rust 虽然也是把字符串的引用由 s1 拷贝到了 s2，但是只保留了最新的 s2 到字符串的指向，同时却把 s1 到字符串的指向给`抹去`了**。s1 之后都处于一种“不可用”的状态，直到函数结束。这就是 Rust 编译器做的那个`更多`的部分。

下面的图示展示了这两种行为上的差异，这有助于**内存安全**：

![image-20240229102755736](./imgs/image-20240229102755736.png)

![image-20240229102828203](./imgs/image-20240229102828203.png)

Rust 明确了所有权的概念，值也可以叫资源，所有权就是拥有资源的权利。一个变量拥有一个资源的所有权，那它就要负责那个资源的回收、释放。**Rust 基于所有权定义出发，推导出了整个世界**。

#### 所有权的定义

**所有权的基础是三条定义：**

1. **Rust中，每一个值都有一个所有者。**
2. **任何一个时刻，一个值只有一个所有者。**
3. **当所有者所在作用域（scope）结束的时候，其管理的值会被一起释放掉。**

这三条规则涉及两个概念：**所有者和作用域**。

所谓所有者，在代码里就用变量表示。而变量的作用域，就是变量有效（valid）的那个代码区间。

在 Rust 中，一个所有权型变量的作用域，简单来说就是它定义时所在的那个最里层的花括号括起的部分，从变量创建时开始，到花括号结束的地方。

比如：

```rust
fn main() {    
    let s = String::from("hello");
    // do stuff with s
}  // 变量s的作用域到这里结束

fn main() {    
    let a = 1u32;
    {
        let s = String::from("hello"); 
    }  // 变量s的作用域到这里结束
    // xxxx
    
}  // 变量a的作用域到这里结束
```

变量在其作用域内是有效的，离开作用域就无效了。

```rust
fn main() {
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");
}
```

在这个例子中，a 具有对值 10u32 的所有权。执行 let b = a 的时候，把值 10u32 复制了一份，b 具有对这个新的 10u32 值的所有权。当 main 函数结束的时候，a、b 两个变量就离开了作用域，其对应的两个 10u32，就都被回收了。这里是栈帧结束，栈帧内存被回收，局部变量位于栈帧中，所以它们所占用的内存就被回收了。

再来看一个字符串的例子：

```rust
fn main() {
    let s1 = String::from("I am a superman.");
    println!("{s1}");
}
```

局部变量 s1 拥有这个字符串的所有权，s1 的作用域从定义到开始，直到花括号结束。s1（栈帧上的局部变量）离开作用域时，变量 s1 上绑定的内存资源（字符串）就被回收掉了。注意，这里发生的事情是，栈帧中的局部变量离开作用域了，顺带要求堆内存中的字符串资源被回收。之所以能够做到这一点，是因为这个堆中的字符串资源被栈帧中的局部变量所指向了的。

而从 Rust 的语法层面看起来，就是变量 s1 对那个字符串拥有所有权。所以 s1 离开作用域的时候，那个资源就一起被回收了。这看起来好像是一个自动的过程，并没有像 C 语言中那样，需要手动调用 free() 函数去释放堆中的字符串资源。

这种**堆内存资源随着关联的栈上局部变量一起被回收的内存管理特性**，叫作 **RAII**（Resource Acquisition Is Initialization）。它实际不是 Rust 的原创，而是 C++ 创造的。对比一下 C 中的 malloc() 分配堆内存的方式，在分配堆内存后，C 语言里面必须由程序员手动在后续的代码中使用 free() 来释放堆内存中的资源。而有了 RAII 特性后，不需要手动写 free()，因此可以认为 RAII 内存管理方式是一个相当大的进步。

有了所有权的知识后，再回过头来分析上面那个例子：

```rust
fn main() {
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    //println!("{s1}");
    println!("{s2}");
}
```

变量 s1 持有这个字符串的所有权，s1 对字符串的所有权从第 2 行定义时开始，到 let s2 = s1 执行后结束。这一行执行后，s2 持有那个字符串的所有权。而此时 s1 处于一种不可用的状态，或者叫无效状态（invalid），这个状态是由 Rust 编译器在编译阶段管理的，只需要从所有权模型去理解它，而不需要操心细节。

然后直到花括号结束，s2 及 s2 所拥有的字符串内存，就被回收掉了，s1 所对应的那个局部变量的内存空间也一并被回收了。

**所有权是 Rust 语言的出发点，我们写的任何 Rust 程序，都必须遵循这套规则**。

需要注意的一点是，所有权其实是内存结构之上的更上层概念，并不是说只有在堆中分配的资源才有所有权。实际上，栈上的资源也是有所有权的。所有权这个概念实际上屏蔽了底层内存结构的细节，让我们可以站在一个新的层次上更有效地对问题进行建模。

#### 使用所有权书写函数

基于所有权规则，函数的写法也会有所改变：

```rust
fn foo(s: String) {
    println!("{s}");
}

fn main() {
    let s1 = String::from("I am a superman.");
    foo(s1);
}

// 输出：
// I am a superman.
```

若想在调用函数结束后，在外面再打印一下 s1 的值：

```rust
fn foo(s: String) {
    println!("{s}");
}

fn main() {
    let s1 = String::from("I am a superman.");
    foo(s1);
    println!("{s1}");    // 这里加了一行
}
```

将会报错：

```shell
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:8:16
  |
6 |     let s1 = String::from("I am a superman.");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
7 |     foo(s1);
  |         -- value moved here
8 |     println!("{s1}");
  |                ^^ value borrowed here after move
  |
note: consider changing this parameter type in function `foo` to borrow instead if owning the value isn't necessary
 --> src/main.rs:1:11
  |
1 | fn foo(s: String) {
  |    ---    ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
7 |     foo(s1.clone());
  |           ++++++++
```

这个例子代码的提示与前面差不多，就是说 s1 所有权已经被移动进函数里面了，不能在移动后再使用了。

这个例子在其他语言中，一般是不会有问题的。foo 函数也许会修改字符串的值，在外面重新打印的时候，会打印出新的值。但是，这其实是一种相当隐晦的设计模式，可能会造成一些错误，而 Rust 阻止了这种模式。

注意提示中的这一行：

```shell
1 | fn foo(s: String) {
  |    ---    ^^^^^^ this parameter takes ownership of the value
```

**函数的参数 s 获取了这个值的所有权**，函数参数是这个函数的一个局部变量，它在这个函数栈帧结束的时候会被回收，因此这个字符串在这个函数调用结束后，就已经被回收了，这就是无法再打印这个字符串的原因。

同样再看一个上面例子的变形：

```rust
fn foo(s: String) {
    println!("{s}");
}

fn main() {
    let s1 = String::from("I am a superman.");
    foo(s1);
    foo(s1);
}
```

简单地想调用两次` foo()`函数都做不到，原因跟前面是一样的。

但是，既然能把所有权移动到函数里面，也当然能把所有权转移出来：

```rust
fn foo(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let s1 = String::from("I am a superman.");
    let s1 = foo(s1);
    println!("{s1}");
}

// 输出结果
// I am a superman.
// I am a superman.
```

#### 移动与复制

u32 这种类型在做变量的再赋值的时候，是做了复制所有权的操作。而 String 这种类型在做变量再赋值的时候，是做了移动所有权的操作。那么，在 Rust 中哪些类型默认是做移动所有权操作，哪些类型默认是做复制所有权操作呢？

复制所有权的操作：

- 整数类型
- 布尔类型
- 浮点数类型
- 字符类型
- 元组类型
- 数组类型
- 不可变引用类型（&）

![image-20240229134940377](./imgs/image-20240229134940377.png)

### 4. 引用

还是这个例子：

```rust
fn foo(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let s1 = String::from("I am a superman.");
    let s1 = foo(s1);
    println!("{s1}");
}
```

这样可能会很麻烦。一是会给程序员造成一些心智负担，还得想着把值传回来再继续使用。如果代码中到处都是所有权传来传去，会让代码显得相当冗余，毕竟很多时候函数返回值是要用作其他类型的返回的。为了解决这个问题，Rust 引入了借用的概念。

`借用`概念也是实际生活中思维的映射，比如有一样东西，别人想用一下，可以借出。那`引用`概念又是什么呢？其实在 Rust 中，借用和引用是一体两面。把东西借给别人用，也就是别人持有了对这个东西的引用。

在 Rust 中，变量前用“&”符号来表示引用，比如` &x`。

其实**引用也是一种值，并且是固定尺寸的值**，一般来说，与机器 CPU 位数一致，比如 64 位或 32 位。因为是值，所以就可以赋给另一个变量。同时它又是固定的而且是小尺寸的值，那其实赋值的时候，就可以直接复制一份这个引用。

来看一下如何使用引用：

```rust
fn main() {
    let a = 10u32;
    let b = &a;        // b是变量a的一级引用
    let c = &&&&&a;    // c是变量a的多级引用
    let d = &b;        // d是变量a的间接引用
    let e = b;         // 引用b再赋值给e
    
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
}
// 输出
// 10
// 10
// 10
// 10
// 10
```

从上面示例中可以看出，Rust 识别了一般情况下的意图，**不会打印出引用的内存地址**，而是打印出了被引用对象的值。示例中的 c 实际是 a 的 5 次引用，但是打印时仍然正确获取到了 a 的值。d 是 a 的间接引用，但是仍然正确获取到了 a 的值。这里可以看出 Rust 与 C 这种纯底层语言的显著区别，Rust 对程序员更友好，它会更多地面向业务。因为人们还是普遍关注最终那个值的部分，而不是中间过程的内存地址。

上面示例中，**b 和 e 都是对 a 的一级引用。由于引用是固定尺寸的值，let e = b 做的就是引用的复制操作，并没有再复制一份 a 的值**。对于字符串也是一样，这些引用都不会导致堆中的字符串资源被复制一份或多份。字符串的所有权仍然在 s1 那里，s2、s3、s4、s5 都是对这个所有权变量的引用。从这里开始，可以将变量按一个新的维度划分为**所有权型变量**和**引用型变量**。

也可以看出，在 Rust 中，一个所有权型变量（如 s1）带有值和类型的信息，一个引用型变量（如 s2、s3、s4、s5）也带有值和类型的信息，不然它没法正确回溯到最终的值。这些信息是 Rust 编译器帮忙维护的。

#### 不可变引用与可变引用

Rust 的变量具有可变性，那么同样的规则，是不是可以施加到引用上来呢？当然可以，这也正是 Rust 语言设计一致性的体现。

实际上默认` &x `指的是不可变引用。而要获取到可变引用，需要使用` &mut `符号，如` &mut x`。

所以：

- 引用分为不可变引用和可变引用
- `&x`是对变量`x`的不可变引用
- `&mut x`是对变量`x`的可变引用

这里 mut 和 x 中间有个空格，是为了避免和 &mutx 混淆。

到目前为止，如果要对一个变量内容进行修改，必须拥有所有权型变量才行。而很多时候，没法拥有那个资源的所有权，比如引用一个别人的库，它没有把所有权类型暴露出来，但是确实又有更新其内部状态的需求。因此需要一个东西，它既是一种引用，又能够修改指向资源的内容，于是就引入了**可变引用**。

前面举的引用的例子，实际只是访问（打印）变量的值，没有修改它们，如果要使用引用修改变量的值：

```rust
fn main() {
    let a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{b}");
}
```

提示：

```shell
error[E0596]: cannot borrow `a` as mutable, as it is not declared as mutable
  --> src/main.rs:19:13
   |
19 |     let b = &mut a;
   |             ^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
18 |     let mut a = 10u32;
   |         +++
```

既然要修改一个变量的值，那么变量名前需要加上`mut`修饰符：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{b}");
}
// 输出 
// 20
```

接下来改动一下例子：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{b}");
    println!("{a}");    // 这里多打印了一行a
}
// 输出 
20
20
```

正确输出了修改后的值，若换一下两个打印的语句：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("{a}");  // 这一句移到前面来
    println!("{b}");
}
```

会报错：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
 --> src/main.rs:6:15
  |
3 |     let b = &mut a;
  |             ------ mutable borrow occurs here
...
6 |     println!("{a}");  // 这一句移到的前面来
  |               ^^^ immutable borrow occurs here
// 提示说这里发生了不可变借用
7 |     println!("{b}");
  |               --- mutable borrow later used here
// 在这后面使用了可变借用
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

- 打印语句` println! `中，不管是传所有权型变量还是引用型变量，都能打印出预期的值。实际上` println! `中默认会对所有权变量做不可变借用操作（对应代码里的第 6 行）。
- 可变引用调用的时机（对应代码里的第 7 行）和不可变引用调用的时机（对应代码里的第 6 行），好像有顺序要求。

另外一个例子：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;      // 在利用b更新了a的值后，c再次借用a
}
```

这个代码是可以顺利编译的。但是加一句打印则会报错：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;       // 在利用b更新了a的值后，c再次借用a
    
    println!("{b}");  // 加了一句打印语句
}
```

提示：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
// 不能将a借用为不可变的，因为它已经被可变借用了
  --> src/main.rs:5:13
  |
3 |     let b = &mut a;
  |             ------ mutable borrow occurs here
// 可变借用发生在这里
4 |     *b = 20;
5 |     let c = &a;
  |             ^^ immutable borrow occurs here
// 不可变借用发生在这里
6 |     
7 |     println!("{b}");  // 加了一句打印语句
  |               --- mutable borrow later used here
// 可变借用在这里使用了
```

试着改一下打印语句：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;
    
    println!("{c}");  // 不打印b了，换成打印c
}
// 输出
// 20
```

尝试一下把变量 c 的定义移到前面一些，结果又不能编译：

```rust
fn main() {
    let mut a = 10u32;
    let c = &a;        // c的定义移到这里来了
    let b = &mut a;
    *b = 20;
  
    println!("{c}");
}
```

提示：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:13
  |
3 |     let c = &a;        // c的定义移到这里来了
  |             -- immutable borrow occurs here
4 |     let b = &mut a;
  |             ^^^^^^ mutable borrow occurs here
...
7 |     println!("{c}");
  |               --- immutable borrow later used here
```

**引用的最后一次调用时机很关键**，一个所有权型变量的作用域是从它定义时开始到花括号结束，而引用型变量的作用域不是这样，**引用型变量的作用域是从它定义起到它最后一次使用时结束**。

比如上面的示例中，所有权型变量 a 的作用域是 2~8 行；不可变引用 c 的作用域只有第 3 行，它定义了，但并没有被使用，所以它的作用域就只有那一行；可变引用 b 的作用域是 4~7 行。同时，还存在一条规则：**一个所有权型变量的可变引用与不可变引用的作用域不能交叠**，也可以说不能同时存在，接下来用这条规则分析前面的示例：

```rust
fn main() {
    let mut a = 10u32;
    let c = &a;        
    let b = &mut a;
    *b = 20;
  
    println!("{c}");
}
```

所有权型变量 a 的作用域是 2~8 行，不可变引用 c 的作用域是 3~7 行，可变引用 b 的作用域是 4~5 行。b 和 c 的作用域交叠了，因此无法编译通过。

再看一个例子：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let d = &mut a;
    
    println!("{d}");      // 打印d
}
// 输出
// 20
```

如果尝试打印 b：

```rust
fn main() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let d = &mut a;
    
    println!("{b}");      // 打印b
}
```

编译不通过，提示：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0499]: cannot borrow `a` as mutable more than once at a time
// 在一个时刻不能把`a`以可变借用形式借用超过一次
 --> src/main.rs:5:13
  |
3 |     let b = &mut a;
  |             ------ first mutable borrow occurs here
4 |     *b = 20;
5 |     let d = &mut a;
  |             ^^^^^^ second mutable borrow occurs here
6 |     
7 |     println!("{b}");
  |               --- first borrow later used here
```

编译器：`在一个时刻不能把 a 以可变借用形式借用超过一次`，可以发现确实这两个可变借用的作用域交叠了，b 的作用域是 3～7 行，d 的作用域是第 5 行，所以**同一个所有权型变量的可变借用之间的作用域也不能交叠**。

继续来一个例子：

```rust
fn main() {
    let mut a = 10u32;
    let r1 = &a;
    a = 20;
    
    println!("{r1}");
}
```

编译报错：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0506]: cannot assign to `a` because it is borrowed
// 不能给a赋值，因为它被借用了
 --> src/main.rs:4:5
  |
3 |     let r1 = &a;
  |              -- `a` is borrowed here
4 |     a = 20;
  |     ^^^^^^ `a` is assigned to here but it was already borrowed
5 |     
6 |     println!("{r1}");
  |               ---- borrow later used here
```

提示在有借用的情况下，不能对所有权变量进行更改值的操作（写操作）。

有可变借用存在的情况下也一样：

```rust
fn main() {
    let mut a = 10u32;
    let r1 = &mut a;
    a = 20;
    
    println!("{r1}");
}
```

编译报错：

```shell
   Compiling playground v0.0.1 (/playground)
error[E0506]: cannot assign to `a` because it is borrowed
 --> src/main.rs:4:5
  |
3 |     let r1 = &mut a;
  |              ------ `a` is borrowed here
4 |     a = 20;
  |     ^^^^^^ `a` is assigned to here but it was already borrowed
5 |     
6 |     println!("{r1}");
  |               ---- borrow later used here
```

提示在有借用的情况下，不能对所有权变量进行更改值的操作（写操作）。

##### 阶段性总结

1. **所有权型变量**的作用域是从它定义时开始到**所属那层花括号结束**。
2. **引用型变量**的作用域是从它定义起到它**最后一次使用时结束**。
3. **引用（不可变引用和可变引用）型变量**的作用域**不会长于所有权变量的作用域**。不然就会出现悬锤引用，这是典型的内存安全问题。
4. 一个**所有权型变量**的**不可变引用可以同时存在多个**，可以复制多份。
5. 一个**所有权型变量**的**可变引用与不可变引用的作用域不能交叠**，也可以说不能同时存在。
6. 某个时刻对某个**所有权型变量只能存在一个可变引用**，不能有超过一个可变借用同时存在，也可以说，对同一个所有权型变量的可变借用之间的作用域不能交叠。
7. **在有借用存在的情况下，不能通过原所有权型变量对值进行更新**。当借用完成后（借用的作用域结束后），物归原主，又可以使用所有权型变量对值做更新操作了。

下面我们再来试试可变引用能否被复制：

```rust
fn main() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    
    println!("{r1}")
}
```

出错了，提示：

```shell
error[E0382]: borrow of moved value: `r1`
 --> src/main.rs:6:16
  |
3 |     let r1 = &mut a;
  |         -- move occurs because `r1` has type `&mut u32`, which does not implement the `Copy` trait
4 |     let r2 = r1;
  |              -- value moved here
5 |     
6 |     println!("{r1}")
  |                ^^ value borrowed here after move
```

r1 的值移动给了 r2，因此 r1 不能再被使用了，修改一下：

```rust
fn main() {
    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    
    println!("{r2}");    // 打印r2
}
// 输出
// 10
```

从这个例子可以看出，可变引用的再赋值，会执行移动操作。赋值后，原来的那个可变引用变量就不能用了。这有点类似于所有权的转移，因此**一个所有权型变量的可变引用也具有所有权特征**，它可以被理解为那个所有权变量的独家代理，具有**排它性**。

#### 多级引用

面这段代码展示了 mut 修饰符，&mut 和 & 同时出现的情况：

```rust
fn main() {
    let mut a1 = 10u32;
    let mut a2 = 15u32;

    let mut b = &mut a1;
    b = &mut a2;

    let mut c = &a1;
    c = &a2;
}
```

再来看一个多级可变引用的例子：

```rust
fn main() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    let c = &mut b;
    **c = 30;          // 多级解引用操作
    
    println!("{c}");
}
// 输出 
// 30
```

再来看一个例子：

```rust
fn main() {
    let mut a1 = 10u32;
    let b = &mut a1;
    let mut c = &b;
    let d = &mut c;
    
    ***d = 30;
    
    println!("{d}");
}
```

报错：

```shell
error[E0594]: cannot assign to `***d`, which is behind a `&` reference
  --> src/main.rs:21:5
   |
21 |     ***d = 30;
   |     ^^^^^^^^^ cannot assign

For more information about this error, try `rustc --explain E0594`.
```

提示：不能这样更新目标的值，因为目标躲在一个 & 引用后面。

##### 阶段性总结

1. 对于多级可变引用，要利用可变引用去修改目标资源的值的时候，需要做正确的多级解引用操作，比如例子中的 `**c`，做了两级解引用。
2. **只有全是多级可变引用的情况下，才能修改到目标资源的值**。
3. 对于多级引用（包含可变和不可变），打印语句中，可以自动为我们解引用正确的层数，直到访问到目标资源的值，这很符合人的直觉和业务的需求。

#### 用引用改进函数的定义

有了引用这个设施，可以改进前面将字符串所有权传进函数，然后又传出来的例子。

第一个例子是将字符串的不可变引用传进函数参数：

```rust
fn foo(s: &String) {
    println!("in fn foo: {s}");
}

fn main() {
    let s1 = String::from("I am a superman.");
    foo(&s1);    // 注意这里传的是字符串的引用 &s1
    println!("{s1}");    // 这里可以打印s1的值了
}
```

然后试试将字符串的可变引用传进函数，并修改字符串的内容：

```rust
fn foo(s: &mut String) {
    s.push_str(" You are batman.");
}

fn main() {
    let mut s1 = String::from("I am a superman.");
    println!("{s1}");
    foo(&mut s1);    // 注意这里传的是字符串的可变引用 &mut s1
    println!("{s1}");
}
```

输出：

```shell
I am a superman.
I am a superman. You are batman.
```

从代码中可以看到，这里 Rust 的代码` &s1 `和` &mut s1 `留下了清晰的足迹。如果一个函数参数接受的是可变引用或所有权参数，那么它里面的逻辑一般都会对其引用的资源进行修改。如果一个函数参数只接受不可变引用，那么它里面的逻辑，就一定不会修改被引用的资源。简单的一个参数的签名形式，就**将函数的意图初步划分出来**了，非常利于代码的阅读。

#### 小结

在同一时刻，同一个所有权变量的不可变引用和可变引用两者不能同时存在，不可变引用可以同时存在多个。可变引用具有排它性，只能同时存在一个。

借用结束后，原本的所有权变量会重新恢复可读可写的状态。不可变引用可以被任意复制多份，但是**可变引用不能被复制，只能转移**，这也体现了**可变引用具有一定的所有权特征**。所有权和引用模型是 Rust 语言编写高可靠和高性能代码的基础，理解这些模型有助于优化程序的效率，提高代码质量。

