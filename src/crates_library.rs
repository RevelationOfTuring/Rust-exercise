/*
crate（中文有 “包，包装箱” 之意）是Rust的编译单元。
当调用rustc some_file.rs 时，some_file.rs被当作crate文件。
如果文件some_file.rs里面含有mod声明，那么模块文件的内容将在编译之前被插入crate文件的相应声明处。
换句话说，模块不会单独被编译，只有crate才会被编译。

crate可以编译成二进制可执行文件（binary）或库文件（library）。
默认情况下，rustc将从crate产生库文件。
这种行为可以通过rustc的选项 --crate-type 重载。
*/

// 创建一个库，然后看看如何把它链接到另一个crate：

pub fn public_function() {
    println!("called `public_function()`");
}

fn private_function() {
    println!("called `private_function()`");
}

pub fn indirect_access() {
    print!("called `indirect_access()`, that\n> ");
    // 调用私有函数
    private_function();
}

// 打开终端，执行：
// $ rustc --crate-type=lib crates_library.rs
// $ ls lib*
// libcrates_library.rlib

// 可见生成了一个库文件libcrates_library.rlib
// 默认情况下，库会使用crate文件的名字，前面加上 “lib” 前缀，
// 但这个默认名称可以 使用crate-name属性覆盖。
// $ rustc --crate-type=lib --crate-name=michael crates_library.rs
// 生成了 libmichael.rlib 文件
