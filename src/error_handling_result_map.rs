/*
    对于Result进行unwrap()时候报出panic的设计不是健壮的(robust)。
    一般地，我们希望把错误返回给`调用者`，这样它可以决定回应错误的正确方式。

    首先，需要了解需要处理的错误类型是什么。
    为了确定 Err 的类型，可以用 parse() 来试验。

    Rust 已经为 i32 类型使用 FromStr trait 实现了 parse()。
    结果表明，这里的 Err 类型被指定为 `ParseIntError`。
*/

/*
    注：由于目前用于`获取类型`的函数仍然是不稳定的，我们可以用间接的方法。使用下面的代码：
    fn main(){
        let i:()="michael.w".parse::<i32>();
    }

    由于Result类型和i的类型(unit)不一样，所以编译器一定或报错，
    并显示报出错误类型：
    note: expected type `()`
              found type `std::result::Result<i32, std::num::ParseIntError>`

*/


mod tests {
    use std::num::ParseIntError;

    // 直接返回Result类型，而不是返回i32
    fn multiply_v1(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
        match first_num_str.parse::<i32>() {
            Ok(n1) => {
                match second_num_str.parse::<i32>() {
                    Ok(n2) => {
                        // 经过重重匹配，返回了正确的结果
                        Ok(n1 * n2)
                    }
                    Err(e) => Err(e)
                }
            }
            Err(e) => Err(e)
        }
    }

    // 打印Result
    fn print_result(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("Right: {}", n),
            Err(e) => println!("Error: {}", e)
        }
    }

    #[test]
    fn test_error_handling_result_map_v1() {
        // 仍然会给出正确的答案
        let result1 = multiply_v1("10", "11");
        print_result(result1);

        // 这种情况下就会提供一条更有用的错误信息。
        let result2 = multiply_v1("michael.w", "11");
        print_result(result2);
        // Error: invalid digit found in string
    }

//    幸运的是，Option 的 map、and_then、以及很多其他组合算子也为 Result 实现了。
//    官方文档的 Result 一节包含完整的方法列表。

    // 就像 `Option` 那样，我们可以使用 `map()` 之类的组合算子。
    // 除去写法外，这个函数与上面那个完全一致，它的作用是：
    // 如果值是合法的，计算其乘积，否则返回错误
    fn multiply_v2(first_num_str: &str, second_num_str: &str) -> Result<i32, ParseIntError> {
        first_num_str.parse::<i32>().and_then(|n1| {
            second_num_str.parse::<i32>().map(|n2| n1 * n2)
        })
    }

    #[test]
    fn test_error_handling_result_map_v2() {
        // 仍然会给出正确的答案
        let result1 = multiply_v2("10", "11");
        print_result(result1);

        // 这种情况下就会提供一条更有用的错误信息。
        let result2 = multiply_v2("michael.w", "11");
        print_result(result2);
    }
}