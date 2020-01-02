/*
可以在路径中使用super（父级）和self（自身）关键字，从而在访问项时消除歧义，以及防止不必要的路径硬编码。
*/

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my_mod {
    fn function() {
        println!("called `my_mod::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my_mod::cool::function()`");
        }
    }

    pub fn indirect_call() {
        // 从这个作用域中访问所有名为`function`的函数！

        // `self`关键字表示当前的模块作用域——在这个例子是`my_mod`。
        // 调用`self::function()`和直接调用`function()`都得到相同的结果，
        // 因为他们表示相同的函数。
        self::function();
        function();

        // 我们也可以使用`self`来访问`my_mod`内部的另一个模块：
        self::cool::function();
        // 等价于
        cool::function();

        // `super`关键字表示父作用域（在`my_mod`模块外面）。
        super::function();

        // 这将在 `crate` 作用域内绑定 `cool::function` 。
        // 在这个例子中，crate作用域是`最外面`的作用域。
        {
            use crate::module_super_and_self::cool::function as root_function;
            root_function();
            // 等价于
            crate::module_super_and_self::cool::function();
        }
    }
}

mod tests {
    #[test]
    fn test_module_super_and_self() {
        crate::module_super_and_self::my_mod::indirect_call();

    }
}