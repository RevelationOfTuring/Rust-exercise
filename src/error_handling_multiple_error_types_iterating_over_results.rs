/*
    遍历Result
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_error_handling_multiple_error_types_iterating_over_results_iter_map() {
        // Iter::map 操作可能失败，比如:
        let strings = vec!["michael.w", "1024", "2048"];
        let possible_numbers: Vec<_> = strings
            // 生成迭代器
            .into_iter()
            // 以此解析strings中的元素
            .map(|str| str.parse::<i32>())
            // 将每个元素的parse结果放到一个新的vec中——possible_numbers
            .collect();
        println!("{:?}", possible_numbers);
        // 输出：[Err(ParseIntError { kind: InvalidDigit }), Ok(1024), Ok(2048)]
        // 注意：第一个元素是Err
    }

    #[test]
    fn test_error_handling_multiple_error_types_iterating_over_results_filter_map() {
        /*  使用filter_map()忽略失败的项
            filter_map 会调用一个函数，过滤掉为 None 的所有结果
        */
        let strings = vec!["michael.w", "1024", "2048"];
        let numbers: Vec<_> = strings
            .into_iter()
            .map(|str| str.parse::<i32>())
            // Result::ok是Result的一个内联方法，将Result转为Option：
            //      Err->None
            //      Ok(t)->Some(t)
            .filter_map(Result::ok)
            .collect();
        println!("{:?}", numbers);
    }

    #[test]
    fn test_error_handling_multiple_error_types_iterating_over_results_collect() {
        /*
            使用collect()使整个操作失败：
            Result 实现了 `FromIter`，因此结果的向量（Vec<Result<T, E>>）可以被转换成结果包裹着向量（Result<Vec<T>, E>）。
            一旦找到一个 Result::Err ，遍历就被终止。
        */

        let strings = vec!["1024", "michael.w", "2048"];
        let numbers: Result<Vec<_>, _> = strings
            .into_iter()
            .map(|str| str.parse::<i32>())
            .collect();
        println!("{:?}", numbers);
        // 输出： Err(ParseIntError { kind: InvalidDigit })
    }

    #[test]
    fn test_error_handling_multiple_error_types_iterating_over_results_partition_1() {
        /*
            使用Partition()收集所有合法的值与错误
        */

        let strings = vec!["1024", "michael.w", "2048"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|str| str.parse::<i32>())
            // Result::is_ok是Result的内联方法
            .partition(Result::is_ok);

        println!("{:?}\n{:?}", numbers, errors);
        // 输出：
        //    [Ok(1024), Ok(2048)]
        //    [Err(ParseIntError { kind: InvalidDigit })]
    }

    // 当看着这些结果时，会发现所有东西还在 Result 中保存着。
    // 要取出它们，需要一些模板化的代码
    #[test]
    fn test_error_handling_multiple_error_types_iterating_over_results_partition_2() {
        let strings = vec!["1024", "michael.w", "2048"];
        let (numbers, errors): (Vec<_>, Vec<_>) = strings
            .into_iter()
            .map(|str| str.parse::<i32>())
            .partition(Result::is_ok);
        // 从Result找中将元素取出来
        // 即利用迭代器依次做unwrap操作
        let numbers: Vec<i32> = numbers.into_iter().map(Result::unwrap).collect();
        // 对元素做取err操作
        let errors:Vec<_>=errors.into_iter().map(Result::unwrap_err).collect();
        println!("{:?}\n{:?}", numbers, errors);
        // 输出：
        //          [1024, 2048]
        //          [ParseIntError { kind: InvalidDigit }]
    }
}