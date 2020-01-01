/*
    返回闭包作为输出参数(output parameter)也应该是可能的。
    然而返回闭包类型会有问题，因为目前Rust只支持返回具体(非泛型)的类型。
    按照定义，匿名的闭包的类型是`未知`的，所以想要返回一个闭包只有使它变成`具体`的类型。
    通过box操作可以实现这点。

    -----------------------------------------------------------------------

    返回值的合法 trait 和前面的略有不同：
        - Fn：和前面的一样
        - FnMut：和前面的一样
        - FnOnce：不平常的事情正是发生在这里!总之现在你需要返回FnBox类型，目前该类型还是不稳定的。这个情况估计将来会改进。

    -----------------------------------------------------------------------
    除此之外，还必须使用move关键字，它表明所有的捕获都是通过`值`进行的。
    这是必须的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。
*/

#[cfg(test)]
mod tests {
    // 举个例子：
    fn create_fn() -> Box<dyn Fn()> {
        // 必须将text变量的所有权转到闭包中
        // 因为函数create_fn执行结束后变量text就将被析构，如果只是单纯地将借用传入闭包，
        // 闭包中将继续使用无效的引用
        let text = "Fn()".to_owned();
        Box::new(move || println!("{}", text))
    }

    fn create_fnmut() -> Box<dyn FnMut()> {
        let text = "FnMut()".to_owned();
        Box::new(move || println!("{}", text))
    }

    #[test]
    fn test_function_closures_as_output_parameters() {
        // 函数的返回值是闭包
        let closure_fn = create_fn();
        // closure_fnmut应该是一个可修改的闭包类型变量
        let mut closure_fnmut = create_fnmut();

        closure_fn();
        closure_fnmut();

        // 也可以直接调用
        create_fn()();
        create_fnmut()();
    }
}