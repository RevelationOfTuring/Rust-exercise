/*
    Iterators

    Iterator trait 用来对集合（collection）类型（比如数组）实现迭代器。

    该trait只需程序员自己定义一个返回下一个元素的 next方法。
    这可手动在 impl 代码块中定义，或者自动定义（比如在数组或区间中）。

    为方便起见，for 循环会使用 .into_iterator() 方法将一些集合类型转换为`迭代器`。

    更多关于Iterators的用法：https://doc.rust-lang.org/core/iter/trait.Iterator.html
*/

#[cfg(test)]
mod tests {
    struct Fibonacci {
        current: u32,
        next: u32,
    }

    // 为 `Fibonacci`（实现 `Iterator`。
    // `Iterator` trait 只需要程序员定义一个能返回下一个元素的方法。
    impl Iterator for Fibonacci {
        // 下个元素的类型
        type Item = u32;

        // 返回类型为 `Option<T>`：
        //    - 当 `Iterator` 结束时，返回 `None`。
        //    - 其他情况，返回被 `Some` 包裹（wrap）的下一个值。
        // 注：next的返回类型一般都要设置成Option<T>，为了获知何时结束迭代。
        fn next(&mut self) -> Option<Self::Item> {
            let current = self.current;
            // 更新 Fibonacci对象，形成迭代逻辑
            self.current = self.next;
            self.next += current;

            // Fibonacci数列不存在终点，
            // 那么 `Iterator` 将不可能返回 `None`，而总是返回 `Some`。
            Some(current)
        }
    }

    // 生成一个Fibonacci数列生成器
    impl Default for Fibonacci {
        fn default() -> Self {
            Fibonacci { current: 1, next: 1 }
        }
    }

    #[test]
    fn test_iterators() {
        // 0..10 其实就是一个Iterator
        let mut sequence = 0..=10;
        assert_eq!(0, sequence.next().unwrap());
        assert_eq!(1, sequence.next().unwrap());
        assert_eq!(2, sequence.next().unwrap());
        assert_eq!(3, sequence.next().unwrap());

        // `for` 遍历 `Iterator` 直到返回 `None`，
        // 并且每个 `Some` 值都被解包（unwrap），然后绑定给一个变量（这里是 `i`）
        for i in 0..=10 {
            println!("{}", i)
        }

        println!("************************************");
        // Iterator的take(n)方法：
        // 作用：提取Iterator的前n项。
        for i in Fibonacci::default().take(5) {
            // 打印Fibonacci数列的前5项
            println!("{}", i)
        }

        println!("************************************");
        // Iterator的skip(n)方法：
        // 作用：移除前n项，从而缩短了Iterator。
        for i in Fibonacci::default().skip(4).take(4) {
            // 跳过Fibonacci数列的前4项，从第5项开始后面四个项
            println!("{}", i)
        }

        println!("************************************");
        // 创建一个数组
        let arr = [1, 2, 3, 4, 5];

        // Iterator的iter方法对数组/slice 产生一个Iterator
        for i in arr.iter() {
            println!("{}", i)
        }
    }
}