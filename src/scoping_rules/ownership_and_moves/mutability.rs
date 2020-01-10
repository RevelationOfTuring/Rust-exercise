/*
    可变性
    当所有权转移时，数据的可变性可能发生改变。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_mutability() {
        // 只读的Box，不可修改
        let immutable_box = Box::new(1024);

        // 可变性错误
//        *immutable_box=2048;

        // *移动* box，改变所有权（和可变性)。可变性变为可变。
        let mut mutable_box = immutable_box;

        assert_eq!(1024, *mutable_box);

        // 修改box的内容
        *mutable_box *= 2;

        assert_eq!(2048, *mutable_box);
    }
}