use std::fs;
use std::io;
use std::num::ParseIntError;

pub fn run() {
    // if let number = read_and_parse() {
    //     println!("result: {:?}", number);
    // }
    // unwrap
    if let Ok(number) = read_and_parse() {
        println!("number: {}", number);
    } else {
        println!("failed!");
    }
    //
    match read_and_parse() {
        Ok(number) => println!("成功读取并解析的数字：{}", number),
        Err(error) => match error {
            MyError::Io(io_error) => println!("IO 错误：{}", io_error),
            MyError::Parse(parse_error) => println!("解析错误：{}", parse_error),
        },
    }
    // 
    let number = read_and_parse().unwrap();
    println!("成功读取并解析的数字：{}", number);
    //
    let number = read_and_parse().unwrap_or(0);
    println!("读取到的数字（如果失败则为 0）：{}", number);
    //
    let number = read_and_parse().expect("无法读取或解析文件");
    println!("成功读取并解析的数字：{}", number);
    //
    if let Err(err) = read_and_parse() {
        println!("处理文件时发生错误：{:?}", err);
    }
}

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

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

fn read_and_parse() -> Result<i32, MyError> {
    let mut contents = String::new();
    // File::open("number.txt")?.read_to_string(&mut contents)?;
    contents = fs::read_to_string("number.txt")?;

    let number: i32 = contents.trim().parse()?;
    Ok(number)
}