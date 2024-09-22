#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// 为 Point 实现 Copy 和 Clone traits
impl Copy for Point {} // 这里我们需要显式实现 Copy

impl Clone for Point {
    fn clone(&self) -> Self {
        *self // 因为 Point 的字段都是 Copy 的，所以我们可以直接返回 *self
    }
}

#[derive(Debug, Clone)] // 我们可以派生 Clone，但不能派生 Copy
struct ComplexObject {
    data: Vec<i32>,
}

pub fn run() {
    main();
    main_string();
}

fn main() {
    // 使用 Copy trait
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 被复制到 p2，p1 仍然可用
    println!("p1: {:?}, p2: {:?}", p1, p2);

    // 使用 Clone trait
    let obj1 = ComplexObject { data: vec![1, 2, 3] };
    let obj2 = obj1.clone(); // 显式调用 clone 方法
    println!("obj1: {:?}", obj1); // obj1 仍然可用
    println!("obj2: {:?}", obj2);

    // 尝试不使用 clone
    let obj3 = ComplexObject { data: vec![4, 5, 6] };
    let obj4 = obj3;
    // println!("obj3: {:?}", obj3); // 这行会导致编译错误，因为 obj3 的所有权已经移动到 obj4
    println!("obj4: {:?}", obj4);
}

/*
// String 没有实现copy，但是实现了clone
内存管理：String 在堆上分配内存来存储其内容。如果 String 实现了 Copy，那么每次赋值或传递 String 时都会复制整个堆内存，这可能会导致性能问题。
所有权语义：String 遵循 Rust 的所有权规则，这意味着当你将一个 String 赋值给另一个变量时，所有权会被转移，而不是复制。
资源管理：String 可能包含大量数据，复制可能会非常昂贵。Rust 通过不实现 Copy 来鼓励开发者明确考虑何时需要复制数据。
*/
fn main_string() {
    // 使用 String
    let s1 = String::from("hello");
    let s2 = s1; // 没有实现copy从而直接转移所有权
    // println!("{}", s1);  // 这行会导致编译错误，因为 s1 的所有权已经移动到 s2

    // 对比使用实现了 Copy 的类型
    let x1 = 5;
    let x2 = x1;
    println!("x1 = {}, x2 = {}", x1, x2);  // 这是可以的，因为 i32 实现了 Copy

    // 如果需要复制 String 的内容，可以使用 clone 方法
    let s3 = String::from("world");
    let s4 = s3.clone(); // 实现了clone
    println!("s3 = {}, s4 = {}", s3, s4);  // 这是可以的，因为我们显式地克隆了 s3
}