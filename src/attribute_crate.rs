/*
    crate_type属性可以告知编译器 crate 是一个二进制的可执行文件还是一个库（甚至是哪种类型的库）
    crate_name 属性可以设定 crate 的名称

    注意:
        在使用 cargo 时，这两种类型时都没有作用。
        由于大多数 Rust 工程都使用 cargo，这意味着 crate_type 和 crate_name 的作用事实上很有限。

*/

// 表示这个crate是一个库文件
#![crate_type = "lib"]
// 库的名称为`michael`
#![crate_name = "michael"]

pub fn public_function() {
    println!("called michael's `public_function()`");
}

fn private_function() {
    println!("called michael's `private_function()`");
}

pub fn indirect_access() {
    print!("called michael's `indirect_access()`, that\n> ");
    private_function();
}

// 当在文件内表明了crate_type属性时，就不需要在编译的时候给rustc指令加上`--crate-type`标记
// 打开终端，输入：
// $ rustc attribute_crate.rs
// $ ls lib*
// 生成了`libmichael.rlib`库文件