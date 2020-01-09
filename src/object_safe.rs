/*
    trait object的构造是受到许多约束的，当这些约束条件不能满足的时候，会产生编译错误。
    那么在哪些条件下trait object是无法构造出来的呢？见下：
*/

#[cfg(test)]
mod tests {
    //  1. 当trait有Self:Sized约束时：
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
}