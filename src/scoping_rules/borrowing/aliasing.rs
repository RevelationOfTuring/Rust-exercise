/*
    别名使用

    数据可以进行`多次`不可变借用，但是在不可变借用的期间，原始数据不可进行可变借用。（可变借用与不可变借用不能共存）

    另一方面，在同一时刻内只允许有`一个`可变借用。
    只有在可变引用离开作用域之后，原始数据才可再次被借用。
*/


#[cfg(test)]
mod tests {
    struct Point(i32, i32, i32);

    #[test]
    fn test_aliasing() {
        let mut point = Point(1, 2, 3);
        {
            // 不可变借用
            let borrowed_point_1 = &point;
            let borrowed_point_2 = &point;

            // 通过引用和原始所有者来访问数据
            // 注：在println!中想打印 "{" 和 "}" 时，请双写—— "{{" 和 "}}"
            println!("Point {{{}, {}, {}}}", point.0, borrowed_point_1.1, borrowed_point_2.2);


            // 可变借用point
            // 如果后面调用了其他的不可变借用，该处编译会报错
//            let mutable_borrow = &mut point;

            // 调用了point的不可变借用
            println!("Point {{{}, {}, {}}}", point.0, borrowed_point_1.1, borrowed_point_2.2);
        }

        {
            // 可变借用
            let mutable_borrow = &mut point;

            // 通过可变引用来改变数据
            mutable_borrow.0 = 1024;
            mutable_borrow.1 = 2048;
            mutable_borrow.2 = 4096;

            // 不可变地借用point
            // 如果后面调用了可变借用,，编译会在此报错！
            // 因为可变和不可变借用不能共存
//            let immutable_borrow = &point;
            // 调用了可变借用
            mutable_borrow.0 = 1025;

            // 打印报错：因为 `println!` 会创建一个不可变引用
            // 而`println!`后面又调用了可变引用mutable_borrow
//            println!("Point {{{}, {}, {}}}", point.0, point.1, point.2);

            mutable_borrow.0 = 1024;

            // 可变引用可以作为不可变引用的传给 `println!`
            println!("Point {{{}, {}, {}}}", mutable_borrow.0, mutable_borrow.1, mutable_borrow.2);
        }
        // 本作用域内没有可变引用
        let immutable_borrow = &point;
        println!("Point {{{}, {}, {}}}", immutable_borrow.0, immutable_borrow.1, immutable_borrow.2);
    }
}