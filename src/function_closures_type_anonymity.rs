/*
    闭包从周围的作用域中捕获变量是简单明了的。
    这样会有某些后果吗？确实有。
    观察一下使用闭包作为函数参数，这要求闭包是`泛型`的，闭包定义的方式决定了这是必要的。

// `F` 必须是泛型的。
fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}

    当闭包被定义，编译器会隐式地创建一个`匿名类型的结构体`，用以储存闭包捕获的变量。
    同时为这个未知类型的结构体实现函数功能，通过Fn、FnMut或FnOnce三种trait中的一种。

    若使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是泛型的。
    然而，使用未限定类型的参数 <T> 过于不明确，并且是不允许的。
    事实上，指明为该结构体实现的是Fn、FnMut或FnOnce中的哪种trait，对于约束该结构体的类型而言就已经足够了。
*/

#[cfg(test)]
mod tests {
    // `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `closure_print` 的要求恰好一样。
    fn apply<F>(f: F) where
        F: Fn() {
        f();
    }

    #[test]
    fn test_function_closures_type_anonymity() {
        let x = 1024;

        // 捕获`x`到匿名类型中，并为它实现 `Fn`。
        // 将闭包存储到`closure_print`中。
        let closure_print = || println!("{}", x);
        apply(closure_print);
    }
}