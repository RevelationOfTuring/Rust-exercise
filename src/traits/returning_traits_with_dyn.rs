/*
       dyn (2018版新关键字)
       Rust编译器要求明确地知道每个函数的返回值类型长度大小。
       这意味着函数的返回值必须是已实现类型(struct)。这点跟其他高级编程语言不一样。

       如果函数要求返回值类型为满足trait A,不能直接将A写成函数返回值类型。
       因为编译器无法知道A的实现类大小。

       如何解决上述问题？
       可以使用Box来包装trait，并作为函数的返回值。
       Box的本质就是堆上对象的引用，这个引用的长度大小编译器是已知的。

       由于Rust要求在堆上分配空间时，要尽可能的显式地标记出来。
       所以，当函数返回一个`堆上`引用Box时，必须加上关键字dyn，如：Box<dyn A>

       因为不加上dyn，无法知道A究竟是个trait还是个struct。

       注：impl 和 dyn 为 Rust 2018 edition 的两个新关键字。
*/
#[cfg(test)]
mod tests {
    struct Sheep;

    struct Cow;

    // 定义trait
    trait Animal {
        fn noise(&self) -> &'static str;
    }

    impl Animal for Sheep {
        fn noise(&self) -> &'static str {
            "baaaaaaaaah!"
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> &'static str {
            "moooooooooo!"
        }
    }

    // 返回值类型为Box
    fn random_animal(num: i32) -> Box<dyn Animal> {
        if num % 2 == 0 {
            Box::new(Sheep)
        } else {
            Box::new(Cow)
        }
    }

    #[test]
    fn test_returning_traits_with_dyn() {
        for i in 0..=10 {
            let animal = random_animal(i);
            println!("{}", animal.noise());
        }

        // 输出：
        //  baaaaaaaaah!
        //  moooooooooo!
        //  baaaaaaaaah!
        //  moooooooooo!
        //  baaaaaaaaah!
        //  moooooooooo!
        //  baaaaaaaaah!
        //  moooooooooo!
        //  baaaaaaaaah!
        //  moooooooooo!
        //  baaaaaaaaah!
    }
}