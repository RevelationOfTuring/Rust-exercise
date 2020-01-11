/*
    运算符重载

    在 Rust 中，很多运算符可以通过 trait 来`重载`。
    也就是说，这些运算符可以根据它们的`输入参数`的不同来完成不同的任务。

    这之所以可行，是因为`运算符`就是`方法调用`的语法糖。

    例如:
        a + b中的 + 运算符会调用add 方法（即，a.add(b)）。
        这个add方法是Add trait的一部分。因此，+ 运算符可以被任何 Add trait 的实现者使用。

    可重载运算符的trait：https://doc.rust-lang.org/core/ops/
*/

#[cfg(test)]
mod tests {
    use std::ops;
    use std::ops::Add;

    struct Test1;

    struct Test2;

    #[derive(Debug)]
    struct Test3;

    #[derive(Debug)]
    struct Test4;

    // `std::ops::Add` trait 用来指明 `+` 的功能
    // Add<Test1>，作用是：用于把实现类Test2(即加法公式的左操作数)的对象和泛型类型Test1(即加法公式的右操作数)加起来
    impl ops::Add<Test1> for Test2 {
        // 输出类型，即加法和的类型——Test3
        type Output = Test3;
        // 即实现 Test2+Test1=Test3 这样的运算
        fn add(self, rhs: Test1) -> Self::Output {
            println!("Test2.Add() is invoked");
            Test3
        }
    }

    // 再实现一个加法trait，使得Test1和Test2加和不满足加法交换律
    // Test1+Test2=Test4
    impl ops::Add<Test2> for Test1 {
        type Output = Test4;

        // Test1+Test2=Test4
        fn add(self, rhs: Test2) -> Self::Output {
            println!("Test1.Add() is invoked");
            Test4
        }
    }

    #[test]
    fn test_operator_overloading() {
        println!("{:?}", Test1 {} + Test2 {});
        // 等同于
        println!("{:?}", Test1 + Test2);

        println!("{:?}", Test2 {} + Test1 {});
        // 等同于
        println!("{:?}", Test2 + Test1);
    }
}