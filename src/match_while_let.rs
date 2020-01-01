// 和 if let 类似，while let 也可以把别扭的 match 改写得好看一些。


#[cfg(test)]
mod tests {
    #[test]
    // 考虑下面这 段使 i 不断增加的代码：
    fn test_while_let_1() {
        // 将 `optional` 设为 `Option<i32>` 类型
        let mut optional = Some(0);
        // 重复运行这个测试。
        loop {
            match optional {
                // 如果 `optional` 解构成功，就执行下面语句块。
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        // 修改optional的值
                        optional = Some(i + 1);
                    }
                    // ^ 需要三层缩进！
                }
                // 当解构失败时退出循环：
                _ => { break; }
                // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
            }
        }
    }


    #[test]
//    使用 while let 可以使这段代码变得更加优雅：
    fn test_while_let_2() {
// 将 `optional` 设为 `Option<i32>` 类型
        let mut optional = Some(0);
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
            // ^ 使用的缩进更少，并且不用显式地处理失败情况。
        }
        // ^ `if let` 有可选的 `else`/`else if` 分句，
        // 而 `while let` 没有
    }
}