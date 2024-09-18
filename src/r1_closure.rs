
pub fn run() {
    add_short();
    add();
    add_capture();
    test_apply();
    test_fnonce();
    test_apply_fn_mut();
}

// case 1: 闭包是一个匿名函数
fn add_short() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("Result: {}", add(5, 3)); // 输出: Result: 8
}

// case 2: 自动类型推断，可以简写，但存在一些类型问题，不是范型
fn add() {
    let add = |x, y| x + y;
    println!("Result: {}", add(5, 3)); // 输出: Result: 8
    // error: expected integer, found floating-point number
    // println!("Result: {}", add(5.0, 3.1)); 
}

// case 3: 捕获环境中的变量
fn add_capture() {
    let x = 5;
    let add_x = |y: i32| y + x; // 捕获了变量x
    println!("Result: {}", add_x(3)); // 输出: Result: 8
}

// case4: 闭包作为参数和返回值
fn apply<F>(f: F)
where F: Fn(i32) -> i32,
{
    println!("Result: {}", f(5));
}

fn create_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn test_apply() {
    // x是参数 - 闭包: 一个参数x，返回 x+1
    apply(|x| x + 1); // 输出: Result: 6
    
    //
    let closure = create_closure();
    println!("Result: {}", closure(5)); // 输出: Result: 6
}

fn consume_with_fn_once<F>(func: F, val: i32)
where F: FnOnce(i32),
{
    func(val); // 调用闭包，并传递参数
    // func(val); // 不能再次调用 - 这也是FnOnce本来的含义
}

// 核心 - 拥有捕获的所有权，但不能修改捕获的变量
fn test_fnonce() {
    let s = String::from("Hello, Rust!");

    // 使用 move 关键字将 s 的所有权移动到闭包中
    let consume_string = move |num: i32| {
        println!("String: {}, Number: {}", s, num); // 使用了捕获的 s 和传入的 num
    };

    consume_with_fn_once(consume_string, 42);
    // consume_with_fn_once(consume_string, 42); // 也不能再次调用了 - 这也是once的含义
    // println!("{}", s); // 这里会报错，s 的所有权已经被移动到闭包中
}

fn apply_fn_mut<F>(mut func: F)
where F: FnMut(),
{
    func();
    func(); // 可以多次调用
}

fn test_apply_fn_mut() {
    let mut count = 0;

    // 闭包可变地捕获了 count
    let increment = || {
        count += 1; // 闭包对 count 进行了修改
        println!("Count: {}", count);
    };

    apply_fn_mut(increment);

    // 输出：
    // Count: 1
    // Count: 2

    // 这里 count 的值是 2
    println!("Final count: {}", count); // 输出: Final count: 2
}