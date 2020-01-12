/*
    vector是Rust中的动态数组。

    vector（动态数组）是大小可变的数组。
    和 slice（切片）类似，它们的大小在编译时是`未知`的，但它们可以随时扩大或缩小。

    一个 vector 使用 3 个词来表示：
        - 1.一个指向数据的指针
        - 1.vector 的长度
        - 3.还有它的容量（此容量指明了要为这个 vector 保留多少内存）

    vector 的长度只要小于该容量，就可以随意扩充元素；
    当需要超过这个阈值时，会自动给 vector 重新分配一段更大的内存容量。

    注：与Golang中的切片机制一模一样。

    重点提示：Rust中的vector，即Vec<T>只实现了Debug trait,并未实现Display。
    所以在打印vector的时候，要使用"{:?}"

    更多Vec的方法可以在 std::vec 模块中找到。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_vector() {

        // 可以将迭代器中的所有元素，收集到 vector 中
        // 注：需要手动标明Vec中的泛型类型，即迭代器中元素的类型。
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into: {:?}", collected_iterator);
        // 输出：Collected (0..10) into: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

        // vec! 宏可以方便快捷地初始化一个 vector(可直接将元素显式地写进去)
        let mut vec = vec![1024i64, 0, 1, 2, 3, 4];
        println!("Initial vector: {:?}", vec);

        // 尾部插入一个新的元素
        vec.push(2048);
        println!("Vector: {:?}", vec);

        // 编译报错，因为collected_iterator不是mut可变的
//        collected_iterator.push(1024);

        // 获取vector当前长度，即包含的元素个数
        println!("Vector size: {}", vec.len());

        // 可以使用下标索引从vector中取元素
        println!("First and second element: {} {}", vec[0], vec[1]);

        // 移除 vector 的最后一个元素(即最近push进来的元素)并将它返回
        println!("Pop last element: {:?}", vec.pop());
        println!("Vector: {:?}", vec);

        // 超出下标范围将抛出一个 panic：'index out of bounds: the len is 6 but the index is 100'
        // 注：Rust的索引越界是在编译期间无法检测出来的
//        println!("{}", vec[100]);

        // 遍历迭代一个vector
        // 使用 vec.iter()
        for i in vec.iter() {
            println!("{}", i);
        }

        // 索引与元素值一起迭代
        // 使用 vec.iter().enumerate()
        for (index, value) in vec.iter().enumerate() {
            println!("{}-{}", index, value);
        }

        // 如何在迭代的过程中修改vector中的元素值
        // 使用vec.iter_mut()
        for i in vec.iter_mut() {
            // i的类型是&mut i64
            *i *= 100;  // 元素都扩大100倍
        }

        println!("After modification: {:?}", vec);
    }
}