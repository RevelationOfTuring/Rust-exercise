mod mutability;
/*
    所有权和移动
    因为变量要负责释放它们拥有的资源，所以资源只能拥有`一个`所有者。
    这也防止了资源的`重复释放`。注意并非所有变量都拥有资源（例如引用）。

    在进行`赋值`（let x = y）或通过`值`来传递`函数参数`（foo(x)）的时候，
    资源的所有权（ownership）会发生转移。
    按照 Rust 的说法，这被称为资源的移动（move）。

    在移动资源之后，原来的所有者不能再被使用，这可避免悬挂指针（dangling pointer）的产生。
*/

#[cfg(test)]
mod tests {
    // 此函数取得堆分配的内存的所有权
    fn destroy_box(b: Box<i32>) {
        println!("Destroying a box that contains {}", b);
        // `b` 被销毁且内存得到释放
    }

    #[test]
    fn test_ownership_and_moves() {
        // 栈分配的整型
        let x = 1024;

        // 复制语义
        let y = x;

        // x和y都可以使用
        println!("x is {}, and y is {}", x, y);

        // `b` 是一个指向堆分配的整数的指针
        let b = Box::new(2048);
        println!("b contains: {}", b);

        // 把 `b` 的指针地址（而非数据）复制到 `b1`。
        // 现在两者都指向同一个堆分配的数据，但是现在是 `b1` 拥有它。
        let b1 = b;     // Box只有移动语义
        println!("{}", b1);

        // 编译报错：因为b已经不再拥有那部分堆上的内存了。
//        println!("{}", b);

        // 此函数从 `b1` 中取得堆分配的内存的所有权
        destroy_box(b1);

        // 此时b1指向的堆内存已经被释放。
        // 下面操作会导致解引用已释放的内存，而这是编译器禁止的。
        // 编译报错！和前面出错的原因一样。
//        println!("{}", b1);
    }
}