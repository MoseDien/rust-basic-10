pub fn run() {
    create_on_heap();
    creat_list();
}

// 最简单的例子
fn create_on_heap() {
    // 创建一个在栈上的整数
    let x = 5;

    // 使用 Box 将整数 x 移动到堆上
    let boxed_x = Box::new(x);

    // 输出栈上的值
    println!("x = {}", x);

    // 输出堆上的值
    println!("boxed_x = {}", boxed_x);
}

// 这是一个链表，实现效果类似于堆栈stack - 
// Cons 来自于Lisp，Construct
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