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