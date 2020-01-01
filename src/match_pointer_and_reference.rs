/*
对指针来说，解构（destructure）和解引用（dereference）要区分开，因为这两者的概念 是不同的，和 C 那样的语言用法不一样。

解引用使用 *
解构使用 &、ref、和 ref mut
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_match_pointer_and_reference() {
        // 获得一个 `i32` 类型的引用。`&` 表示取引用。
        let reference = &1024;

        match reference {
            // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
            // `&i32`（译注：即 `reference` 的类型）
            //    |
            // `&val`（译注：即用于匹配的模式）
            // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
            // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
            &val => println!("Got a value via destructuring: {:?}", val),
            // 匹配关系： &val <-> &i32
            // 所以 val就是一个i32
        }

        // 如果不想用 `&`，需要在匹配前解引用。
        // *reference此时就是一个i32
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        // 如果一开始就不用引用，会怎样？
        // 上面`reference` 是一个 `&` 类型，因为赋值语句的右边已经是一个引用。
        // 但下面这个不是引用，因为右边不是。
        let _not_a_reference = 2048;

        // Rust 对这种情况提供了 `ref`。它更改了赋值行为，从而可以对具体值创建引用。
        // 下面这行将得到一个引用。
        let ref _is_a_reference = 2048;

        // 相应地，定义两个非引用的变量，通过 `ref` 和 `ref mut` 仍可取得其引用。
        let value = 4096;
        let mut mut_value = 4096;

        // 使用 `ref` 关键字来创建引用。
        // 译注：下面的 r 是 `&i32` 类型，它像 `i32` 一样可以直接打印，因此用法上
        // 似乎看不出什么区别。
        // 但可以把 `println!` 中的 `r` 改成 `*r`，仍然能
        // 正常运行。前面例子中的 `println!` 里就不能是 `*val`，因为不能对整数解
        // 引用。
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        // 类似地使用 `ref mut`。
        match mut_value {
            ref mut m => {
                // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
                println!("We added 10. `mut_value`: {:?}", m);
                *m += 10;
            }
        }
        // mut_value显示已经改变成4106
        println!("{}", mut_value);
    }
}
