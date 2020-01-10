/*
    Rust中的`析构函数`概念是通过 Drop trait 提供的。
    当资源离开作用域，就调用析构函数。
    程序员`无需`为每种类型都实现 Drop trait，只要为那些需要自己的析构函数`逻辑`的类型实现就可以了。
*/
#[cfg(test)]
mod tests {
    struct S;

    impl Drop for S {
        fn drop(&mut self) {
            println!("struct S is dropped by its destructor!");
        }
    }

    #[test]
    fn test_destructor() {
        let x = S;
        println!("this is just a mark.");
    }
    // 注：打印为
    //    this is just a mark.
    //    struct S is dropped by its destructor!

    // 在离开test_destructor函数作用域后，x对象的析构函数被自动调用。
}