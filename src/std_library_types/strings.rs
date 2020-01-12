/*
    字符串

    Rust中有两种字符串类型：String 和 &str 。

    String:
        String 被存储为由`字节`组成的 vector（Vec<u8>），但保证了它一定是一个`有效`的 UTF-8 序列。
        String 是堆分配的，可增长的，且不是零结尾的（null terminated）。
        （C++中的字符串以0表示结束，所以C++的字符串中不能包含0）

    &str:
        &str 是一个总是指向有效 UTF-8 序列的`切片`（&[u8]）
        可用来查看 String 的内容，就如同 &[T] 是 Vec<T> 的全部或部分引用一样。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_strings() {
        // 一个对只读内存中分配的字符串的引用：&str
        let test_str = "hello world awesome Rust!";

        // 逆序迭代单词，这里并没有分配新的字符串空间
        // split_whitespace()返回了一个迭代器（字符串被' '字符split）
        // rev()返回一个从右向左迭代的反向迭代器
        for word in test_str.split_whitespace().rev() {
            println!("{}", word);
        }
        // 输出：
        //      Rust!
        //      awesome
        //      world
        //      hello

        // 复制&str中的字符到vector中
        let test_str1 = "zxczxcvbbbbnm";
        // 需要手动表明Vec泛型类型
        let mut vec_chars: Vec<char> = test_str1.chars().collect();
        // 排序
        vec_chars.sort();
        println!("{:?}", vec_chars);
        // ['b', 'b', 'b', 'b', 'c', 'c', 'm', 'n', 'v', 'x', 'x', 'z', 'z']

        // 去重
        vec_chars.dedup();
        println!("{:?}", vec_chars);
        // ['b', 'c', 'm', 'n', 'v', 'x', 'z']

        // 试试汉字(因为Rust中的char类型占4个字节，突然想到是不是可以处理汉字)
        assert_eq!(4, std::mem::size_of::<char>());
        let mut vec_chars_1: Vec<char> = "好好学习，天天向上".chars().collect();

        // 排序
        vec_chars_1.sort();
        println!("{:?}", vec_chars_1);
        // ['上', '习', '向', '天', '天', '好', '好', '学', '，']

        // 去重
        vec_chars_1.dedup();
        println!("{:?}", vec_chars_1);
        // ['上', '习', '向', '天', '好', '学', '，']

        // 注：汉字的排序是按照unicode码大小升序排列

        // 创建一个空的且可增长的 `String`
        let mut test_string = String::new();
        // 向String尾部插入char
        for i in vec_chars {
            // 注：此时的vec_chars内容为：['b', 'c', 'm', 'n', 'v', 'x', 'z']

            // 插入char类型字符
            test_string.push(i);

            // 注：由于for ... in ... 是直接对vec_chars本体进行的
            // 所以 vec_chars 中的元素将被move到变量i中，vec_chars失去所有权
            // 如果在不想让vec_chars失去所有权，请使用 for i in &vec_chars {...}
        }

        println!("test_string: {}", test_string);
        assert_eq!(test_string.as_str(), "bcmnvxz");
        
        // 编译报错：vec_chars已失去所有权
//        println!("test_string: {:?}", vec_chars);

        // 这个缩短的字符串是原字符串的一个切片，所以没有执行新的分配操作
        // 强制类型转换：将&[char;7]转换为胖指针&[char]
        let chars_to_trim = &[' ', ',', '.'] as &[char];

        // 过滤字符串前后的特定字符
        let test_string1 = String::from(" ,. ab    c,d....,  ., ef,  ");
        // 将胖指针chars_to_trim中包含的字符都过滤掉(字符串前后部分区域是过滤执行区)
        let trimmed_str = test_string1.trim_matches(chars_to_trim);
        assert_eq!(trimmed_str, "ab    c,d....,  ., ef");

        // 直接在堆上分配一个字符串
        let test_string2 = String::from("I'm Satoshi");
        // 分配新内存test_string3并存储修改过的字符串
        // 字符串替换
        let test_string3 = test_string2.replace("Satoshi", "Michael.W");

        assert_eq!(test_string2.as_str(), "I'm Satoshi"); // 没变
        assert_eq!(test_string3.as_str(), "I'm Michael.W"); // 修改后的新字符串
    }
}