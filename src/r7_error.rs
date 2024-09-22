use std::fs;
use std::io;
use std::num::ParseIntError;

pub fn run() {
    // 错误的拆包方式，number可能是i32或者MyError，反正是有值
    // if let number = read_and_parse() {
    //     println!("result: {:?}", number);
    // }

    // unwrap
    if let Ok(number) = read_and_parse() {
        println!("number: {}", number);
    } else {
        println!("failed!");
    }
    // match 匹配
    match read_and_parse() {
        Ok(number) => println!("成功读取并解析的数字：{}", number),
        Err(error) => match error {
            MyError::Io(io_error) => println!("IO 错误：{}", io_error),
            MyError::Parse(parse_error) => println!("解析错误：{}", parse_error),
        },
    }
    // 如果是MyError则直接crash
    let number = read_and_parse().unwrap();
    println!("成功读取并解析的数字：{}", number);
    // 
    let number = read_and_parse().unwrap_or(0);
    println!("读取到的数字（如果失败则为 0）：{}", number);
    // 如果是MyError则直接crash
    let number = read_and_parse().expect("无法读取或解析文件");
    println!("成功读取并解析的数字：{}", number);
    // 只取出错误
    if let Err(err) = read_and_parse() {
        println!("处理文件时发生错误：{:?}", err);
    }
}

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

// From traits
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// 为了能直接使用直接返回
fn read_and_parse() -> Result<i32, MyError> {
    let mut contents = String::new();
    // File::open("number.txt")?.read_to_string(&mut contents)?;
    contents = fs::read_to_string("number.txt")?;

    let number: i32 = contents.trim().parse()?;
    Ok(number)
}