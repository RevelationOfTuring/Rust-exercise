/*
    虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在编写函数时，这种模糊写法是不允许的。
    当以闭包作为`输入参数`时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的。
    其受限制程度按以下顺序递减：
        - Fn：表示捕获方式为通过引用（&T）的闭包
        - FnMut：表示捕获方式为通过可变引用（&mut T）的闭包
        - FnOnce：表示捕获方式为通过值（T）的闭包

    注：顺序之所以是这样，是因为 &T 只是获取了不可变的引用，&mut T 则可以改变变量，T 则是拿到了变量的所有权而非借用。

-------------------------------------------------------------------------------------------------

      对闭包所要捕获的每个变量，编译器都将以满足使用需求的前提下尽量以限制最多的方式捕获。

      例如：用一个类型说明为 FnOnce 的闭包作为参数。
      这说明闭包可能采取 &T，&mut T 或 T 中的一种捕获方式，但编译器最终是根据所捕获变量在闭包里的使用情况决定捕获方式。
      这是因为如果能以移动的方式捕获变量，则闭包也有能力使用其他方式借用变量。

      注意：
        反过来就不再成立：如果参数的类型说明是 Fn，那么不允许该闭包通过 &mut T 或 T 捕获变量。
*/
#[cfg(test)]
mod tests {
    //    试着分别用一用 Fn、FnMut 和 FnOnce，看看会发生什么：

    // 该函数将闭包作为参数并调用它。
    fn apply<F>(f: F) where
    // 闭包没有输入值和返回值。
        F: FnOnce() {
        // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

        // 换成F:
        //      FnMut() 编译报错，因为闭包类型为FnMut时，无法获得捕获变量的所有权，所以`mem::drop(my_str2);`编译报错
        // 换成F:
        //      Fn() 编译报错，因为不允许该闭包通过 &mut T 或 T 捕获变量。所以`mem::drop(my_str2);`和`my_str2.push_str("ethereum");`编译报错
        f();
    }

    fn apply_to_3<F>(f: F) -> i32 where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
        F: Fn(i32) -> i32 {
        f(3)
    }

    #[test]
    fn test_function_closures_as_input_parameters() {
        use std::mem;

        let my_str1 = "michael.w";
        // my_str1为不可复制的类型。

        let mut my_str2 = "Bitcoin".to_owned();
        // `to_owned`从借用的数据创建有所有权的数据。

        // 捕获2个变量：通过引用捕获`my_str1`，通过值捕获 `my_str2`。

        let closure = || {
            // `my_str1`通过引用捕获，故需要闭包是 `Fn`
            println!("my_str1 : {}", my_str1);

            // 下面需要改变了 `my_str2` ，因而要求闭包通过可变引用来捕获它。
            // 现在需要 `FnMut`。
            my_str2.push_str("Ethereum");
            println!("my_str2 : {}", my_str2);

            // 手动调用 drop 又要求闭包通过值获取 `my_str2` 本体
            // 现在需要 `FnOnce`。
            mem::drop(my_str2);
        };

        // 以闭包作为参数，调用函数 `apply`。
        apply(closure);

        // 闭包 `closure_double` 满足 `apply_to_3` 的 trait 约束
        // apply_to_3的闭包参数约束： Fn(i32) -> i32
        let closure_double = |n| n * 10;
        // 等价于
//        let closure_double = |n: i32| -> i32 { n * 10 };
//        let closure_double: fn(i32) -> i32 = |n| n * 10;

        println!("{}", apply_to_3(closure_double)); // 30
    }
}