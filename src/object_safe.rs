/*
    trait object的构造是受到许多约束的，当这些约束条件不能满足的时候，会产生编译错误。
    那么在哪些条件下trait object是无法构造出来的呢？见下：
*/

#[cfg(test)]
mod tests {
    //  条件一：当trait有Self:Sized约束时：
    trait Foo where Self: Sized {
        fn foo(&self);
    }

    // 让i32实现trait Foo
    impl Foo for i32 {
        fn foo(&self) {
            println!("{}", self);
        }
    }

    #[test]
    fn test_object_safe_v1() {
        let x = 1024;
        // i32变量可以调用Foo trait中的方法foo
        x.foo();
        // 那试试trait object
        // 此处编译器报错：the trait `object_safe::tests::Foo` cannot be made into an object
//        let p = &x as &dyn Foo;
//        p.foo();

        /*
            所以，如果不希望一个trait可以通过trait object的方式使用，可以为它加上 Self: Sized 约束 。
        */
    }

    // 同理，如果我们想阻止一个函数在虚函数表中出现，可以专门为该函数加上 Self: Sized 约束:
    trait Foo1 {
        // trait Foo1中有两个方法
        fn foo1(&self);
        fn foo2(&self) where Self: Sized;
    }

    impl Foo1 for i32 {
        fn foo1(&self) {
            println!("foo1() is invoked: {}", self);
        }

        fn foo2(&self) where Self: Sized {
            println!("foo2() is invoked: {}", self);
        }
    }

    #[test]
    fn test_object_safe_v2() {
        let x = 1024;
        // 主要关注foo2方法，因为其被 Self:Sized 修饰
        x.foo2();
        // 用trait object的方式来调用一下
        let p = &x as &dyn Foo1;    // 这步可以编译通过，因为Foo1并未被 Self:Sized 修饰
        p.foo1();
        // 此处编译报错：error: the `foo2` method cannot be invoked on a trait object
//        p.foo2();

        /*
           所以对trait中的方法添加 Self: Sized 约束后，就不能通过 trait object 来调用这个函数了 。
        */
    }


    //  条件二：当trait有Self:Sized约束时：
    /*
        Self类型是个很特殊的类型，它代表的是impl这个trait的当前类型。
        比如说， Clone 这个 trait中的 clone方法就返回了一个 Self类型:
            pub trait Clone {
                fn clone(&self) -> Self;
                fn clone_from(&mut self,source:&Self) {...}
            }

        假如创建了一个Clone trait的trait object，并调用它的clone方法。那么返回值是什么类型呢？
        let p : &dyn Clone = if xxx { &obj1 as &dyn Clone} else { &obj2 as &dyn Clone};
        p到底是什么类型？
        编译器不知道，因为它在编译阶段元法确定。编译器在编译阶段仅仅知道：该类型实现了 Clone trait。
        所以，std::clone::Clone 这个trait就不是 object safe的。
        不能利用 dyn Clone 构造 trait object 来实现虚函数调用。
    */

    //    Rust规定，如果函数中除了self这个参数之外，还在`其他参数`或者`返回值`中用到了Self类型， 那么这个函数就不是 object safe 的。
    // 看下面的例子：定义一个trait Double
    trait Double {
        fn new() -> Self;
        fn double(&mut self);
    }

    impl Double for i32 {
        fn new() -> Self { 0 }

        fn double(&mut self) { *self *= 2 }
    }

    #[test]
    fn test_object_safe_v3() {
        let mut x = 1024;
        // 下面这步编译期直接报错：error[E0038]: the trait `std::clone::Clone` cannot be made into an object
//        let p = &mut x as &mut dyn Clone;

        /*
            因为 Double trait 中的new()方法是不满足object safe条件的。

            我只是想在 trait object 中调用 double 方法，不调用new 方法，该怎么办？
            把new方法用 Self:Sized修饰。
        */
    }

    trait Double1 {
        fn new() -> Self where Self: Sized;
        // 编译器就不会在生成虚函数表的时候考虑new方法
        fn double(&mut self);
    }

    impl Double1 for i32 {
        fn new() -> Self where Self: Sized { 0 }

        fn double(&mut self) { *self *= 2 }
    }

    #[test]
    fn test_object_safe_v4() {
        let mut x = 1024;
        let p = &mut x as &mut dyn Double1;
        p.double();
        println!("{}", x);  // 编译通过，x自身扩大了2倍
    }
}