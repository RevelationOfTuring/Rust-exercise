/*
    约束

    就如`泛型类型`能够被约束一样，生命周期（它们本身就是泛型）也可以使用约束。
    `:`字符的意义在这里稍微有些不同，不过 + 是相同的。注意下面的说明：
        -  `T:'a` : 在 T 中的所有引用都必须比生命周期'a活得更长
        -  `T:Trait + 'a` : T 类型必须实现 Trait trait，并且在 T 中的所有引用都必须比'a活得更长
*/

#[cfg(test)]
mod tests {
    use std::fmt::Debug;   // 用于约束的 trait

    // struct tuple
    // 里面成员是泛型类型，而且是引用
    #[derive(Debug)]
    struct Ref<'a, T: 'a>(&'a T);
    /*
        Ref包含一个指向泛型类型T的引用，其中T拥有一个未知的生命周期'a。
        T拥有生命周期限制：T中的任何`引用`都必须比'a活得更长
        Ref的生命周期也不能超出'a
    */

    // 一个泛型函数，使用 Debug trait 来打印内容
    fn print<T>(t: T) where
        T: Debug {
        println!("print: {:?}", t);
    }
    /*
        等价于：
        fn print<T: Debug>(t: T) {
            println!("print: {:?}", t);
        }
    */

    // 这里接受一个指向T的引用，其中T实现了Debug trait。
    // 并且在T中的所有引用都必须比'a存活时间更长。
    // 另外，'a也要比函数活得更长。
    fn print_ref<'a, T>(t: &'a T) where
        T: Debug + 'a {
        println!("print_ref: {:?}", t);
    }

    /*
        等价于：
        fn print_ref<'a, T: Debug + 'a>(t: &'a T) {
            println!("print_ref: {:?}", t);
        }
    */

    #[test]
    fn test_bounds() {
        let x = 1024;
        let ref_x = Ref(&x);

        print_ref(&ref_x);
        print(ref_x);
    }
}