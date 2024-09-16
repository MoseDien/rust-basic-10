// primitive都需要手动de ref
// 非primitive会自动de ref
pub fn run() {
    {
        let mut x = 5;
        let y = &mut x;  // y 是一个可变引用
        *y += 1;         // 通过解引用修改 x 的值
        println!("x 的新值: {}", x);  // 输出 x = 6
    }

    {
        let b = Box::new(10);  // b 是一个 Box 智能指针，指向值 10 - 基本类型
        println!("b 解引用后的值: {}", *b);  // 必须解引用以获取值
    }

    // 
    {
        let x = 5;
        let x: i32 = 5;
        let y: &i32 = &x;     // y 是 x 的引用
        let z: &&i32 = &y;     // z 是 y 的引用，也就是指向引用的指针
        println!("z 的解引用后的值: {}", **z);  // 需要两次解引用才能获取 x 的值
    }
    
    let s = Box::new(String::from("hello"));
    // 这里 Rust 自动帮我们解引用了 Box，调用了 String 类型的方法
    println!("字符串的长度是: {}", s.len());  // 自动解引用，不需要 *s.len()
    println!("字符串的长度是: {}", (*s).len());  // 可以手动
}

