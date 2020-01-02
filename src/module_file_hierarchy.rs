/*
模块可以分配到文件/目录的层次结构中。
将module_visibility.rs中的例子代码拆分到多个文件中：

    $ tree .

    ├── module_file_hierarchy
    │   │   └── my_mod
    │   │       ├── inaccessible.rs
    │   │       ├── mod.rs
    │   │       └── nested.rs
    │   ├── module_file_hierarchy.rs

*/

mod my_mod;

fn function(){
    println!("called `function()`. File path: src/module_file_hierarchy.rs")
}

#[cfg(test)]
mod tests{
    use crate::module_file_hierarchy::function;

    #[test]
    fn test_module_file_hierarchy(){
        // 调用本文件内的函数function
        function();

        // 调用module_file_hierarchy/my_mod/mod.rs中的函数function
        super::my_mod::function();
        // 等价于
        crate::module_file_hierarchy::my_mod::function();

        // 调用module_file_hierarchy/my_mod/mod.rs中的函数indirect_access
        super::my_mod::indirect_access();

        // 调用module_file_hierarchy/my_mod/nested.rs中的函数
        // 由于nested.rs中有私有函数，所以默认该mod也是私有的
        // 如果需要调用该mod nested中的公共函数，需要在module_file_hierarchy/my_mod/mod.rs中导入nested.rs时，
        // 将其声明为：pub mod nested;
        super::my_mod::nested::function()
    }
}