use std::sync::{Arc, Mutex};
use std::thread;

/*
Arc的意思是 "atomic reference counter"(原子引用计数器) -- 多个onwership的情况
Rc只能处理单线程，而Arc可以处理多线程
当线程之间所有权需要共享时，可以使用Arc
*/
pub fn run() {
    main();
    main2();
    main3();
    main_rc();
    main_rc_mut();
}
fn main() {
    // 1- 创建一个 Arc 包装的 Mutex，保护共享的数据 - 这儿是一个整数
    // counter此处隐函了mut
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 启动10个线程，每个线程增加counter的值
    for _ in 0..10 {
        let counter = Arc::clone(&counter); // 2- 克隆Arc以在线程之间共享
        let handle: std::thread::JoinHandle<()> = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 3- 获取锁并访问数据
            *num += 1; // 4 - 手动 de ref - 并增加inc，在退出这个scope的时候会自动unlock
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

fn main2() {
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || { // Only the clone goes into Thread 1
        for _ in 0..10 {
            *my_number1.lock().unwrap() +=1; // Lock the Mutex, change the value
        }
    });

    let thread2 = std::thread::spawn(move || { // Only the clone goes into Thread 2
        for _ in 0..10 {
            *my_number2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {:?}", my_number);
    println!("Exiting the program");
}

// 如下例子，x并没有在线程之间共享，而是主线程有x，而每个线程有单独的一份x
// 最后输出的是 主线程的那个x
fn main3() {
    let mut x = 0;
    x += 1;

    let thread1 = std::thread::spawn(move || { 
        for _ in 0..5 {
            x += 1;
            // println!("Value is: {:?}", x);
        }
    });

    let thread2 = std::thread::spawn(move || { 
        for _ in 0..5 {
            x += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {:?}", x);
    println!("Exiting the program");
}

use std::rc::Rc;

// 定义一个简单的结构体
struct Data {
    value: i32,
}

impl Data {
    fn new(value: i32) -> Self {
        Data { value }
    }

    fn print_value(&self) {
        println!("Value: {}", self.value);
    }
}

fn main_rc() {
    // 创建一个 Rc<Data>
    let data = Rc::new(Data::new(42));

    // 创建两个 Rc 克隆，它们都指向同一个 Data 实例
    let data1 = Rc::clone(&data);
    let data2 = Rc::clone(&data);

    // 使用共享的数据
    data.print_value();
    data1.print_value();
    data2.print_value();

    // 打印引用计数
    println!("Reference count: {}", Rc::strong_count(&data));

    // 在一个新的作用域中创建另一个引用
    {
        let data3 = Rc::clone(&data);
        println!("Reference count inside scope: {}", Rc::strong_count(&data));
    } // data3 在这里离开作用域

    // 作用域结束后，引用计数减少
    println!("Reference count after scope: {}", Rc::strong_count(&data));
}


use std::cell::RefCell;

// 定义一个简单的结构体
struct Counter {
    count: u32,
}

impl Counter {
    fn new(initial: u32) -> Self {
        Counter { count: initial }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn get_count(&self) -> u32 {
        self.count
    }
}

/*
单线程中使用 Rc<RefCell<T>> 来修改
同样，并不需要显示声明伟mut，本身就隐含了mut
*/
fn main_rc_mut() {
    // 创建一个 Rc<RefCell<Counter>>
    let counter = Rc::new(RefCell::new(Counter::new(0)));

    // 创建两个 Rc 克隆
    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);
    let counter3 = Rc::clone(&counter);

    // 使用第一个引用修改计数器
    {
        let mut mutable_counter = counter1.borrow_mut();
        mutable_counter.increment();
        println!("Counter1: {}", mutable_counter.get_count());
    } // mutable_counter 在这里离开作用域，释放借用

    // 使用第二个引用修改计数器
    {
        let mut mutable_counter = counter2.borrow_mut();
        mutable_counter.increment();
        println!("Counter2: {}", mutable_counter.get_count());
    }

    // 使用原始引用读取计数器的值
    println!("Final count: {}", counter.borrow().get_count());

    // 打印引用计数 - 这个是对clone的计数
    println!("Reference count: {}", Rc::strong_count(&counter));
}