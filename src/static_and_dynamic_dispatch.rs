/*
    如何用Rust实现多态？
    Rust 可以同时支持“静态分派”(static dispatch)和“动态分派”(dynamic dispatch)。

    静态分派：指具体调用哪个函数，在编译阶段就确定下来了。
    Rust中的“静态分派”靠泛型以及impl trait来完成:
    对于不同的泛型类型参数，编译器会生成不同版本的函数，在编译阶段就确定好了应该调用哪个函数。

    动态分派：具体调用哪个函数，在执行阶段才能确定。
    Rust中的“动态分派” 靠Trait Object来完成 。
    Trait Object本质上是指针，它可以指向不同的类型：指向的具体类型不同，调用的方法也就不同。
*/


#[cfg(test)]
mod tests {
    //  假设有一个trait Bird，有另外两个类型都实现了这个trait，我们要设计一个函数，既可以接受 Duck 作为参数，也可以接受 Swan 作为参数 。

    trait Bird {
        fn fly(&self);
    }

    // 两个实现类
    struct Duck;

    impl Bird for Duck {
        fn fly(&self) {
            println!("Duck flies!")
        }
    }

    struct Swan;

    impl Bird for Swan {
        fn fly(&self) {
            println!("Swan flies!")
        }
    }

    #[test]
    fn test_static_and_dynamic_dispatch_v1() {
        /*
            首先，要牢牢记住的一件事情:trait是一种DST类型，它的大小在编译阶段是不固定的。
            这意味着下面这样的代码是无法编译通过的:
            fn test(arg:Bird) {}
            fn test() -> Bird {}

            Rust编译器是不允许直接使用 trait作为参数类型和返回类型的。这也是trait跟许多语言中的“interface” 的一个区别 。
            有两种选择。一种是利用泛型:
            fn test<T:Bird>(arg:T) {
                arg.fly()
            }
        */

        fn test<T: Bird>(arg: T) {
            arg.fly();
        }

        let duck = Duck {};
        test(duck);

        let swan = Swan {};
        test(swan);

        /*
            所以 ，通过泛型函数实现的“多态”
            是在`编译阶段`就已经确定好了调用哪个版本的函数，因此被称为“静态分派” 。
        */
    }

    #[test]
    fn test_static_and_dynamic_dispatch_v2() {
        /*
            另一种实现多态的方式：
            我们还有另外一种办法来实现“多态”，那就是通过指针。
            虽然trait是DST类型，但是指向 trait 的指针不是DST。

            如果把trait隐藏到指针的后面，那它就是一个trait object,
            而它是可以作为`参数`和`返回类型`的 。
            根据不同需求，可以用不同的指针类型，如 Box/& /&mut 等

            fn test(arg:Box<dyn Bird>) {
                arg.fly()
            }
        */

        fn test(arg: Box<dyn Bird>) {
            arg.fly()
        }

        let duck = Box::new(Duck {});
        test(duck);
        // 也可以使用关键box，等同Box::new()
        // 但是stable的编译器将报错，目前box关键字不是很稳定，还是推荐使用
        let swan = Box::new(Swan {});
        test(swan);

        /*
            test函数的参数既可以是Box<Duck>类型，也可以是Box<Swan>类型， 一样实现了 “多态”。
            但在参数类型这里已经将“具体类型”信息抹掉了 ，我们只知道它可以调用 Bird trait 的方法。
            而具体调用的是哪个版本的方法，实际上是由这个指针的值来决定的。
            这就是“动态分派” 。
        */
    }
}