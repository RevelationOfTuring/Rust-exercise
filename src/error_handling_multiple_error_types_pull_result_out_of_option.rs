/*
    处理混合错误类型的最基本的手段就是让它们互相包含。
*/
#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        // vec.first()返回值是Option，map只是在该Option里面更新值
        vec.first().map(|num_str| {
            // Option中将&str换成: num_str.parse::<i32>().map(|num| num * 2)
            num_str.parse::<i32>().map(|num| num * 2)
            // num_str.parse::<i32>()的返回值是Result<i32,Err>
            // map只是在该Result中将原来的i32值扩大二倍
        })
    }

    #[test]
    fn test_error_handling_multiple_error_types_pull_result_out_of_option() {
        let numbers = vec!["10", "20", "30"];
        println!("{:?}", double_first(numbers));
        // 输出:Some(Ok(20))

        let strings = vec!["michael.w", "20", "30"];
        println!("{:?}", double_first(strings));
        // 输出:Some(Err(ParseIntError { kind: InvalidDigit }))

        let empty = Vec::new();
        println!("{:?}", double_first(empty));
        // 输出:None
    }

    /*
        有时候我们不想再处理错误（比如使用 ? 的时候），但如果 Option 是 None 则继续处理错误。
        一些组合算子可以轻松地交换 Result 和 Option。
    */

    fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        // 先计算并形成类型Option<Result<...>>
        let option = vec.first().map(|num_str| {
            num_str.parse::<i32>().map(|num| num * 2)
        });

        // 由于返回值类型为Result<Option<...>>，需要Option和Result交互一下
        // 重绑定
        let option = option.map_or(Ok(None), |r| r.map(Some))?;
        /*
        option的map_or方法源码：
        pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
            match self {
                Some(t) => f(t),
                None => default,
            }
        }

        其功能：
            如果该option是None，则返回参数default，即Ok(None)
            如果该option是Some，则将其中包含的r，利用闭包f进行计算并返回，即返回r.map(Some)
            其中r为Result<i32,ParseIntError>类型，其map方法是用op（即Some）包裹其中i32（如果Result是Ok的话），否则返回Err
        最后的?是为整个Result拆箱
        重绑定的option有三种情况：None、Some(i32)和解析报错e
        */

        Ok(option)
    }

    #[test]
    fn test_error_handling_multiple_error_types_pull_result_out_of_option_v2() {
        let numbers = vec!["10", "20", "30"];
        println!("{:?}", double_first_v2(numbers));
        // 输出:Ok(Some(20))

        let strings = vec!["michael.w", "20", "30"];
        println!("{:?}", double_first_v2(strings));
        // 输出:Err(ParseIntError { kind: InvalidDigit })

        let empty = Vec::new();
        println!("{:?}", double_first_v2(empty));
        // 输出:Ok(None)
    }
}