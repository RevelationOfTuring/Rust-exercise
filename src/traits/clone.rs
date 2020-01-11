/*
    Clone

    当处理资源时，默认的行为是在赋值或函数调用的同时将它们转移。(移动语义)
    但是有时候也需要把资源复制一份。

    Clone trait 正好帮助我们完成这任务。
    通常，我们可以使用 Clone trait 中定义的clone()方法。
*/
#[cfg(test)]
mod tests {
    // 不含资源的结构体
    #[derive(Debug, Copy, Clone)]
    struct Nil;

    // 包含资源的结构体，并实现了Clone trait
    #[derive(Debug, Clone)]
    struct Pair<'a>(Box<f64>, &'a str);

    #[test]
    fn test_clone() {
        let nil = Nil;
        // 因为Nil实现了Copy trait，所以是复制语义
        let nil_copy = nil;
        // nil和nil_copy都可以使用
        println!("{:?} {:?}", nil, nil_copy);

        let pair = Pair(Box::new(1024f64), "michael.w");
        // 因为Pair没有实现Copy trait，所以是移动语义
        let pair_move = pair;
        println!("{:?}", pair_move);
        // 编译报错，pair已经没有所用权
//        println!("{:?}", pair);

        // 手动复制克隆一份一模一样的pair(调用clone方法)
        let pair_clone = pair_move.clone();
        // 排除共享引用的可能性，将原来的pair_move的内存释放掉
        std::mem::drop(pair_move);

        println!("{:?}", pair_clone);   // 编译通过，并打印内容
    }
}