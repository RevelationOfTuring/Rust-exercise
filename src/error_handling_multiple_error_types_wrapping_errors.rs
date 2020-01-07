/*
    把错误装箱这种做法也可以改成把它包裹到你自己的错误类型中。
*/
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;
    use std::fmt::Formatter;

    // 定义Result别名
    type Result<T> = std::result::Result<T, DoubleError>;

    // 自定义错误类型（枚举）
    #[derive(Debug)]
    enum DoubleError {
        EmptyVec,
        // 在这个错误类型中，我们采用 `parse` 的错误类型中 `Err` 部分的实现。
        // 若想提供更多信息，则该类型中还需要加入更多数据。
        Parse(ParseIntError),
    }

    impl std::fmt::Display for DoubleError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            // 注意：在这里进行 match 匹配
            match *self {
                // 当vector中无元素时
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                // 这是一个封装（wrapper），它采用内部各类型对 `fmt` 的实现。
                DoubleError::Parse(ref e) => e.fmt(f),
                // 等价于
                // DoubleError::Parse(ref e) => write!(f, "parse Error: {}", e),
            }
        }
    }

    impl std::error::Error for DoubleError {}

    // 实现从 `ParseIntError` 到 `DoubleError` 的转换。
    impl From<ParseIntError> for DoubleError {
        fn from(parse_int_err: ParseIntError) -> Self {
            DoubleError::Parse(parse_int_err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        // 这个地方如果发生报错，那一定是empty错误
        let first_str = vec.first().ok_or(DoubleError::EmptyVec)?;
        // 事实上 `?` 是 “要么 unwrap 要么 return Err(From::from(err))”,
        // 即如果报错将自动将ParseIntError转为自定义错误类型DoubleError
        let num = first_str.parse::<i32>()?;
        Ok(num * 2)
    }

    fn print_result(result: Result<i32>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("DoubleError: {}", e),
        }
    }

    #[test]
    fn test_error_handling_multiple_error_types_wrapping_errors() {
        let numbers = vec!["10", "20", "30"];
        print_result(double_first(numbers));
        // 输出:20

        let strings = vec!["michael.w", "20", "30"];
        print_result(double_first(strings));
        // 输出:DoubleError: parse Error: invalid digit found in string

        let empty = Vec::new();
        print_result(double_first(empty))
        // 输出:DoubleError: please use a vector with at least one element
    }
}