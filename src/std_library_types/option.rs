/*
    Option

    有时候想要捕捉到程序某部分的失败信息，而不是直接调用 panic!
    这时可使用 Option 枚举类型来实现。

    Option<T> 有两个变量：
        - None，表明失败或缺少值
        - Some(value)，元组结构体，封装了一个 T 类型的值 value
*/

#[cfg(test)]
mod tests {
    // 简单模拟一个不会panic掉的整数除法
    // 返回值为Option枚举类型，而不直接是商
    fn divide(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            // 失败表示成 `None`
            None
        } else {
            // 商被包装到 `Some` 中
            Some(dividend / divisor)
        }
    }

    // 构造一个用来处理可能发生失败的divide
    fn try_division(dividend: i32, divisor: i32) {
        // 模式解构，检查除法结果
        match divide(dividend, divisor) {
            Some(quotient) => println!("{}/{}={}", dividend, divisor, quotient),
            None => println!("{}/{} failed", dividend, divisor),
        }
    }

    #[test]
    fn test_option() {
        // 正常除法
        try_division(1024, 2);

        // 错误除法,但是不会panic
        try_division(1024, 0);

        // Option<T> 在使用时的一些注意事项：
        // 1.绑定 `None` 到一个变量需要显式`类型`标注
        //   编译报错：error[E0282]: type annotations needed for `std::option::Option<T>`
//        let none = None;
        let none: Option<i32> = None;
        //   等价于
        let none_equivalent = None::<i32>;
        assert_eq!(none, none_equivalent);

        // 解包Some,用unwrap()
        let some_float64 = Some(1024f64);
        assert_eq!(1024f64, some_float64.unwrap());

        // 解包None会引发 panic!
        // 编译期间检查不出来，触发panic：'called `Option::unwrap()` on a `None` value'
//        none_equivalent.unwrap();
    }
}