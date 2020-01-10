/*
    函数

    排除省略（elision）的情况，带上生命周期的`函数签名`有一些限制：
        - 任何引用都必须拥有标注好的生命周期
        - 任何被返回的引用都必须拥有和某个输入量相同的生命周期或是静态类型（static）

    另外要注意，如果没有输入的函数返回引用，有时会导致返回的引用指向无效数据。
    这种情况下禁止它返回这样的引用。
    下面例子展示了一些合法的带有生命周期的函数。
*/
#[cfg(test)]
mod tests {
    // 一个拥有生命周期 `'a` 的输入引用
    // 其中 `'a` 的存活时间至少与函数的一样长。
    fn print_one<'a>(x: &'a i32) {
        println!("`print_one`: x is {}", x);
    }

    // 可变引用同样也可能拥有生命周期
    fn double<'a>(x: &'a mut i32) {
        *x *= 2
    }

    // 拥有不同生命周期的多个元素。
    // 对下面这种情形，两者即使拥有相同的生命周期 `'a` 也没问题。
    // 但对一些更复杂的情形，可能就需要不同的生命周期了。
    fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
        println!("`print_multi`: x is {}, y is {}", x, y);
    }

    // 返回传递进来的引用也是可行的。
    // 但必须返回正确的生命周期。
    fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
        x
    }

    /*
        // 下面函数是无效的：`'a` 存活的时间必须比函数的长
        fn invalid_output<'a>() -> &'a String {
            // 创建一个String对象，然后取引用
            &String::from("michael.w")
            // String对象在离开函数invalid_output时会被释放掉
            // 这时返回的引用将指向无效数据
            // 编译会报错：error[E0515]: cannot return reference to temporary value
        }
    */

    #[test]
    fn test_functions() {
        let (x, y) = (1024, 2048);
        print_one(&x);
        print_multi(&x, &y);

        let z = pass_x(&x, &y);
        print_one(&z);

        let mut m = 4096;
        double(&mut m);
        print_one(&m);
    }
}