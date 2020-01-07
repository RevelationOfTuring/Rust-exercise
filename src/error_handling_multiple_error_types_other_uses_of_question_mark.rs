/*
   前面的例子中，调用 parse 后总是立即把错误从标准库错误 map 到装箱的错误。
   .and_then(|s| s.parse::<i32>()
    .map_err(|e| e.into())

    因为这个操作很简单常见，如果有省略写法就好了。
    and_then 不够灵活，所以不能实现这样的写法。不过，我们可以使用 `?` 来代替它。

    之前我们说 `?` 就是 “要么 unwrap 要么 return Err(error)”。
    这大部分是对的，但事实上 `?` 是 “要么 unwrap 要么 return Err(From::from(err))”。

    From::from 是不同类型间的转换工具。
    也就是说，如果在错误能够转换成返回类型的地方使用 ?，它就会自动转换成返回类型。

    这里，使用 ? 重写之前的例子。这样，只要为我们的错误类型实现 From::from，就可以不再使用 map_err。
*/
#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    // 为 `Box<error::Error>` 取别名。
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl std::fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl std::error::Error for EmptyVec {}

    // 这里的结构和之前一样，但是这次没有把所有的 `Results` 和 `Options` 串起来，
    // 而是使用 `?` 立即得到内部值。
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        // first 为 &&str
        let first_str = vec.first().ok_or(EmptyVec)?;
        let parsed_num = first_str.parse::<i32>()?;

        // 扩大二倍并返回
        Ok(parsed_num * 2)
    }

    fn print_result(result: Result<i32>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("Box Error: {}", e),
        }
    }

    #[test]
    fn test_error_handling_multiple_error_types_other_uses_of_question_mark() {
        let numbers = vec!["10", "20", "30"];
        print_result(double_first(numbers));
        // 输出:20

        let strings = vec!["michael.w", "20", "30"];
        print_result(double_first(strings));
        // 输出:Box Error: invalid digit found in string

        let empty = Vec::new();
        print_result(double_first(empty))
        // 输出:Box Error: invalid first item to double
    }
}