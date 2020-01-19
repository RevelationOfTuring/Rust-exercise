/*
    重载

    宏可以重载，从而接受不同的参数组合。

    在这方面，macro_rules! 的作用类似于 匹配（match）代码块
*/
#[cfg(test)]
mod tests {
    // 根据调用的方式，自定义宏将以不同的方式来比较 `$left` 和 `$right`
    macro_rules! compare_michael {
        // 参数之间使用逗号和分号隔开，之间可以加自定义字符串进行分隔(下面的hello rust就是自定义字符)
        // 参数可以任意组合
        ($left:expr, hello rust $right:expr)=>(
            println!("{:?} and {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left && $right
            )
        );
      // ^ 每个分支都必须以分号结束。
      ($left:expr; or $right:expr)=>(
            println!("{:?} or {:?} is {:?}",
                    stringify!($left),
                    stringify!($right),
                    $left || $right
            )
      );
    }

    #[test]
    fn test_overload() {
        compare_michael!(1024u64+100>1, hello rust 1024u32-100!=1 );
        compare_michael!(true; or false);
    }
}