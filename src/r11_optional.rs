pub fn run() {
    main();
}
fn main() {
    // 创建一个Option<i32>
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    // 使用match来处理Option
    match some_number {
        Some(i) => println!("Got a number: {}", i),
        None => println!("No number"),
    }

    // 使用if let来简化匹配
    // 注意不是swift/kt的 if let x = 
    if let Some(i) = some_number {
        println!("Number: {}", i);
    }

    // 使用unwrap_or提供默认值
    let result = no_number.unwrap_or(0);
    println!("Result with default: {}", result);

    // 使用map对Some中的值进行转换
    let doubled = some_number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);

    // 链式调用
    let name = Some("Alice");
    let greeting = name
        .map(|n| format!("Hello, {}!", n))
        .unwrap_or_else(|| "Hello, stranger!".to_string());
    println!("{}", greeting);
}