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

// 使用dyn trait对象
fn animal_sound(animal: &dyn Animal) {
    animal.make_sound();
}

fn main() {
    let dog = Dog;
    let cat: &dyn Animal = &Cat;

    // 我们可以通过&dyn Animal调用animal_sound，传入不同的类型
    animal_sound(&dog); // 输出: Woof!
    animal_sound(cat); // 输出: Meow!
}
