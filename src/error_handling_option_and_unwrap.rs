/*
    在标准库（std）中有个叫做 Option<T>（选项）的枚举类型，
    用于有 “不存在” 的可能性的情况。
    它表现为以下两个 “option”（选项）中的一个：
        - Some(T)：找到一个属于 T 类型的元素
        - None：找不到相应元素

    这些选项可以通过 match `显式`地处理，或使用 unwrap `隐式`地处理。
    隐式处理要么返回 Some 内部的元素，要么就 panic。

    请注意，手动使用 expect 方法自定义 panic 信息是可能的，
    但相比显式处理，unwrap 的输出仍显得不太有意义。

    在下面例子中，显式处理将举出更受控制的结果，同时如果需要的话，仍然可以使程序 panic。
*/
#[cfg(test)]
mod tests {
    // 显式处理panic
    fn occurs_panic_explicit(mark: Option<&str>) {
        // 指出每种情况下的做法
        match mark {
            // 如果Option中装的是"panic"
            Some("panic") => panic!("here is a panic!"),
            // 如果Option中有东西且不是"panic"
            Some(i) => println!("{}", i),
            None => println!("there is nothing!")
        }
    }

    // 隐式处理panic
    fn occurs_panic_implicit(mark: Option<&str>) {
        // inner为Option中装载的&str
        let inner = mark.unwrap();
        // 注：如果mark中什么都没有，即装的是None，在unwrap的过程中会自动panic

        if inner == "panic" {
            panic!("here is a panic!")
        }

        println!("{}", inner);
    }

    #[test]
    fn test_error_handling_option_and_unwrap_explicit() {
        occurs_panic_explicit(Some("michael.w"));
        occurs_panic_explicit(None);
        // 此处会引发panic
        occurs_panic_explicit(Some("panic"))
    }

    #[test]
    fn test_error_handling_option_and_unwrap_implicit() {
        occurs_panic_implicit(Some("michael.w"));
        // 会panic
        occurs_panic_implicit(None);
        // 会panic
        occurs_panic_implicit(Some("panic"));
    }
}