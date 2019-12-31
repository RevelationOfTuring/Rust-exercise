// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

#[cfg(test)]
mod tests {
//    use super::*;

    // 类型转换
    #[test]
    fn test_casting() {
        let decimal = 65.4321_f32;
        /*         错误！不提供隐式转换
                let integer: u8 = decimal;
        */
        let integer: u8 = decimal as u8;
        let character = integer as char;
        println!("Casting: {} -> {} -> {}", decimal, integer, character); // Casting: 65.4321 -> 65 -> A

        // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1) 直到值位于新类型 T 的范围内。
        // 1000 已经在 u16 的范围内
        println!("1000 as a u16 is: {}", 1000 as u16);

        // 1000 - 256 - 256 - 256 = 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
        // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
        // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
        // 最左边一位和最右边一位。

        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // 对正数，这就和取模一样。
        println!("1000 mod 256 is : {}", 1000 % 256);

        // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
        // 如果 MSB 是 1，则该值为负” 是一样的。

        // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
        println!(" 128 as a i16 is: {}", 128 as i16); // 128
        // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
        println!(" 128 as a i8 is : {}", 128 as i8); // -128

        // 重复之前的例子
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // 232 的二进制补码是 -24
        println!(" 232 as a i8 is : {}", 232 as i8);
    }

    // 字面量
    #[test]
    fn test_literals() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;
        let i = 1;
        let f = 1.0;

// std::mem::size_of_val 是一个函数，这里使用其完整路径（full path）调用。
// 代码可以分成一些叫做模块（module）的逻辑单元。
// 在本例中，size_of_val 函数是在 mem 模块中定义的，而 mem 模块又是在 std crate 中定义的。
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    // `NanoSecond` 是 `u64` 的新名字
    type NanoSecond = u64;
    // 由于变量名未使用驼峰定义，编译器会报错
    // 但是通过下面这个属性屏蔽警告。
    #[allow(non_camel_case_types)]
    type u64_t = u64;

    // 别名
    #[test]
    fn test_aliasing() {
// 可以用 type 语句给已有的类型取个新的名字。
// 类型的名字必须遵循驼峰命名法（像是 CamelCase 这样），否则编译器将给出错误。
// 原生类型是例外，比如： usize、f32，等等。

        let nanoseconds: NanoSecond = 5 as u64_t;

        // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
        println!("{}", nanoseconds);

        // 别名的主要用途是避免写出冗长的模板化代码（boilerplate code）。
        // 如 IoResult<T> 是 Result<T, IoError> 类型的别名。
    }
}