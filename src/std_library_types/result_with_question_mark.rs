/*
    ? 运算符

    result.rs文件中（op函数）的match嵌套显得很繁琐。
    通过使用?运算符可将这种逻辑变得简洁漂亮。

    ?运算符用只能用在返回值为Result或Option类型的函数中，并等表达这样一种逻辑：

    如果出现Err，那么立刻返回Err。如果所有分支都没有Err，那么就返回最后的子分支Ok(T)结果
*/

// 继续用result.rs文件中的例子做对比
mod checked {
    #[derive(Debug)]
    pub enum MichaelMathError {
        MichaelDivisionByZero,
        MichaelNegativeSquareRoot,
        MichaelNegativeLogarithm,
    }

    type MichaelResult = Result<f64, MichaelMathError>;

    pub fn div(divident: f64, divisor: f64) -> MichaelResult {
        if divisor == 0.0 {
            Err(MichaelMathError::MichaelDivisionByZero)
        } else {
            Ok(divident / divisor)
        }
    }

    pub fn ln(num: f64) -> MichaelResult {
        if num < 0.0 {
            Err(MichaelMathError::MichaelNegativeLogarithm)
        } else {
            Ok(num.ln())
        }
    }

    pub fn sqrt(num: f64) -> MichaelResult {
        if num < 0.0 {
            Err(MichaelMathError::MichaelNegativeSquareRoot)
        } else {
            Ok(num.sqrt())
        }
    }

    // 定义运算 sqrt(ln(x/y))
    // （用?运算法）
    pub fn op(num1: f64, num2: f64) {
        match run_op(num1, num2) {
            Err(e) => {
                // 逻辑和可读性立刻变得简单明了
                match e {
                    MichaelMathError::MichaelDivisionByZero => println!("Error: divisor is Zero: {:?}", e),
                    MichaelMathError::MichaelNegativeSquareRoot => println!("Error: radicand is negative: {:?}", e),
                    MichaelMathError::MichaelNegativeLogarithm => println!("Error: logarithm's number is negative: {:?}", e),
                }
            }
            Ok(result) => println!("Result is: {}", result)
        }
    }

    // 中间计算过程
    // 因为？运算符只能用在返回值为Result或Option的函数中
    // 所以要额外定义这样一个满足上述条件的中间函数
    fn run_op(num1: f64, num2: f64) -> MichaelResult {
        let result_div = div(num1, num2)?;
        let result_ln = ln(result_div)?;
        sqrt(result_ln)
    }
}

#[cfg(test)]
mod tests {
    use super::checked::op;

    #[test]
    fn test_result_with_question_mark() {
        /*      运算 sqrt(ln(x/y))        */
        //引发错误：MichaelMathError::MichaelDivisionByZero
        op(10.0, 0.0);
        // 打印：Error: divisor is Zero: MichaelDivisionByZero

        //引发错误：MichaelMathError::MichaelNegativeSquareRoot
        op(1.0, 10.0);
        // 打印：Error: radicand is negative: MichaelNegativeSquareRoot

        //引发错误：MichaelMathError::MichaelNegativeLogarithm
        op(1.0, -1.0);
        // 打印：Error: logarithm's number is negative: MichaelNegativeLogarithm

        // 无错误的运算
        op(-10.0, -0.1);
        // 打印： Result is: 2.145966026289347
    }
}