pub fn run() {
    main();
}

fn main() {
    // 变量绑定 - 这行代码将值5绑定到变量名x上。
    let x = 5;
    
    // 创建一个到x的不可变引用
    let y = &x;
    
    // 创建一个可变绑定
    let mut z = 10;
    
    // 创建一个到z的可变引用
    // Rust的借用规则确保在同一时间只能有一个可变引用
    let w = &mut z;
    
    // 使用引用 - 此处不能输出z，因为z存在一个可变的引用，此时w拥有所有权而且可以修改
    println!("x: {}, y: {}, z: {}", x, y, w);

    // 此处primitive type需要显示的解引用deref，如果是非primitive，则编译器会自动
    *w += 5;
    // w的生命周期结束了，可以输出z了；
    // 相当于编译器可以判断出w的生命周期做了优化，
    // 如果根据scope，w是在函数结束之前才被销毁的；当然也可添加scope来加强{}
    println!("z after modification: {}", z);
}