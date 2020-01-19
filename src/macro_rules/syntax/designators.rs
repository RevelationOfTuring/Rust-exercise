/*
    指示符

    宏的参数使用一个美元符号`$`作为前缀，并使用一个指示符（designator）来注明类型。
*/

/*
    全部指示符：
        - block
        - expr 用于表达式
        - ident 用于变量名或函数名
        - item
        - pat (模式 pattern)
        - path
        - stmt (语句 statement)
        - tt (标记树 token tree)
        - ty (类型 type)
*/
#[cfg(test)]
mod tests {
    // 定义一个宏来创建函数
    macro_rules! create_function {
        // 此宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数。
        // `ident` 指示符用于变量名或函数名
        ($function_name:ident)=>(
            fn $function_name(){
                // `stringify!` 宏把 `ident` 转换成字符串
                println!("function {} is invoked", stringify!($function_name))
            }
        )
    }

    // 借助上述宏来创建名为 `func1` 和 `func2` 的函数。
    create_function!(func1);
    create_function!(func2);

    macro_rules! print_result {
        // 此宏接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起打印出来。
        // `expr` 指示符表示表达式。
        ($expression:expr)=>(
            // `stringify!` 把表达式`原样`转换成一个字符串。
            println!("{:?} = {:?}",
                stringify!($expression),    // 为了打印，将表达式转成字符串
                $expression                 // 执行表达式
            )
        )
    }


    #[test]
    fn test_designators() {
        // 执行利用宏定义的函数
        func1();
        func2();

        // 宏的参数是表达式
        print_result!(1024f64*10.0);
        // 输出："1024f64 * 10.0" = 10240.0

        // 切记：代码块也是表达式
        print_result!({
            let x = 15f64;
            x*x+2.0*x+1.0
        });
    }
}