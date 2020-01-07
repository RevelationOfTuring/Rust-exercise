/*
    有时候把所有不同的错误都视为一种错误类型会`简化`代码。
    我将用一个`自定义`错误类型来演示这一点。

    Rust 允许定义自己的错误类型。一般来说，一个 “好的” 错误类型应当：
        - 用同一个类型代表了多种错误;
        - 向用户提供了清楚的错误信息;
        - 能够容易地与其他类型比较:
            - 好的例子：Err(EmptyVec)
            - 坏的例子：Err("Please use a vector with at least one element".to_owned())
        - 能够容纳错误的具体信息:
            - 好的例子：Err(BadChar(c, position))
            - 坏的例子：Err("+ cannot be used here".to_owned())
        - 能够与其他错误很好地整合
*/
#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};
    use std::error::Error;

    // Result别名
    type Result<T> = std::result::Result<T, MichaelError>;

    // 开始定义`自定义的错误类型`，这种类型可以根据错误处理的实际情况定制。
    // 我们可以完全自定义错误类型，也可以在类型中完全采用底层的错误实现，
    // 也可以介于二者之间。
    #[derive(Debug)]
    struct MichaelError;        // 自定义错误类型

    // Error的生成与它如何显示是完全没关系的。
    // 没有必要担心复杂的逻辑会导致混乱的显示。
    // 注：我没有储存关于错误的任何额外信息，（即MichaelError这个struct并没有成员）
    // 也就是说，如果不修改我们的错误类型定义的话，就无法指明是哪个字符串解析失败了

    // 为自定义错误类型实现 `std::fmt::Display` trait
    impl Display for MichaelError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "invalid first item to double")
        }
    }

    // 为 `MichaelError` 实现 `std::error::Error` trait，
    // 这样其他错误可以包裹这个错误类型
    // 注：Error trait 继承了 Debug和Display trait
    impl Error for MichaelError {}

    // 功能函数
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        // 返回值Result<i32>为类型别名，即 std::result::Result<i32, MichaelError>
        vec.first()
            // 把错误换成自定义的新类型
            .ok_or(MichaelError)
            .and_then(|num_str| num_str.parse::<i32>()
                // 将parse后得到的Result中的错误类型换成自定义的错误类型
                .map_err(|_| MichaelError)
                // 将i32扩大2倍
                .map(|num| num * 2)
            )
    }

    /*
    关于option的ok_or方法：
       pub fn ok_or<E>(self, err: E) -> Result<T, E> {
            match self {
                Some(v) => Ok(v),
                None => Err(err),
            }
        }
    功能：如果option是Some，则返回Ok(v)，
         如果option为None，则返回Err(自定义类型错误)
    总的看来就是一个option -> Result的过程，并且如果为None，错误类型换成自定义的错误类型。
    */

    fn print_result(result: Result<i32>) {
        match result {
            Ok(num) => println!("{}", num),
            Err(e) => println!("Error: {}", e),
        }
    }

    #[test]
    fn test_error_handling_multiple_error_types_define_an_error_type() {
        let numbers = vec!["10", "20", "30"];
        print_result(double_first(numbers));
        // 输出:20

        let strings = vec!["michael.w", "20", "30"];
        print_result(double_first(strings));
        // 输出:Error: invalid first item to double

        let empty = Vec::new();
        print_result(double_first(empty))
        // 输出:Error: invalid first item to double
    }
}