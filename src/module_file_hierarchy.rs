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
    println!("called `function()`")
}

#[cfg(test)]
mod tests{
    use crate::module_file_hierarchy::function;

    #[test]
    fn test_module_file_hierarchy(){
        function()
    }
}