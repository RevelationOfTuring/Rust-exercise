/*
    条件编译可能通过两种不同的操作符实现：

        1. cfg 属性：在属性位置中使用 #[cfg(...)]
        2. cfg! 宏：在布尔表达式中使用 cfg!(...)

        两种形式使用的参数语法都相同。
*/

// 下面函数只有当目标系统是linux的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("I'm running linux!")
}

// 下面函数只有当目标系统不是linux的时候才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("I'm not running linux!")
}

#[cfg(test)]
mod tests {
    use crate::attribute_cfg::are_you_on_linux;

    #[test]
    fn test_attribute_cfg() {
        // 调用上面设定条件编译的函数
        are_you_on_linux();

        // 通过宏将条件编译逻辑加入代码
        if cfg!(target_os="linux") {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely not linux!");
        }

        // 运行结果：
        // I'm not running linux!
        // Yes. It's definitely not linux!
    }
}