/*
    约束的工作机制会产生这样的效果：即使一个 trait 不包含任何功能，你仍然可以用它作为约束。
    标准库中的 Eq 和 Ord 就是这样的 trait。
*/
#[cfg(test)]
mod tests {
    struct Cardinal;

    struct BlueJay;

    struct Turkey;

    trait Red {}

    trait Blue {}

    impl Red for Cardinal {}

    impl Blue for BlueJay {}

    // 这些函数只对实现了相应的 trait 的类型有效。
    // 事实上这些 trait 内部是空的，但这没有关系。
    fn red<T: Red>(_: &T) -> &str { "red" }

    fn blue<T: Blue>(_: &T) -> &str { "blue" }

    #[test]
    fn test_generics_empty_bounds() {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        println!("{}", red(&cardinal));
        println!("{}", blue(&blue_jay));

//        let turkey = Turkey;
//        println!("{}", red(&turkey));
        // 编译错误
    }
}