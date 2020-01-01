/*
    发散函数（diverging function）绝不会返回。 它们使用 ! 标记，这是一个空类型。
*/
#[cfg(test)]
mod tests {
    #[test]
    fn test_function_diverging_functions() {
        // 发散函数，如：
        fn foo() -> ! {
            panic!("This call never returns.");
        }

        // 和所有其他类型相反，这个类型无法实例化，因为此类型可能具有的所有可能值的集合为空。
        // 注意，它与 () 类型不同，()类型只有一个可能的值。
        // 如下面例子，虽然返回值中没有信息，但此函数会照常返回。

        fn some_fn() {
            ()
        }

        let a = some_fn();

        // 下面这个函数相反，这个函数永远不会将控制内容返回给调用者。
        /*
        let x: ! = panic!("This call never returns.");
        println!("You will never see this line!");
        */

        //        虽然这看起来像是一个抽象的概念，但实际上这非常有用且方便。
//        这种类型的主要优点是它可以被转换为`任何其他类型`，从而可以在需要精确类型的地方使用，
//        例如在 match 匹配分支。 这允许我们编写如下代码：
        fn sum_odd_numbers(up_limit: u32) -> u32 {
            // 求(0:up_limit)中的奇数和
            let mut acc = 0;
            for i in 0..up_limit {
                // 注意这个 match 表达式的返回值必须为 u32，
                // 因为 “addition” 变量是这个类型。
                let addition = match i % 2 == 1 {
                    // “i” 变量的类型为 u32，这毫无问题。
                    true => i,
                    // 另一方面，“continue” 表达式不返回u32，但它仍然没有问题，
                    // 因为它永远不会返回，因此不会违反匹配表达式的类型要求。
                    false => continue,
                };
                acc += addition;
            }
            // 返回值
            acc
        }

        println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
    }
}