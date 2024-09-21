/*
最简单直接的智能指针是 box，其类型是 Box<T>。
box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针。
*/
pub fn run() {
    create_on_heap();
    creat_list();
}

// 最简单的例子
fn create_on_heap() {
    // 创建一个在栈上的整数
    let x = 5;

    // 使用 Box 将整数 x 移动到堆上 - 这儿其实是copy，在heap上重新分配了一份
    let boxed_x = Box::new(x);

    // 输出栈上的值
    println!("x = {}", x);

    // 输出堆上的值
    println!("boxed_x = {}", boxed_x);
}

/*
一个递归数据类型
这是一个链表，实现效果类似于堆栈stack - 
Cons 来自于Lisp，Construct
cons list 并不是一个 Rust 中常见的类型，大部分都用来演示
*/
enum List {
    // 存储一个整数值和指向下一个节点的 Box
    Cons(i32, Box<List>),
    // 代表链表的结束
    Nil,
}

impl List {
    // 创建一个新链表
    fn new() -> List {
        List::Nil
    }

    // 向链表中添加一个新元素
    fn prepend(self, value: i32) -> List {
        List::Cons(value, Box::new(self))
    }

    // 打印链表中的所有元素
    fn print(&self) {
        match self {
            List::Cons(value, next) => {
                print!("{} ", value);
                next.print();
            }
            List::Nil => println!(),
        }
    }
}

fn creat_list() {
    // 创建一个空链表
    let list = List::new();

    // 向链表中添加元素
    let list = list.prepend(1);
    let list = list.prepend(2);
    let list = list.prepend(3);

    // 打印链表
    list.print(); // 输出: 3 2 1
}

/*
// Rust 需要在编译时知道类型占用多少空间, 但如下的定义是不知道大小的
//  ----- recursive without indirection
enum List2 {
    Cons(i32, List2),
    Nil,
}
*/