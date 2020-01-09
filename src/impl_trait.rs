/*
    impl trait是一个全新的语法
    看这样的一个例子：
        fn foo(num:u32) -> impl Iterator<Item=u32> {
            (0..n).map(|x| x*100 )
        }

    在写泛型函数的时候，参数传递方式可以有两种:
    1.静态分派：
        fn consume_iter_static<I:Iterator<Item=u8>>(iter:I)
    2.动态分派：
        fn consume_iter_dynamic(iter:Box<dyn Iterator<Item=u8>>)

    不论选用哪种方式，都可以写出针对一组类型的抽象代码，而不是针对某一个具体类型的。

    在静态分派版本中:每次调用的时候，编译器都会为不同的实参类型实例化不同版本的函数;
    在动态分派版本中:每次调用的时候，实参的具体类型隐藏在trait object后面，通过虚函数表，在执行阶段选择调用正确的函数版本。

    上面这两种方式在做`函数参数`时都可以正常使用，但是做函数返回值时，只有动态分派是合法的：
    fn produce_iter_dynamic() -> Box<dyn Iterator<Item=u8>>
    静态分派是不合法的：
    fn produce_iter_static() -> Iterator<Item=u8>

    目前版本中， Rust只支持返回“具体类型”，而不能返回一个 trait。
*/
#[cfg(test)]
mod tests {
    // Rust函数无法直接返回一个闭包。
    // 因为闭包的类型是编译器自动生成的一个`匿名类型`,无法在函数的返回类型中手工指定。
    // 所以返回一个闭包一定要 “装箱”到堆内存中，然后把胖指针返回回去，这样是有性能开销的 。

    fn multiply_v1(num: i32) -> Box<dyn Fn(i32) -> i32> {
        // 闭包装箱
        Box::new(move |x| x * num)
    }

    #[test]
    fn test_impl_trait_v1() {
        // 生成一个闭包
        let f = multiply_v1(10);
        assert_eq!(50, f(5));
    }

    // 注意：引入一个泛型参数代表这个闭包是不行的(不用Box)
    /*
    fn multiply_v2<T>(num: i32) -> T
        where T: Fn(i32) -> i32 {
        // multiply_v2函数编译直接出错： expected type parameter, found closure
        move |x| x * num
    }
    */

    #[test]
    fn test_impl_trait_v2() {
        // 编译出错：error[E0277]: the size for values of type `dyn std::ops::Fn(i32) -> i32` cannot be known at compilation time
//        let f: dyn Fn(i32) -> i32 = multiply_v2(10);
    }

    /*
        为了解决上面的问题， 提出了impl trait这个方案。
        此方案引人了一个新的语法，可以表达一个不用装箱的匿名类型，
    */
    fn multiply_v3(num: i32) -> impl Fn(i32) -> i32 {
        move |x| x * num
    }

    /*
        这里的impl Fn(i32)->i32 表示：
        这个返回类型，虽然我们不知道它的具体名字(匿名标量)，
        但是知道它满足 Fn(size)->isize 这个 trait 的约束 。

        impl trait与泛型函数的主要区别是：
        泛型函数的类型参数是函数的`调用者`指定的 ;
        impl trait 的具体类型是函数的`实现体`指定的。
    */

    // 用trait实现类作为函数返回值类型例子
    #[test]
    fn test_impl_trait_v3() {
        let f = multiply_v3(10);
        assert_eq!(50, f(5));// 编译通过并运行
    }

    trait Trait1 {
        fn trait1_func(&self);
    }

    struct S;

    impl S {
        fn new() -> impl Trait1 {
            S {}
        }
    }

    impl Trait1 for S {
        fn trait1_func(&self) {
            println!("trait1_func() is invoked");
        }
    }


    #[test]
    fn test_impl_trait_v4() {
        let tmp = S::new();
        tmp.trait1_func();
    }

    /* 注意！！！！！！！！！！
        不要过于激进地使用这个功能：
            如在每个可以使用 impl trait 的地方都用它替换原来的具体类型 。
            它更多地倾向于简洁性，而牺牲了一部分表达能力 。

        正常的复杂写法：
            fn test() -> Chain<Map<...>>               (第一种)
        可能处于简洁考虑，写成：
            fn test() -> impl Iterator<Item=u16>         (第二种)
        问题出现了，第二种方法的返回值只能调用Iterator的方法，
        其他的Chain<Map<...>>实现的trait的方法都不可以再调用了。
        如果希望支持clone，第二种方法需要像下面这样改进：
            fn test() -> impl Iterator<Item=u16> + Clone

        在某些场景下， 需要罗列出各种接口才能`完整替代`原来的写法，类似下面这样:
            fn test() -> impl Iterator<Item=u16> +
                Clone +
                ExactSizeIterator +
                TrustedLen


        先不管这种写法是否可行，单说这个复杂程度，就已经完全失去了 impl trait功能的意义了。
        所以，什么时候该用这个功能，什么时候不该用，应该仔细权衡一下。
    */
}


