pub fn run() {
    main()
}

// 定义一个Trait
trait Animal {
    fn make_sound(&self);
}

// 实现Trait的类型1：Dog
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// 实现Trait的类型2：Cat
struct Cat;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

// 使用dyn trait对象 - 
// dyn是dynamic的简写
fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

/*
静态分发 - static dispatch

定义：
静态分发意味着编译器在编译时就知道要调用哪个具体的函数或方法。这与动态分发相反，后者在运行时才确定调用哪个方法。

工作原理：
当使用泛型时，Rust编译器会为每个具体类型生成专门的代码。这个过程称为单态化（monomorphization）。

优点：
性能：由于函数调用在编译时解析，避免了运行时的开销。
内联优化：编译器可以将方法调用内联，进一步提高性能。
类型安全：在编译时捕获类型错误。

缺点：
代码膨胀：为每个具体类型生成代码可能导致编译后的程序变大。
编译时间：大量使用泛型可能增加编译时间。

与动态分发的对比：
动态分发（使用 trait 对象和 dyn 关键字）在运行时决定调用哪个方法，通过虚表（vtable）实现。
*/
fn animal_sound_static<T: Animal>(animal: T) {
    animal.make_sound();
}

fn main() {
    let dog = Dog; // 注意，由于Dog没有参数，这也相当于初始化了一个实例
    let cat: &dyn Animal = &Cat;
    // 我们可以通过&dyn Animal调用animal_sound，传入不同的类型
    animal_sound(&dog); // 输出: Woof!
    animal_sound(cat); // 输出: Meow!

    // 静态分发 - static dispatch
    animal_sound_static(dog);

    // 如下代码错误 - 因为animal_sound_static 只针对具体的类对象，而不是针对traits对象
    // ^^^ the trait `Animal` is not implemented for `&dyn Animal`
    // animal_sound_static(cat);
    animal_sound_static(Cat);
}
