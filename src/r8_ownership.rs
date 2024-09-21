// 示例1: 基本所有权
pub fn run() {
    main1();
    main2();
    main3();
}

// String 不是primitive type
// String 没有实现Copy，但是实现了Clone
fn main1() {
    let s1 = String::from("hello"); // s1 是这个字符串的所有者
    let s2 = s1; // 所有权从 s1 移动到 s2
    
    // println!("{}", s1); // 这行会导致编译错误,因为 s1 不再拥有这个值
    println!("{}", s2); // 这是可以的,因为 s2 现在是所有者
}

// 示例2: 函数与所有权
// primitive types 默认都是实现了copy
fn main2() {
    let s = String::from("hello");
    takes_ownership(s); // s 的值移动到函数里
    // println!("{}", s); // 这行会报错,s 的值已经被移走了

    let x = 5;
    makes_copy(x); // i32 是 Copy 的,所以 x 后面仍然可用
    println!("{}", x); // 这是可以的,因为 x 没有被移走
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // 这里,some_string 离开作用域并被丢弃

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 这里,some_integer 离开作用域,不会有特殊操作

// 示例3: 返回值与作用域
fn main3() {
    let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数里,函数又将返回值移给 s3
} // 这里, s3 离开作用域并被丢弃, s2 已被移走,s1 离开作用域并被丢弃

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}