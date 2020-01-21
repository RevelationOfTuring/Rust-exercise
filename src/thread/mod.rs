/*
    线程

    Rust 通过 spawn 函数提供了创建本地操作系统（native OS）线程的机制。
    该函数的参数是一个通过值捕获变量的闭包（moving closure）。
*/
#[cfg(test)]
mod tests {
    /*
        注：一下代码必须整合到main函数中才能看到打印信息
    */
    use std::thread;

    const THREAD_NUM: i32 = 10;

    #[test]
    fn test_thread() {
        // 提供一个 vector 来存放所创建的子线程（children）
        let mut thread_handlers = vec![];

        for i in 0..THREAD_NUM {
            // 启动线程
            // 并将线程的handler放进vector中
            thread_handlers.push(thread::spawn(move || {
                println!("THREAD {} is starting", i);
            }))
        }

        for handler in thread_handlers {
            // 等待线程结束。返回一个结果
            let _ = handler.join();
        }
    }

    /*
        这些线程由操作系统调度（schedule）
    */
}