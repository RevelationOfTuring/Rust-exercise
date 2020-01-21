/*
    DSL(领域专用语言)

    DSL是Rust的宏中集成的微型 “语言”。
    这种语言是完全合法的，因为宏系统会把它转换成普通的Rust语法树，它只不过看起来像是另一种语言而已。

    这就允许你为一些特定功能创造一套简洁直观的语法（当然是有限制的）。
*/

// 比如:我想要定义一套小的计算器API，可以传给它表达式，它会把结果打印到控制台上。
macro_rules! calculate {
    (michael $e:expr) => {
        {
            {
                // 强制类型转换为整型
                let val:usize = $e;
                println!("{} = {}",stringify!{$e}, val);
            }
        }
    };
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_domain_specific_languages() {
        // 宏后面可以跟{}和()
        // ()后面必须跟分号;
        // {}后面可以不写分号;
        calculate! {
            michael (1024+1)*(8/2)
        }
        // michael并不是Rust的关键字，只是我们自己设定的一个flag
        calculate!(michael 1+2);
    }
}

/*
    这个例子非常简单，但是已经有很多利用宏开发的复杂接口了，比如 lazy_static 和 clap
*/