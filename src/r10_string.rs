
/*
String::from("Hello") 在堆上分配空间来存储 "Hello"
s 是一个存储在栈上的 String 结构体，包含指向堆数据的指针、长度和容量
当我们修改字符串时，如果新的内容超过了当前的容量，Rust 会在堆上重新分配一个更大的空间
 
栈上的 String 结构体：包含三个字段：
- 指向堆内存的指针（ptr）
- 字符串的长度（len）
- 分配的容量（capacity）
这个结构体本身是存储在栈上的

堆上的字符数据：
- 实际的字符序列存储在堆上-heap
这允许字符串内容动态增长而不需要在栈上移动数据
 */

pub fn run() {
    main();
}

fn main() {
    let s = String::from("Hello");
    println!("字符串的长度: {}", s.len());
    println!("字符串的容量: {}", s.capacity());
    
    // 修改字符串
    let mut s_mut = s;
    s_mut.push_str(" World");
    println!("修改后的字符串: {}", s_mut);
    println!("新的长度: {}", s_mut.len());
    println!("新的容量: {}", s_mut.capacity());
}
// String、str、&str、&String、Box<str> 或 Box<&str> 

// str
// str代表字符串字面量，其存储在常量区，是全生命周期的，
// 没有内存回收问题，与所有权机制冲突，所以str类型只存在于概念中，只有引用形式&str；

fn main2() {
    // &str
    // &str是切片slice类型，也就是 &str指向 str或者String
    // 如下的s是不可改变的，immutable
    let s: &str = "Hello, world!";
    println!("{}", s); // This will print: Hello, world!
}

// 很少用到 Box<str>
fn main3() {
    let s: Box<str> = Box::from("Hello, world!");
    println!("{}", s); // This will print: Hello, world!
}

// 很少用到 Box<&str>
fn main4() {
    let s: &str = "Hello, world!";
    let t: Box<&str> = Box::new(s);
    println!("{}", t); // This will print: Hello, world!
}

// raw_str: 其实是 &str类型
// 可以直接包函unicode
fn main5() {
    let raw_str: &str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你要在原始字符串中写引号，请在两边加一对 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果字符串中需要写 "#，那就在定界符中使用更多的 #。
    // 可使用的 # 的数目没有限制。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}


use std::str;

// 非unicode的字符串，字节串 - b-str
// u8的数组
fn main6() {
    // 注意这并不是一个 &str
    let bytestring: &[u8; 20] = b"this is a bytestring";

    // 字节串没有实现 Display，所以它们的打印功能有些受限
    println!("A bytestring: {:?}", bytestring);

    // 字节串可以使用单字节的转义字符...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但不能使用 Unicode 转义字符
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // 原始字节串和原始字符串的写法一样
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 把字节串转换为 &str 可能失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"

    // 但这样的话它们就无法转换成 &str 了
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}