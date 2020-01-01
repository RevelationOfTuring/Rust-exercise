/*
    既然闭包可以作为参数，你很可能想知道函数是否也可以呢。
    确实可以！如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的trait约束的`函数`都可以作为其参数。
*/

#[cfg(test)]
mod tests {
    // 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
    fn call_func<F: Fn()>(f: F) {
        f()
    }
    /*
    等价于:

    fn call_func<F>(f: F) where
        F: Fn() {
        f()
    }

    */

    // 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
    fn function() {
        println!("function!!!")
    }

    #[test]
    fn test_function_closures_input_functions() {
        let closure = || println!("closure!!!");

        // 调用闭包
        call_func(closure);
        // 通过函数变量调用函数
        call_func(function);
    }
}