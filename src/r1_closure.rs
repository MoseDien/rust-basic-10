
pub fn run() {
    add_short();
    add();
    add_capture();
    test_apply();
    test_fnonce();
    test_apply_fn_mut();
    _ = main_FnOnce();
    main_return_Fn();
}

// case 1: 闭包是一个匿名函数，本处把参数类型标出来
fn add_short() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    println!("Result: {}", add(5, 3)); // 输出: Result: 8
}

// case 2: 自动类型推断，可以简写，
// 但存在一些类型问题，不是范型，而仅仅是省略了参数类型
// 也就是第一次被调用之后所有的类型就确定了，
fn add() {
    let add = |x, y| x + y;
    println!("Result: {}", add(5, 3)); // 输出: Result: 8
    // 错误: 因为add闭包已经确定了类型 
    // expected integer, found floating-point number
    // println!("Result: {}", add(5.0, 3.1)); 
}

// case 3: 捕获环境中的变量 - closure和fn的不同
// 这儿还可以添加move，把所有权转移到闭包里面去
fn add_capture() {
    let x = 5;
    let add_x = |y: i32| y + x; // 捕获了变量x
    println!("Result: {}", add_x(3)); // 输出: Result: 8
}

// case 4: 闭包作为参数Fn
// Fn - 以只读的方式捕获变量，FnMut则可以修改捕获的变量
/* Fn - Trait std::ops::Fn - Fn是一个trait，定义在ops里面
pub trait Fn<Args>: FnMut<Args>
where
Args: Tuple,
{
// Required method
extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
*/
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

// case: FnOnce
// 同样的 FnOnce也是一个traits
fn consume_with_fn_once<F>(func: F, val: i32)
where F: FnOnce(i32),
{
    func(val); // 调用闭包，并传递参数
    // func(val); // 不能再次调用 - 这也是FnOnce本来的含义
}

// 核心 - 拥有捕获的所有权，但不能修改捕获的变量
// 注意move关键字 - 
fn test_fnonce() {
    let s = String::from("Hello, Rust!");

    // 使用 move 关键字将 s 的所有权移动到闭包中
    let consume_string = move |num: i32| {
        println!("String: {}, Number: {}", s, num); // 使用了捕获的 s 和传入的 num
    };

    consume_with_fn_once(consume_string, 42);

    // 不能再次调用了，如下会抱错 - 这也是FnOnce的含义
    // consume_with_fn_once(consume_string, 42); 
    // println!("{}", s); // 这里会报错，s 的所有权已经被移动到闭包中
}

// case: FnMute，可以修改捕获的变量
// FnMut 也是 traits
// 也就是说此处不能是Fn，因为闭包increment修改了捕获的count
fn apply_fn_mut<F>(mut func: F)
where F: FnMut(),
{
    func();
}

fn test_apply_fn_mut() {
    let mut count = 0;

    // 闭包对捕获的 count 进行了修改
    let increment = || {
        count += 1;
    };

    apply_fn_mut(increment);

    // 输出: Final count: 1
    println!("Final count: {}", count); 
}

use std::fs::File;
use std::io::Write;

// case: 实际的FnOnce
// writer这个FnOnce，只能被调用一次
fn write_to_file<F>(filename: &str, writer: F) -> std::io::Result<()>
where
    F: FnOnce(&mut File) -> std::io::Result<()>
{
    let mut file = File::create(filename)?;
    writer(&mut file)?;
    // 编译错误，不能再此调用 - value used here after move
    // writer(&mut file)?;
    Ok(())
}

/*
write!返回的是 std::io::Result 
pub type Result<T> = Result<T, Error>;
后面这个Result来自 - 最基本的枚举类型，并没有限定T/E
std::result::Result
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
而这个Error - 是特定的 std::io::Result
*/
fn main_FnOnce() -> std::io::Result<()> {
    write_to_file("example.txt", |file| {
        writeln!(file, "Hello, FnOnce!");
        writeln!(file, "Hello, DONE!")
    })?;
    Ok(())
}

// case: 函数返回一个闭包Fn，
// 这个闭包有一个输入i32参数，并且返回一个i32
// x是函数的参数，y是闭包的参数
// 而x最终成为闭包的捕获，而且强制要求使用move，把x的所有权转移到闭包
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    // 错误 - 这儿必须添加 move
    // |y| x + y
    move |y| x + y
}

fn main_return_Fn() {
    let add_five = create_adder(5);
    println!("结果: {}", add_five(10));  // 输出: 结果: 15
}