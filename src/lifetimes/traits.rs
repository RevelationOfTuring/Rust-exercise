/*
    trait

    trait 方法中生命期的标注基本上与函数类似。
    注意，impl 也可能有生命周期的标注
*/

#[cfg(test)]
mod tests {
    // 带有生命周期标注的结构体
    #[derive(Debug)]
    struct S<'a> {
        name: &'a str
    }

    // 给 impl 标注生命周期
    // 此处的生命周期是必须显式写出来的
    impl<'a> Default for S<'a> {
        fn default() -> Self {
            Self {
                name: "michael.w",
            }
        }
    }

    #[test]
    fn test_traits() {
        // 实现了Default trait后，必须在let后面显式给出类型，编译器才知道执行哪个类型的default方法。
        let s: S = Default::default();
        println!("s is {:?}", s);
    }
}