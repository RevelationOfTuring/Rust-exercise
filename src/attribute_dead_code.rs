/*
       编译器提供了 dead_code（死代码，无效代码）lint，这会对未使用的函数产生warning。
       可以用一个属性来禁用这个 lint。
       简直是强迫症爱好者的福音呀！
*/



#[cfg(test)]
mod tests {
    fn used_function() {
        println!("called `used_function`")
    }

    // 该函数未被调用，编译会报waring
    fn unused_function() {
        println!("called `unused_function`")
    }

    // 加上#[allow(dead_code)]属性，未调用的函数不会报warning
    #[allow(dead_code)]
    fn unused_function_2() {
        println!("called `unused_function_2`")
    }

    // 注意在实际程序中，需要将死代码清除掉。
    // 在开发过程中有时需要临时调试函数，所以可以允许一些死代码的出现。

    #[test]
    fn test_attribute_dead_code() {
        used_function()
    }
}