// 需要使用完整的格式 - 在Cargo.toml里面已经定义了: name = "rust_10"
use rust_10::r1_closure;
use rust_10::r2_box;
use rust_10::r3_arc;
use rust_10::r4_deref;
use rust_10::r5_dyn;
use rust_10::r6_mut;
use rust_10::r7_error;
use rust_10::r8_ownership;
use rust_10::r9_copyclone;
use rust_10::r10_string;
use rust_10::r11_optional;
use rust_10::r12_binding;

fn main() {
    println!("Hello, world!");
    r12_binding::run();
    // r11_optional::run();
    // r10_string::run();
    // r9_copyclone::run();
    // r8_ownership::run();
    // r7_error::run();
    // r6_mut::run();
    // r5_dyn::run();
    // r4_deref::run();
    // r3_arc::run();
    // r2_box::run();
    // r1_closure::run();
}
