#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    student: bool
}

fn main() {
    let mut a = User {
        name: String::from("mike"),
        age: 20,
        student: false,
    };
    let User {
        ref mut name,    // 这里加了一个ref
        age,
        student,
    } = a;

    name.push_str("!");
    
    println!("{}", name);
    println!("{}", age);
    println!("{}", student);
    println!("{:?}", a);
}
// 输出 
// mike!
// 20
// false
// User { name: "mike!", age: 20, student: false }