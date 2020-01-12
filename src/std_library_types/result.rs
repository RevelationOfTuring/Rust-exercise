/*
    Result

    Option 枚举类型可以用作可能失败的函数的返回值，其中返回 None 可以表明失败。
    但是有时要强调为什么一个操作会失败，需要携带一些错误信息。
    为做到这点，使用 Result 枚举类型。

    Result<T, E> 类型拥有两个取值：
        - Ok(value) 表示操作成功，并包装操作返回的 value（value 的类型为T）。
        - Err(why)，表示操作失败，并包装 why，它（但愿）能够解释失败的原因（why 的类型为E）。
*/

// 定义一个mod，用来进行数学运算
mod checked {
    // 自定义一种数学错误
    // 由于要让本mod外的代码调用，所以可见性设为pub
    #[derive(Debug)]        // 用于错误原因打印
    pub enum MichaelMathError {
        // 自定义三种数学错误类型：
        // 除数为0
        MichaelDivisionByZero,
        // 对负数开平方根
        MichaelNegativeSquareRoot,
        // 对数为负数
        MichaelNegativeLogarithm,
    }

    // 出于便捷性，定义Result类型别名
    pub type MichaelResult = Result<f64, MichaelMathError>;

    // 定义运算逻辑
    // 返回值为自定义Result类型
    pub fn divide(divident: f64, divisor: f64) -> MichaelResult {
        if divisor == 0.0 {
            // 如果除数为0
            Err(MichaelMathError::MichaelDivisionByZero)
        } else {
            // 除法结果包在Ok中
            Ok(divident / divisor)
        }
    }

    // 定义开平方运算
    // 返回值为自定义Result类型
    pub fn sqrt(num: f64) -> MichaelResult {
        if num < 0.0 {
            // 如果被开方数为负数
            Err(MichaelMathError::MichaelNegativeSquareRoot)
        } else {
            Ok(num.sqrt())
        }
    }

    // 定义自然对数运算(log e (N))
    // 返回值为自定义Result类型
    pub fn ln(num: f64) -> MichaelResult {
        if num < 0.0 {
            // 如果被开方数为负数
            Err(MichaelMathError::MichaelNegativeLogarithm)
        } else {
            Ok(num.ln())
        }
    }
}

#[cfg(test)]
mod tests {
    // 定义一个用来处理checked mod 中运算的函数（主要是解析结果）
    // 运算逻辑为：sqrt(ln(x / y))
    // 输入和输出都是f64，直接给使用者调用
    fn op(num1: f64, num2: f64) -> f64 {
        // 三层的match嵌套！！
        // 1.除法
        match super::checked::divide(num1, num2) {
            Err(e) => {
                println!("divide Error: {:?}", e);
                0.0
            }
            Ok(result_divide) => {
                // 2.取自然对数
                match super::checked::ln(result_divide) {
                    Err(e) => {
                        println!("ln Error: {:?}", e);
                        0.0
                    }
                    Ok(result_ln) => {
                        match super::checked::sqrt(result_ln) {
                            Err(e) => {
                                println!("sqrt Error: {:?}", e);
                                0.0
                            }
                            Ok(result_sqrt) => result_sqrt
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_result() {
        // 执行运算 sqrt(ln(1 / 10))
        println!("The result of sqrt(ln(1 / 10) is {}", op(1.0, 10.0));
        // 输出：sqrt Error: MichaelNegativeSquareRoot
        //      The result of sqrt(ln(1 / 10) is 0

        // 因为ln(0.1)为负数，会引发自定义数学错误：
        // MichaelMathError::MichaelNegativeSquareRoot
        // 但是我自定义出错后的逻辑，不会panic。返回值为0。


        // 执行运算 sqrt(ln(10 / 0))
        println!("The result of sqrt(ln(10 / 0) is {}", op(10.0, 0.0));
        // 输出：divide Error: MichaelDivisionByZero
        //      The result of sqrt(ln(10 / 0) is 0

        // 执行运算 sqrt(ln(10 / -1))
        println!("The result of sqrt(ln(10 / -1) is {}", op(10.0, -1.0));
        // 输出：ln Error: MichaelNegativeLogarithm
        //      The result of sqrt(ln(10 / -1) is 0

        // 以上自定义的三种数学错误都被触发
        // 正常执行一次有效的计算
        println!("The result of sqrt(ln(10 / 0.5) is {}", op(10.0, 0.5));
        // 输出：The result of sqrt(ln(10 / 0.5) is 1.7308183826022854
    }
}