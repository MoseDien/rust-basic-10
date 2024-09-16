// 需要使用完整的格式 - 在Cargo.toml里面已经定义了: name = "rust_10"
use rust_10::r1_closure;
use rust_10::r2_box;
use rust_10::r3_arc;
use rust_10::r4_deref;
use rust_10::r5_dyn;
use rust_10::r6_mut;

fn main() {
    println!("Hello, world!");
    r6_mut::run();
    // r5_dyn::run();
    // r4_deref::run();
    // r3_arc::run();
    // r2_box::run();
    // r1_closure::run();
}
