/*
    “关联项”（associated item）指与多种类型的项有关的一组规则。
    它是 trait 泛型的扩展，允许在 trait 内部定义新的项。

    一个这样的项就叫做一个关联类型。
    当实现了trait的容器类型支持泛型时，关联项提供了简单的使用方法。

    注：「关联项」这个说法实际上只在 RFC 里出现了，
    官方的《The Rust Programming Language》第一版和第二版都只有「关联类型」的说法。
    如果觉得这里的说法很别扭的话不要理会就是了。
    TRPL 对关联类型的定义是：「一种将类型占位符与 trait 联系起来的做法，这样 trait 中的方法签名中就可以使用这些占位符类型。
    trait 的实现会指定在该实现中那些占位符对应什么具体类型。」
*/

/*
    trait 如果对实现了它的容器类型是泛型的，
    则须遵守类型规范要求，即：trait 的使用者必须指出 trait 的全部泛型类型。

    在下面例子中，Contains trait 允许使用泛型类型 A 或 B。
    然后为 Container 类型实现了这个 trait，将 A 和 B 指定为 i32，
    这样就可以对它们使用 difference() 函数。

    因为 Contains 是泛型的，我们必须在 fn difference() 中显式地指出所有的泛型类型。
    但实际上，A 和 B 究竟是什么类型是由输入 C 决定的。
    后面会看到，关联类型恰好提供了这样的功能。
*/

#[cfg(test)]
mod tests {
    struct Container(i32, i32);

    // 定义trait。
    // 这个 trait 能检查给定的 2 个项是否储存于容器中
    // 并且能够获得容器的第一个或最后一个值。
    trait Contains<A, B> {
        // 显式地需要 `A` 和 `B`
        fn contains(&self, _: &A, _: &B) -> bool;
        // 未显式地需要 `A` 或 `B`
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        // 如果存储的数字和给定的相等则为真
        fn contains(&self, n1: &i32, n2: &i32) -> bool {
            (&self.0 == n1) && (&self.1 == n2)
        }

        // 得到第一个数字
        fn first(&self) -> i32 {
            self.0
        }

        // 得到最后一个数字
        fn last(&self) -> i32 {
            self.1
        }
    }

    // 容器 `C` 就包含了 `A` 和 `B` 类型。difference函数的参数就是容器C。
    // 鉴于此，还需要重新指出 `A` 和 `B` 显得很麻烦。
    // (where子句中有用到A和B,所以在函数名后要显式写上A和B)
    fn difference<A, B, C>(container: &C) -> i32 where
    // where子句
        C: Contains<A, B> {
        container.last() - container.first()
    }

    #[test]
    fn test_generics_associated_items_problem() {
        let (n1, n2) = (10, 20);
        let container = Container(n1, n2);
        println!("{}", container.contains(&n1, &n2));
        println!("{}", container.contains(&10, &20)); // true
        println!("{}", container.first());
        println!("{}", container.last());

        println!("{}", difference(&container)); // 20-10=10
    }
}