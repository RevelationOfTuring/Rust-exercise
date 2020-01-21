/*
    测试实例：map-reduce

    Rust 使数据的并行化处理非常简单，在 Rust 中你无需面对并行处理的很多传统难题。

    标准库提供了`开箱即用`的线程类型，把它和 Rust 的所有权概念与别名规则结合起来，可以自动地避免`数据竞争（data race）`。

    当某状态对某线程是`可见`的，别名规则就自动地`避免`了别的线程对它的操作。即一个可变引用 XOR 一些只读引用。
    ps:XOR是异或，即「二者仅居其一」。

    当需要同步处理时，请使用 Mutex 或 Channel 这样的同步类型。
*/

/*
    在本例中，我们将会计算一堆数字中每一位的和。
    我们将把它们分成几块，放入不同的线程。
    每个线程会把自己那一块数字的每一位加起来，之后我们再把每个线程提供的结果再加起来。

    注意:虽然我们在线程之间传递了引用，但 Rust 理解我们是在传递`只读`的引用，因此不会发生数据竞争等不安全的事情。
    另外，因为我们把数据块 move 到了线程中，Rust 会保证数据存活至线程退出，因此不会产生`悬挂指针`。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_testcase_map_reduce() {
        // 将要处理的数据
        let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
        // 我们会通过线程实现 map-reduce 算法，从而计算每一位的和
        // 每个用空白符隔开的块都会分配给单独的线程来处理

        // 创建一个Vec，用于储存将要创建的子线程
        let mut thread_handlers = vec![];

        /************************************************************************
         * "Map" 阶段
         *
         * 把数据分段，并进行初始化处理
         ************************************************************************/

        // 把数据分段，每段将会单独计算
        // 每段都是完整数据的一个引用（&str）
        let chunked_data = data.split_whitespace();

        // 对分段的数据进行迭代。
        // enumerate方法会把当前的迭代计数与被迭代的元素以元组 (index, element)的形式返回。
        // 接着立即使用 “解构赋值” 将该元组解构成两个变量 `i` 和 `data_segment`。
        for (i, data_segment) in chunked_data.enumerate() {
            println!("data segment {} is \"{}\"", i, data_segment);

            // 每一段数据都用单独的线程处理
            //
            // std::thread::spawn方法返回新线程的句柄（handle）
            // 我们必须拥有句柄，才能获取线程的返回值。

            /*
                 move || -> u32' 语法表示该闭包：
                     * 没有参数（'||'）
                     * 会获取所捕获变量的所有权（'move'）
                     * 返回无符号 32 位整数（'-> u32'）
            */
            let handler = std::thread::spawn(
                move || -> u32{
                    // 计算该数据段的每一位的和
                    let result = data_segment
                        // 对该段中的字符进行迭代,即将&str转成chars（可以理解为一个char的迭代器）
                        .chars()
                        // 把字符转成十进制的数字
                        .map(|ch| ch.to_digit(10).expect("should be a digit"))
                        // 对返回的数字类型的迭代器求和
                        .sum();

                    // println! 会锁住标准输出，这样各线程打印的内容不会交错在一起。
                    // 即 println！是线程安全的
                    println!("processed segment {}, result={}", i, result);
                    result
                }
            );

            thread_handlers.push(handler);
        }

        /*************************************************************************
         * "Reduce" 阶段
         *
         * 收集中间结果，得出最终结果
         ************************************************************************/

        // 把每个线程的计算结果收入一个新的Vec中
        let mut thread_sums = vec![];
        for handler in thread_handlers {
            // 收集每个子线程的返回值
            let handler_result = handler.join().unwrap();
            thread_sums.push(handler_result);
        }

        // 把所有中间结果加起来，得到最终结果

        // 可以使用"turbofish"写法:`::<>` 来为sum()提供类型提示
        let final_sum_1 = thread_sums.iter().sum::<u32>();

        // 或者不使用"turbofish"写法，直接显式指定final_sum的类型
        let final_sum_2: u32 = thread_sums.iter().sum();
        assert_eq!(final_sum_1, final_sum_2);

        // 打印结果
        println!("Final sum result: {}", final_sum_1);
    }
}