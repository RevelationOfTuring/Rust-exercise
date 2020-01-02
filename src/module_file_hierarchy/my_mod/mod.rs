// 类似地，`mod inaccessible`和`mod nested`将找到同级目录下的
// `nested.rs`和`inaccessible.rs`文件，并在它们放到各自的模块中。
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my_mod::function()`. File path: src/module_file_hierarchy/my_mod/mod.rs")
}

// 私有的
fn private_function() {
    println!("called `my_mod::private_function()`. File path: src/module_file_hierarchy/my_mod/mod.rs");
}

pub fn indirect_access() {
    print!("called `my_mod::indirect_access()`, that\n> ");
    private_function();
}