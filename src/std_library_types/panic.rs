/*
    panic!

    Rust中panic!（宏）可用于产生一个 panic，并开始回退（unwind）它的栈。
    在回退栈的同时，runtime将会释放该线程所拥有的所有资源，这是通过调用线程中所有对象的析构函数完成的。
*/

#[cfg(test)]
mod tests {
    // 构造一个简单的除数为0的panic
    fn div(dividend: f64, divisor: f64) -> f64 {
        if divisor == 0.0 {
            panic!("divisor is Zero!");
        } else {
            dividend / divisor
        }
    }

    // 出于对panic是否会调用析构函数的实验
    struct S<'a> {
        name: &'a str,
    }

    // 向S的析构函数中加逻辑
    impl<'a> Drop for S<'a> {
        fn drop(&mut self) {
            println!("the object of struct S is dropping!")
        }
    }

    #[test]
    fn test_panic() {
        // 首先在堆上创建一个对象
        let s = Box::new(S { name: "Michael.W" });
        assert_eq!(s.name, "Michael.W");

        // 运行除法，触发panic
        div(1024.0, 0.0);

        // 永远无法执行到该语句
        println!("never reaches here!")

        // 运行test_panic(), 观察panic后堆上空间有无通过调用析构函数来释放
        /*
            打印输出：
                divisor is Zero!
                thread 'std_library_types::panic::tests::test_panic' panicked at 'divisor is Zero!', src/std_library_types/panic.rs:13:13
                stack backtrace:
                   0: backtrace::backtrace::libunwind::trace
                             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88

                   ...

                  22: test::run_test::run_test_inner::{{closure}}
                             at src/libtest/lib.rs:473
                note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
                the object of struct S is dropping!

            最后一句表明程序panic后，堆上的资源确实是通过调用对应的析构函数来释放。
        */
    }
}