/*
    散列表HashMap

    vector 通过整型下标来存储值，而 HashMap（散列表）通过键（key）来存储值。

    HashMap 的`键`可以是布尔型、整型、字符串，或任意实现了 Eq 和 Hash trait 的其他类型。

    同 vector 类似，HashMap 也是`可增长`的，但 HashMap 在占据了多余空间时还可以缩小自己。
        - 使用 HashMap::with_capacity(unit) 创建具有一定初始容量的 HashMap；
        - 也可以使用 HashMap::new() 来获得一个带有默认初始容量的 HashMap（推荐这个方式）。

    注：Rust标准库中HashMap的位置：std::collections::HashMap

    想要了解更多关于散列（hash）与散列表（hash map）（有时也称作 hash table）的工作原理，可以查看:
    https://en.wikipedia.org/wiki/Hash_table
*/
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hashmap() {
        // 创建一个hashmap
        // 因为要添加KV对，所以要是mut
        let mut hm = HashMap::new();

        // 向其中加入KV对
        // 使用insert()方法
        hm.insert("btc", 100000);
        hm.insert("eth", 200000);
        hm.insert("atom", 300000);
        hm.insert("etc", 400000);

        // 如何查询？
        // 使用get()方法：接受参数是一个`引用`
        // 返回的是一个Option<&V>，Option中是V的共享引用，不是可修改引用，不是副本
        match hm.get(&"atom") {
            Some(amount) => println!("the amount of atom is {}", amount),
            None => println!("the amount of atom doesn't exist!"),
        }

        // 如果insert的值为新内容，那么 `HashMap::insert()` 返回None，
        // 否则返回之前的值Some(old_value)

        // 1.插入一个新key
        assert_eq!(hm.insert("eos", 999999), None);

        // 2.更新KV对
        assert_eq!(hm.insert("eos", 1000000), Some(999999));
        // 检查更新后的value
        assert_eq!(hm.get(&"eos"), Some(&1000000));

        // 删除KV对使用remove()方法：接受参数是一个`引用`
        // 注：如果删除成功，remove方法会返回被删除的Some(value)——Some里面不是引用，直接是value对象
        //    如果删除失败（根本没有对应key存在），remove方法返回None
        assert_eq!(hm.remove(&"eos"), Some(1000000));
        assert_eq!(hm.remove(&"eos"), None);

        // 如何迭代遍历hashmap?
        // `HashMap::iter()` 返回一个迭代器，该迭代器以`任意`顺序举出KV对(&key, &value)。都是引用。
        // 注：HashMap迭代内部元素是`无序`的。
        for (key, value) in hm.iter() {
            println!("key: {}, value: {}", key, value);
        }
    }
}