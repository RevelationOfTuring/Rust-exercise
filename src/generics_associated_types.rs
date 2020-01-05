/*
    通过把容器内部的类型放到 trait 中作为`输出类型`，
    使用 “关联类型” 增加了代码的可读性。
    这样的 trait 的定义语法如下：

    `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
    注：trait里的 `type` 不同于为类型取别名时的 `type`。

    当使用了关联类型后，Contains trait 的函数就不需要写出 A 或 B 了。
    (之前的基础代码见文件：generics_associated_items_problem.rs)

    // 不使用关联类型(需要写出A和B)
    fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

    // 使用关联类型(不需要写出A和B)
    fn difference<C: Contains>(container: &C) -> i32 { ... }
*/

// 利用关联类型重写generics_associated_items_problem.rs文件中的demo
#[cfg(test)]
mod tests {
    struct Container(i32, i32);

    // 这个 trait 检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains {
        // 在这里定义可以被方法使用的泛型类型。即关联类型
        type A;
        type B;

        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        // 得到第一个数字
        // (因为difference函数中涉及到container.last() - container.first()，
        // 所有在定义trait Contains时候first和last方法返回类型为i32，
        // 而不是Self::A和Self::B)
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains for Container {
        // 在impl中需要指出 `A` 和 `B` 是什么类型。
        // 如果 `input`（输入）类型为 `Container(i32, i32)`，
        // 那么 `output`（输出）类型会被确定为 `i32` 和 `i32`。
        type A = i32;
        type B = i32;

        fn contains(&self, n1: &Self::A, n2: &Self::B) -> bool {
            (n1 == &self.0) && (n2 == &self.1)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    // 这是不在需要将A和B的显式的写出来
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    #[test]
    fn test_generics_associated_types() {
        let (n1, n2) = (10, 20);
        let container = Container(n1, n2);
        println!("{}", container.contains(&n1, &n2));
        println!("{}", container.first());
        println!("{}", container.last());

        println!("{}", difference(&container));
    }
}
