/*
    错误处理（error handling）是处理可能发生的失败情况的过程。
    比如：读取一个文件失败了。如果继续使用这个无效的输入，那显然是有问题的。
         注意到错误的产生并且显式地处理这种错误，可以避免程序的其他部分产生潜在的问题。

    在 Rust 中有多种处理错误的方式，但是它们之间多少有些区别。
    使用场景也不尽相同。总的来说：

        1.显式的 panic 主要用于测试，以及处理不可恢复的错误。在原型开发中这很有用。
          比如：用来测试还没有实现的函数，不过这时使用 unimplemented 更能表达意图。
               另外在测试中，panic 是一种显式地失败（fail）的好方法。
        2.Option 类型是为了值是`可选`的、或者`缺少`值并不是错误的情况准备的。
          比如：寻找父目录时，/ 和 C: 这样的目录就没有父目录，这应当并不是一个错误。
          当处理 Option 时，unwrap 可用于原型开发，也可以用于能够确定 Option 中一定有值的情形。
          然而 expect 更有用，因为它允许你指定一条错误信息，以免万一还是出现了错误。
        3.当错误有可能发生且应当由调用者处理时，使用 Result。
          也可以 unwrap ,然后使用 expect。
          但是除了在测试或者原型开发中，请不要这样做。
*/

/*
    最简单的错误处理机制就是 panic。
    它会打印一个错误消息，开始回退（unwind）任务，且通常会退出程序。
    这里显式地在错误条件下调用 panic
*/

#[cfg(test)]
mod tests {
    fn occurs_panic(mark: &str) {
        if mark == "panic" {
            // 显式调用panic
            panic!("here is a panic!")
        }
        println!("there is no panic!")
    }

    #[test]
    fn test_error_handling_panic() {
        occurs_panic("michael.w");
        occurs_panic("panic");
    }
}