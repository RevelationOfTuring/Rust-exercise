/*
    derive

    通过 #[derive] 属性，编译器能够提供某些 trait 的基本实现。
    如果需要更复杂的行为，这些 trait 也可以手动实现。

    以下是可以自动推导的 trait：
        - 比较trait: Eq, PartialEq, Ord, PartialOrd
        - Clone, 用来从 &T 创建副本 T。
        - Copy，使类型具有 “复制语义”（copy semantics）而非 “移动语义”（move semantics）。
        - Hash，从 &T 计算哈希值（hash）。
        - Default, 创建数据类型的一个空实例。
        - Debug，使用 {:?} formatter 来格式化一个值。
*/
#[cfg(test)]
mod tests {
    // `Test1`，可以进行比较的tuple struct
    // 因为实现了PartialEq, PartialOrd
    #[derive(PartialOrd, PartialEq)]
    struct Test1(f64);

    // `Test2`，可以进行打印的tuple struct
    // 因为实现了Debug
    #[derive(Debug)]
    struct Test2(f32);

    impl Test2 {
        // 向Test转换
        fn to_test1(&self) -> Test1 {
            // 模式匹配，取到struct tuple中的f32数字
            let &Test2(num) = self;
            Test1(num as f64)
        }
    }

    // `Test3`，不带附加属性的tuple struct
    struct Test3(i32);

    #[test]
    fn test_derive() {
        let test3 = Test3(1024);

        // 报错，因为Test3没有实现Debug trait
//        println!("{:?}", test3);

        println!("{}", Test1(1024f64) < Test1(1025f64));

        // 报错，因为Test2没有实现PartialOrd trait
//        println!("{}", Test2(1024f64) < Test2(1025f64));

        let test1 = Test1(2048f64);
        let test2 = Test2(1024f32);
        // test2.to_test1()的返回值为Test1，且Test1实现了PartialOrd trait
        println!("{}", test2.to_test1() < test1);   // 通过编译
    }
}