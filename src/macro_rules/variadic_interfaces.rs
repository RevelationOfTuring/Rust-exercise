/*
    可变参数接口

    可变参数接口可以接受任意数目的参数。
    比如说 println 就可以，其参数的数目是由`格式化`字符串指定的。
*/

//  把 domain_specific_languages.rs 中的 calculater! 宏改写成可变参数接口：
macro_rules! calculate {
    // 如果是单个参数
    (michael $e:expr) => {
        {
            let val:usize = $e;
            println!("{} = {}",stringify!{$e},val);

        }
    };

    // 如果是多个参数
    // 利用递归来拆解多个的参数
    (michael $e:expr,$(michael $e_next:expr),+) => {
        calculate!{michael $e}
        calculate!($(michael $e_next),+);

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_variadic_interfaces() {
        calculate! {michael (1024+1)/(4+1)}

        // 如果调用宏后面使用括号()，务必要在后面加分号
        calculate!(
        michael (1024+1)/(4+1),
        michael (1024+1)/(9+1),
        michael (1024+1)/(24+1)
        );
    }
}