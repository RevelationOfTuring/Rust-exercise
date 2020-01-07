/*
    我们可以显式地使用组合算子处理了错误。
    另一种处理错误的方式是使用 `match 语句` 和 `提前返回`（early return）的结合。

    也就是说：
    如果发生错误，我们可以`停止`函数的执行然后返回错误。
    这样的代码更好写，更易读。
*/
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    // 如果遇到
    fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
        let n1 = match first_num_str.parse::<i32>() {
            Ok(i) => i,
            // 提前返回，如果出了错误
            Err(e) => return Err(e),
        };

        let n2 = match second_num_str.parse::<i32>() {
            Ok(i) => i,
            Err(e) => return Err(e),
        };

        // 正确的返回
        Ok(n1 * n2)
    }

    fn print_result(result: Result<i32, ParseIntError>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_error_handling_result_early_returns() {
        // 正常现象
        print_result(multiply("10", "11"));
        // 返回错误(提前返回)
        print_result(multiply("michael.w", "11"));
    }
}