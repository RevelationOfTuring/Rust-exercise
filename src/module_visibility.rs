/*
Rust 提供了一套强大的模块（module）系统，可以将代码按层次分成多个逻辑单元（模块），
并管理这些模块之间的可见性（公有（public）或私有（private））。

模块是项（item）的集合，项可以是：函数，结构体，trait，impl 块，甚至其它模块。
*/

// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项(item)可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod my_mod_inner {
        // 公有的
        pub fn function() {
            println!("called `my_mod::my_mod_inner::function()`");
        }

        // 私有的
        fn private_function() {
            println!("called `my_mod::my_mod_inner::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::module_visibility::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::my_mod_inner::public_function_in_my_mod()`, that\n > ");
            public_function_in_my_mod_inner();
        }
//        注：relative paths are not supported in visibilities on 2018 edition，try: `crate::module_visibility::my_mod`

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_my_mod_inner() {
            println!("called `my_mod::my_mod_inner::public_function_in_my_mod_inner");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::my_mod_inner::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        my_mod_inner::public_function_in_my_mod();
        print!("> ");
        my_mod_inner::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_my_mod_inner {
        pub fn function() {
            println!("called `my_mod::private_my_mod_inner::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

#[cfg(test)]
mod tests {
//    默认情况下，模块中的项拥有`私有`的可见性（private visibility），
//    不过可以加上 pub 修饰语来重载这一行为。
//    模块中只有公有的（public）项可以从模块外的作用域访问。



    use crate::module_visibility::function;
    use crate::module_visibility::my_mod;

    #[test]
    fn test_module_visibility() {
        // 模块机制消除了相同名字的项之间的歧义。
        function();
        my_mod::function();

        // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
        // 调用其他模块的pub函数
        my_mod::indirect_access();
        // 调用其他模块中嵌套模块的pub函数
        my_mod::my_mod_inner::function();
        // call_public_function_in_my_mod函数调用了同级嵌套模块中的函数
        my_mod::call_public_function_in_my_mod();

        // pub(crate) 项可以在同一个 crate 中的任何地方访问
        my_mod::public_function_in_crate();

        // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的
        // 报错！my_mod中的`private_function`函数是私有的
//        my_mod::private_function();

//        报错！my_mod_inner中的`private_function` 是私有的
//        my_mod::my_mod_inner::private_function();

//        private_my_mod_inner是my_mod中的一个私有模块
//        my_mod::private_my_mod_inner::function();


//        my_mod_inner中的函数public_function_in_my_mod规定只能在路径crate::module_visibility::my_mod下可见
//        报错！
//        my_mod::my_mod_inner::public_function_in_my_mod();
    }
}