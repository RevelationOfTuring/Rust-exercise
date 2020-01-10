/*
    Rust的变量不只是在栈中保存数据,比如Box<T>占有堆（heap）中的内存。

    Rust 强制实行RAII（Resource Acquisition Is Initiallization，资源获取即初始化）。
    所以任何对象在`离开`作用域时，它的`析构函数`（destructor）就被调用，然后它占有的资源就被释放。

    这种行为避免了资源泄漏（resource leak）,
    程序员不用手动释放内存或者担心内存泄漏（memory leak）了。
*/
mod destructor;

mod tests {
    // 在堆上分配一个整型数据
    fn create_box() {
        let _box1 = Box::new(1024);
        // `_box1` 在这里(离开create_box函数的作用域时)被销毁，内存得到释放
    }

    #[test]
    fn test_raii() {
        // 在堆上分配一个整型数据
        let _box2 = Box::new(2048);

        // 嵌套作用域：
        {
            let _box3 = Box::new(4096);
            // `_box3` 在这里被销毁，内存得到释放
        }

        // 创建许许多多的box。
        // 完全不需要手动释放内存！
        for _ in 0..1024 {
            create_box()
        }

        // `_box2` 在这里被销毁，内存得到释放
    }
}