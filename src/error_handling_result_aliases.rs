/*
    当我们要重用某个 Result 类型时，，Rust 允许我们创建别名。
    若某个 Result 有可能被重用，可以方便地给它取一个别名。

    在模块的层面上创建别名特别有帮助。
    同一模块中的错误常常会有`相同`的 Err 类型，所以单个别名就能简便地定义所有相关的 Result。
    这太有用了，以至于标准库 也提供了一个别名： io::Result！
*/

// 下面给出一个简短的示例来展示语法：
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    // 为带有错误类型 `ParseIntError` 的 `Result` 定义一个泛型别名。
    type AliasedRusult<T> = Result<T, ParseIntError>;

    fn multiply(first_num_str: &str, second_num_str: &str) -> AliasedRusult<i32> {
        first_num_str.parse::<i32>().and_then(|n1| {
            second_num_str.parse::<i32>().map(|n2| n1 * n2)
        })
    }

    fn print_aliased_result(result: AliasedRusult<i32>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("Error: {}", e)
        }
    }

    #[test]
    fn test_error_handling_result_aliases() {
        // 无错误
        print_aliased_result(multiply("10", "11"));
        // 有error
        print_aliased_result(multiply("michael.w", "11"));
    }
}