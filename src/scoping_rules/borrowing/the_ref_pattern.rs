/*
    ref 模式

    在通过 let 绑定来进行`模式匹配或解构`时，ref 关键字可用来创建结构体/元组的字段的引用。
*/

#[cfg(test)]
mod tests {
    #[derive(Copy, Clone)]
    struct Point(i32, i32);

    #[test]
    fn test_the_ref_pattern() {
        let c = 'A';

        // 赋值语句中左边的 `ref` 关键字等价于右边的 `&` 符号。
        let ref ref_c1 = c;
        let ref_c2 = &c;
        // ref_c1和ref_c2都是c的不可变引用
        println!("{} {} {}", ref_c1, ref_c2, *ref_c1 == *ref_c2);

        let point = Point(1, 2);

        // 在解构一个结构体时 `ref` 同样有效
        let copy_of_x = {
            // 模式匹配解构
            let Point(ref ref_to_x, _) = point;

            // 返回一个 `point` 的 `x` 字段的拷贝。
            *ref_to_x
        };

        assert_eq!(1, copy_of_x);

        // `point` 的可变拷贝
        // 需要Point实现Copy, Clone trait
        let mut mutable_point = point;

        {
            // `ref` 可以与 `mut` 结合以创建可变引用
            let Point(_, ref mut mut_ref_to_y) = mutable_point;

            // 修改
            *mut_ref_to_y = 1024;
        }

        // 修改验证
        assert_eq!(1024, mutable_point.1);

        // 包含一个指针的可变元组
        let mut mutable_tuple = (1024, Box::new(2048f64));
        {
            // 解构 `mutable_tuple` 来改变 `last` 的值(扩大二倍)
            // last故意取成Box的可变引用。为什么要引用？
            // 如果不用引用，那么Box会从mutable_tuple被move走，外面就无法使用*mutable_tuple.1了
            let (_, ref mut last) = mutable_tuple;
            **last *= 2f64;
        }

        // 验证修改
        assert_eq!(4096f64, *mutable_tuple.1)
    }
}