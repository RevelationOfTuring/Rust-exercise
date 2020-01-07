/*
    前面出现的例子都是很方便的情况:
    都是 Result 和其他 Result 交互，还有 Option 和其他 Option 交互。

    有时 Option 需要和 Result 进行交互，或是 Result<T, Error1> 需要和 Result<T, Error2> 进行交互。
    在这类情况下，我们想要以一种方式来管理不同的错误类型，使得它们可组合且易于交互。

    在下面代码中，unwrap 的两个实例生成了不同的错误类型:
    Vec::first 返回一个 Option，而 parse::<i32> 返回一个 Result<i32, ParseIntError>：
*/

#[cfg(test)]
mod tests {
    fn double_first(vec: Vec<&str>) -> i32 {
        // 取出vec的第一个成员中的&str
        let first_str = vec.first().unwrap(); // 这可能产生panic，如果Option里面装的是None
        2 * first_str.parse::<i32>().unwrap()       // 这也可能产生panic，如果Result里面是Err
    }

    #[test]
    fn test_error_handling_multiple_error_types() {
        let numbers = vec!["10", "20", "30"];
        println!("{}", double_first(numbers));

        let strings = vec!["michael.w", "20", "30"];
        // 编译通过，但运行报错。因为parse返回Err
        println!("{}", double_first(strings));

        let empty = Vec::new();
        // 编译通过，但运行报错。因为vec.first().unwrap()返回None
        println!("{}", double_first(empty));
    }
}