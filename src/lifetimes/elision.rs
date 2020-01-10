/*
    省略

    有些生命周期的模式太常用了，所以借用检查器将会隐式地添加它们以减少程序输入量和增强可读性。
    这种隐式添加生命周期的过程称为省略（elision）。
    在 Rust 使用`省略`仅仅是因为这些模式太普遍了。
*/

#[cfg(test)]
mod tests {
    // fn elided_input(x: &i32)的生命周期会被编译器自动添加。
    fn elided_input(x: &i32) {
        println!("`elided_input`: {}", x)
    }

    // 编译报错：error[E0428]: the name `elided_input` is defined multiple times
//    fn elided_input<'a>(x: &'a i32) {
//        println!("`annotated_input`: {}", x)
//    }

    // 注：
    //         fn elided_input(x: &i32) 和 fn elided_input<'a>(x: &'a i32)
    //      拥有相同的签名。

    // 换个函数名
    fn annotated_input<'a>(x: &'a i32) {
        println!("`annotated_input`: {}", x)
    }

    // 类似地，`elided_pass` 和 `annotated_pass` 也拥有相同的签名，
    // 生命周期会被隐式地添加进 `elided_pass`：
    fn elided_pass(x: &i32) -> &i32 { x }

    fn annotated_pass<'a>(x: &'a i32) -> &'a i32 { x }

    #[test]
    fn test_elision() {
        let x = 1024;

        elided_input(&x);
        // 等价于：
        annotated_input(&x);

        println!("`elided_pass`: {}", elided_pass(&x));
        // 等价于：
        println!("`annotated_pass`: {}", annotated_pass(&x));
    }
}