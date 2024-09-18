use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    main();
}
fn main() {
    // 1- 创建一个 Arc 包装的 Mutex，保护共享的数据 - 这儿是一个整数
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 启动10个线程，每个线程增加counter的值
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 2- 克隆Arc以在线程之间共享
        let handle: std::thread::JoinHandle<()> = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 3- 获取锁并访问数据
            *num += 1; // 4 - 手动 de ref -
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印最后的计数值
    println!("Result: {}", *counter.lock().unwrap());
}
