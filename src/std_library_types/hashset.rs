/*
    散列集HashSet

    请把 HashSet 当成这样一个 HashMap：只关心其中的键而非值。
    HashSet<T> 实际上只是对 HashMap<T, ()> 的封装。

    那这东西有什么意义呢？我完全可以将key都存到一个Vec中不就得了。

    HashSet的独特之处在于，它保证了不会出现`重复`的元素。
    这是任何 set 集合类型（set collection）遵循的规定。HashSet 只是其一个实现。

    ps.Rust中另一个set实现——BTreeSet
    传送门：https://doc.rust-lang.org/std/collections/struct.BTreeSet.html

    如果插入的值已经存在于 HashSet 中（即新值等于已存在的值，并且拥有相同的散列值），那么新值将会替换旧的值。

    如果不想要一样东西出现多于`一次`，或者要判断一样东西是不是已经存在，这种做法就很高效了。

    注：集合（set）可以做更多的事情：
        - union（并集）：获得两个集合中的所有元素（不含重复值）；
        - difference（差集）：获取属于第一个集合而不属于第二集合的所有元素；
        - intersection（交集）：获取同时属于两个集合的所有元素；
        - symmetric_difference（对称差）：获取所有只属于其中一个集合，而不同时属于两个集合的所有元素。

    HashSet中元素的存储顺序也是`无序`的。
*/

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn test_hashset() {
        // 创建两个不同的HashSet对象
        // 可以利用vec!宏来快速创建(HashSet类型必须显式标注出来)
        let mut set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
        let mut set2: HashSet<i32> = vec![3, 4, 5, 6, 7, 8].into_iter().collect();
        println!("{:?}", set1);

        // 向HashSet中增添元素，使用.insert()方法
        // 如果HashSet是第一次插入该值，返回true；如果不是，返回false
        assert!(set1.insert(99));   // 第一次插入返回true
        assert!(!set1.insert(99));  // 第二次插入返回false

        // set1: {1, 2, 3, 4, 5, 99}
        // set2: {3, 4, 5, 6, 7, 8}

        // 求并集
        println!("Union: {:?}", set1.union(&set2));
        // 打印：  Union: [99, 2, 5, 1, 4, 3, 8, 6, 7]

        // 求差集
        println!("Difference: {:?}", set1.difference(&set2));
        // 打印：  Difference: [99, 2, 1]

        // 求交集
        println!("Intersection: {:?}", set1.intersection(&set2));
        // 打印：  Intersection: [5, 4, 3]

        // 求对称差
        println!("Symmetric Difference: {:?}", set1.symmetric_difference(&set2));
        // 打印：  Symmetric Difference: [99, 2, 1, 8, 6, 7]

        // ps.上面打印出的都是乱序的
    }

    /****************************************************************************
      注： 如果一个集合(collection)的元素实现了Debug trait，那么该集合也就自然也实现了Debug
    *****************************************************************************/
    //  验证如下：

    //  因为要做HashSet的key
    #[derive(Debug, Eq, PartialEq, Hash)]
    struct Test<'a> {
        name: &'a str
    }

    #[test]
    fn test_hashset_verification() {
        // 创建HashSet对象
        let set: HashSet<Test> = vec![
            Test { name: "Michael.W" },
            Test { name: "Satoshi" },
            Test { name: "Vitalik" }
        ].into_iter().collect();

        // 由于元素类型Test实现了Debug，所以包含其的集合HashSet也自动实现了Debug trait
        println!("{:?}", set);
        // 打印：{Test { name: "Michael.W" }, Test { name: "Vitalik" }, Test { name: "Satoshi" }}
    }
}