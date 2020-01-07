/*
    有时我只是想 unwrap 且避免产生 panic。

    到现在为止，对 unwrap 的错误处理都在强迫我们一层层地嵌套，
    然而我们只是想把里面的变量拿出来。
    `?` 正是为这种情况准备的。

    当找到一个 Err 时，可以采取两种行动：
        - panic!，不过我们已经决定要尽可能避免 panic 了。
        - 返回它，因为 Err 就意味着它已经不能被处理了。

    `?` 几乎就等于一个会返回 Err 而不是 panic 的 unwrap。
    看下面的例子：
*/
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
        let n1 = first_num_str.parse::<i32>()?;
        let n2 = second_num_str.parse::<i32>()?;

        Ok(n1 * n2)
    }

    fn print_result(result: Result<i32, ParseIntError>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_error_handling_result_question_mark() {
        // 正常现象
        print_result(multiply("10", "11"));
        // 返回错误(提前返回)
        print_result(multiply("michael.w", "11"));
    }

    /*
        补： 宏 try!
            在 ? 出现以前，相同的功能是使用 `try!` 宏完成的。
            现在我们推荐使用 ? 运算符，但是在老代码中仍然会看到 try!。
            如果使用 try! 的话，前面的 multiply 函数看起来会像是这样：
    */

    fn multiply_v2(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
        let n1 = try!(first_num_str.parse::<i32>());
        let n2 = try!(second_num_str.parse::<i32>());

        Ok(n1 * n2)
    }

    #[test]
    fn test_error_handling_result_try_macro() {
        print_result(multiply_v2("10", "11"));
        print_result(multiply_v2("michael.w", "11"));

        /* 注：
            上面编译都会报错(try现在是一个预留的关键字，会跟try！发生冲突):
            error: expected expression, found reserved keyword `try`
             --> src/error_handling_result_question_mark.rs:49:18
               |
            49 |         let n1 = try!(first_num_str.parse::<i32>());
               |                  ^^^ expected expression
        */

    }
}