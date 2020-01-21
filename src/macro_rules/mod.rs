/*
    使用`macro_rules!`来创建宏

    Rust提供了一个强大的宏系统，可进行元编程（metaprogramming）。

    宏看起来和函数很像，只不过名称末尾有一个感叹号 !

    宏并不产生函数调用，而是`展开`成源码，并和程序的其余部分一起被编译。

    Rust又有一点和C以及其他语言都不同，那就是Rust的宏会展开为抽象语法树（AST，abstract syntax tree），
    而不是像字符串预处理那样直接替换成代码，这样就不会产生无法预料的优先权错误。
*/

/*
    宏有什么用处？

    1. 不写重复代码（DRY，Don't repeat yourself.）。
        很多时候需要在一些地方针对不同的类型实现类似的功能，这时常常可以使用宏来避免重复代码（稍后详述）。
    2. 领域专用语言（DSL，domain-specific language）。
        宏允许你为特定的目的创造特定的语法（稍后详述）。
    3. 可变接口（variadic interface）。
        有时你需要能够接受不定数目参数的接口，比如 println!，根据格式化字符串的不同，它需要接受任意多的参数（稍后详述）。
*/
mod syntax;
mod dont_repeat_yourself;
mod domain_specific_languages;
mod variadic_interfaces;

#[cfg(test)]
mod tests {
    // 定义一个简单的宏,其名字为`hello_michael`
    macro_rules! hello_michael {
        // `()` 表示此宏不接受任何参数。
        ()=>(
            // 此宏将会展开成这个代码块里面的内容
            println!("Hello Michael.W！");
        )
    }

    #[test]
    fn test_macro_rules() {
        // 调用宏
        hello_michael!();
    }
}