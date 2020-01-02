/*
use 声明可以将一个完整的路径绑定到一个新的名字，从而更容易访问。
*/

#[cfg(test)]
mod tests {
    //    绑定到一个新的名字
    use deeply::inner::function as i_function;

    mod deeply {
        pub mod inner {
            pub fn function() {
                println!("called `deeply::inner::function()`")
            }
        }
    }

    fn function() {
        println!("called `function()`");
    }

    #[test]
    fn test_module_use_declaration() {
        // 更容易访问 `deeply::inner::function`
        i_function();

        println!("Entering block");
        {
            // 这和 `use deeply::inner::function as i_function` 等价。
            // 此 `function()` 将掩蔽外部的同名函数。
            use deeply::inner::function;
            function();

            // `use` 绑定拥有局部作用域。在这个例子中，`function()`
            // 的掩蔽只存在在这个代码块中。
            println!("Leaving block");
        }
        function();
    }
}
