/*
    static

    'static 生命周期是可能的生命周期中`最长`的，它会在整个程序运行的时期中存在。
    'static 生命周期也可被强制转换成一个`更短`的生命周期。

    有两种方式使变量拥有 'static 生命周期，它们都把数据保存在可执行文件的`只读内存区`：
        - 使用 static 声明来产生常量（constant）
        - 产生一个拥有 &'static str 类型的 string 字面量
*/
// 产生一个拥有 `'static` 生命周期的常量
static NUM: i32 = 1024;

#[cfg(test)]
mod tests {
    use super::NUM;

    // 返回一个指向 `NUM` 的引用，该引用不取 `NUM` 的 `'static` 生命周期，
    // 而是被强制转换成和输入参数的一样。
    fn coerce<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }
    /*
        等同于：
        fn coerce(_: &i32) -> &i32 {
            &NUM
        }
    */

    #[test]
    fn test_static() {
        {   // 作用域1
            // 产生一个 `string` 字面量并打印它
            let static_str = "michael.w";   // 该字面量拥有'static的生命周期
            println!("{}", static_str);

            // 当 `static_str` 离开作用域时，该引用不能再使用。
            // 不过数据仍然存在于二进制文件里面
        }


        {   // 作用域2
            // 产生一个整型给 `coerce_static` 使用：
            let num = 1024;

            // 将对 `NUM` 的引用强制转换成 num 的生命周期：
            let coerced_static = coerce(&num);
            println!("coerced_static: {}", coerced_static);
        }

        // 作用域3
        println!("NUM: {} stays accessible!", NUM);
    }
}