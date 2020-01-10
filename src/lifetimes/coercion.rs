/*
    强制转换

    一个较长的生命周期可以强制转成一个较短的生命周期，使它在一个通常情况下不能工作的作用域内也能正常工作。
    强制转换可由编译器隐式地推导并执行，也可以通过声明不同的生命周期的形式实现。
*/

#[cfg(test)]
mod tests {
    // 在这里，Rust 推导了一个尽可能短的生命周期。
    // 然后这两个引用都被强制转成这个生命周期
    fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
        first * second
    }
    /*
        等同于：
        fn multiply(first: &i32, second: &i32) -> i32 {
            first * second
        }
    */

    // `<'a: 'b, 'b>` 读作生命周期 `'a` 至少和 `'b` 一样长
    // 在这里我们我们接受了一个 `&'a i32` 类型和`&'b i32`类型并返回一个 `&'b i32` 类型
    // first和second声明周期不同
    // 由于逻辑的设定，返回值可能为first或second。这时，必须强行将'a 和 'b 统一，作为返回值的生命周期
    // 返回值声明周期必须是'a和'b中最短的
    fn choose_max<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
        if first > second {
            first
        } else {
            second
        }
    }


    #[test]
    fn test_coercion() {
        // 较长生命周期
        let first = 1024;
        {
            // 较短生命周期
            let second = 10;
            println!("The product is {}", multiply(&first, &second));
            println!("{} is the max", choose_max(&first, &second));
            // 实参second和first换位置也可以
            println!("{} is the max", choose_max(&second, &first));
        }
    }
}