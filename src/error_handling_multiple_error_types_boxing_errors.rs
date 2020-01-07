/*
    如果又想写简单的代码，又想保存原始错误信息，一个方法是把它们装箱（Box）。
    即，将错误类型装到Box中，并将Box放到Result的Err的位置。

    这样做的坏处就是，被包装的错误类型只能在`运行时`了解，而不能被静态地判别。

    对任何实现了 Error trait 的类型，
    标准库的 Box 通过 From 为它们提供了到 Box<Error> 的转换。
*/

mod tests {
    use std::fmt::Formatter;

    // 为 `Box<error::Error>` 取别名
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[derive(Debug)]
    struct EmptyVec;

    impl std::fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    impl std::error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        // first()返回的是一个Option
        vec.first().
            // 将错误类型EmptyVec装箱.ok_or_else返回的是一个Result
            ok_or_else(|| EmptyVec.into())
            // 开始parse&str
            .and_then(|num_str| num_str.parse::<i32>()
                // Result中的错误装箱
                .map_err(|e| e.into())
                .map(|num| num * 2))
    }

    /*
    ok_or_else是Option的一个类方法：
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Some(v) => Ok(v),
            None => Err(err()),
        }
    }

    功能：
        如果Option为Some，则返回对应的Ok(转成Result)
        如果Option为None，则返回闭包err生成的Err
    */

    fn print_result(result: Result<i32>) {
        match result {
            Ok(i) => println!("{}", i),
            Err(e) => println!("Box Error: {}", e),
        }
    }

    #[test]
    fn test_error_handling_multiple_error_types_boxing_errors() {
        let numbers = vec!["10", "20", "30"];
        print_result(double_first(numbers));
        // 输出：20

        let strings = vec!["michael.w", "20", "30"];
        print_result(double_first(strings));
        // 输出：Box Error: invalid digit found in string

        let empty = Vec::new();
        print_result(double_first(empty));
        // 输出：Box Error: invalid digit found in string
    }
}