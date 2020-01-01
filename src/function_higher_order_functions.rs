// Rust提供了高阶函数（Higher Order Function, HOF。
// 指那些输入一个或多个函数，并且/或者产生一个更有用的函数的函数。
// HOF和惰性迭代器（lazy iterator）给Rust带来了函数式（functional）编程的风格。




#[cfg(test)]
mod tests {
    fn is_odd(number: u32) -> bool {
        number % 2 == 1
    }

    // Find the sum of all the squared odd numbers under 1000
    #[test]
    fn test_function_higher_order_functions() {
        let upper = 1000;

        // 命令式（imperative）的写法
        // 声明累加器变量
        let mut acc = 0;
        // 迭代：0，1, 2, ... 到无穷大
        for n in 0.. {
            let n_squared = n * n;
            if n_squared > upper {
                break;
            } else if is_odd(n_squared) {
                acc += n_squared;
            }
        }

        println!("{}", acc); //5456

        // 函数式的写法
        let sum_of_squared_odd_numbers =
            (0..).map(|n| n * n)                        // 所有自然数取平方
                .take_while(|&n| n < upper)       // 一直计算到取到小于上限为止
                .filter(|&n| is_odd(n))     // 取奇数
                .fold(0, |sum, i| sum + i);       // 依次累加起来

        println!("{}", sum_of_squared_odd_numbers); // 5456


        // 注：Option和迭代器都实现了不少高阶函数。
    }
}