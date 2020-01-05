/*
    在使用泛型时，类型参数常常必须使用 trait 作为约束（bound）来明确规定类型应实现哪些功能。

    下面例子：
    用到了 Display trait 来打印，所以它用 Display 来约束 T
    也就是说 T 必须实现 Display:

    fn printer<T: Display>(t: T) {
    println!("{}", t);
}
*/

/*
    约束把泛型类型限制为符合约束的类型:

    struct S<T: Display>(T);
    // 报错！`Vec<T>` 未实现 `Display`。此次泛型具体化失败。
    let s = S(vec![1]);
*/

//  约束的另一个作用是泛型的实例可以访问作为约束的 trait 的方法。例如：
#[cfg(test)]
mod tests {
    // 这个 trait 用来实现打印标记：`{:?}`。
    use std::fmt::Debug;

    // 新建一个trait
    trait HasArea {
        fn area(&self) -> f64;
    }

    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    // 泛型 `T` 必须实现 `Debug` 。
    // 只要满足这点，无论什么类型都可以让下面函数正常工作。
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    // `T` 必须实现 `HasArea`。
    // 任意符合该约束的泛型的实例都可访问 `HasArea` 的 `area` 函数
    fn area<T: HasArea>(t: &T) -> f64 {
        t.area()
    }

    #[test]
    fn test_generics_bounds() {
        let rectangle = Rectangle { length: 10.24, height: 10.0 };
        print_debug(&rectangle);
        println!("{}", area(&rectangle));

        // 未实现HasArea
//        struct Rectangle1 {
//            length: f64,
//            height: f64,
//        }
//        let r1 = Rectangle1 { length: 1.1, height: 2.2 };
//        println!("{}", area(&r1));
        // 编译报错
    }
}