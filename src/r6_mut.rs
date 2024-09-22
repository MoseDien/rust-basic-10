pub fn run() {
    main();
}
struct MyStruct {
    field: i32,
}

// 如果想修改MyStruct，则必须提供 mut的函数
trait MyTrait {
    fn modify_field(&mut self);
}

impl MyTrait for MyStruct {
    fn modify_field(&mut self) {
        self.field += 1;
    }
}

fn main() {
    let mut s = MyStruct { field: 0 };
    s.modify_field();
    println!("field: {}", s.field);
}