/*
    当然 trait 也可以是泛型的。
    在这里定义了一个 trait，它把 Drop trait 作为泛型方法重实现了，可以 drop（丢弃）`调用者本身`和`一个输入参数`。
*/

#[cfg(test)]
mod tests {
    // 不可复制的类型。
    #[derive(Debug)]
    struct Empty;

    #[derive(Debug)]
    struct Null;

    // `T` 的泛型 trait
    trait DoubleDrop<T> {
        // 定义一个调用者的方法，接受一个额外的参数 `T`，但不对它做任何事
        fn double_drop(self, _: T);
    }

    // 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>`
    impl<T, U> DoubleDrop<T> for U {
        // 此方法获得两个传入参数的所有权，并释放它们
        fn double_drop(self, _: T) {}
        // 注：由于调用者类型是self，而不是&self
        // 所以在调用double_drop方法是U对象的所有权会转移到函数double_drop内，
        // 并在该函数结束时被释放掉
        // 同理参数`_:T`
    }

    #[derive(Debug)]
    struct A {
        i: i32,
    }

    #[test]
    fn test_generics_trait() {
        let empty = Empty;
        let null = Null;

        // 释放 `empty` 和 `null`
        empty.double_drop(null);

        //TODO:为什么empty对象可以直接调用double_drop，即struct Empty什么时候实现了trait DoubleDrop<T>

        // 已经释放掉了，无法再打印
//        println!("{:?}", empty);
//        println!("{:?}", null);

    }
}