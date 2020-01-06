/*
    Result 是 Option 类型的更丰富的版本，描述的是`可能的错误`而不是可能的`不存在`。

    也就是说，Result<T，E> 可以有两个结果的其中一个：
        - Ok<T>：找到 T 元素
        - Err<E>：找到 E 元素，E 即表示错误的类型。
        按照约定，预期结果是 “Ok”，而意外结果是 “Err”。

    Result 有很多类似 Option 的方法。
    例如: unwrap()，它要么举出元素 T，要么就 panic。
    对于事件的处理，Result 和 Option 有很多相同的组合算子。

    在使用 Rust 时，你可能会遇到返回 Result 类型的方法，例如: parse() 方法。
    它并不是总能把字符串解析成指定的类型，所以 parse() 返回一个 Result 表示可能的失败。
*/

// 我们来看看当 parse() 字符串成功和失败时会发生什么：
#[cfg(test)]
mod tests {
    fn multiply(first_num_str: &str, second_num_str: &str) -> i32 {
        // 试着用 `unwrap()` 把数字parse出来。它会咬我一口吗？
        // 泛型函数的显式调用
        let first_num = first_num_str.parse::<i32>().unwrap();
        let second_num = second_num_str.parse::<i32>().unwrap();
        first_num * second_num
    }

    #[test]
    fn test_error_handling_result() {
        // 一个正确的过程
        println!("{}", multiply("11", "12"));

        // 一个报错的过程:"1a" parse 会失败
        println!("{}", multiply("1a", "12"));
        // panic报错：'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }'

        // 在失败的情况下，parse() 产生一个错误，留给 unwrap() 来解包并产生 panic。
        // 另外，panic 会退出我们的程序，并提供一个让人很不爽的错误消息。

        // 为了改善错误消息的质量，我们应该更具体地了解返回类型并考虑显式地处理错误。
    }
}