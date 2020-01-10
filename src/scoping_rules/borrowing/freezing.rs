/*
    冻结

    当数据被`不可变地借用`时，它还会冻结（freeze）。
    已冻结的数据无法通过原始对象来修改，直到对这些数据的所有引用离开作用域为止。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_freezing() {
        let mut mutable_i32 = 1024;
        {
            // 不可变借用
            let ref_immutable = &mutable_i32;

            // 编译报错！`mutable_i32` 在本作用域被冻结
//            mutable_i32 = 2048;

            // 注：只有当不可变借用在后面被调用时，前面mutable_i32修改才会报错。
            println!("ref_immutable refers to {}", ref_immutable)
        }

        // mutable_i32在该作用域没有被冻结
        // 因为这里没有对mutable_i32的不可变借用
        mutable_i32 = 2048;
        println!("{}",mutable_i32)
    }
}