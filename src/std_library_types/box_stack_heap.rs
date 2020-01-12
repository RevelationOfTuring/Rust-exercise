/*
    在 Rust 中，所有值默认都是`栈分配`的。

    通过创建 Box<T>，可以把值装箱来使它在堆上分配。
    箱子（box，即 Box<T> 类型的实例）是一个`智能指针`，指向堆分配的 T 类型的值。

    当箱子离开`作用域`时，它的析构函数会被调用，内部的对象会被`销毁`，堆上分配的内存也会被释放。

    注：被装箱的值可以使用 * 运算符进行解引用；这会移除掉一层装箱。
*/

#[cfg(test)]
mod tests {
    struct Test1 {
        x: f32,
        y: f64,
    }

    // Test2 由 Test1组成
    struct Test2 {
        field1: Test1,
        field2: Test1,
    }

    impl Default for Test1 {
        fn default() -> Self {
            Test1 { x: 0.0, y: 0.0 }
        }
    }

    // 创建Box<Test1>的默认值
    fn boxed_default() -> Box<Test1> {
        // 在堆上分配这个Test1对象，并返回一个指向它的指针
        Box::new(Test1 { x: 0.0, y: 0.0 })
    }

    #[test]
    fn test_box_stack_heap() {
        // 栈分配的变量
        let test1 = Test1::default();
        let test2 = Test2 {
            field1: Test1::default(),
            field2: Test1 { x: 1024.0, y: 2048.0 },
        };

        // 堆分配变量
        let boxed_test1 = boxed_default();
        let boxed_test2 = Box::new(Test2 {
            field1: Test1::default(),
            field2: Test1::default(),
        });

        // 可以直接将函数的返回值装箱
        let boxed_test1_2 = Box::new(Test1::default());

        // 两层装箱(Box里面装Box)
        let box_double = Box::new(boxed_default());

        /* 看内存占用情况 */
        use std::mem;
        println!("[stack] Test1: {}", mem::size_of_val(&test1));
        assert_eq!(16, mem::size_of_val(&test1));

        println!("[stack] Test2: {}", mem::size_of_val(&test2));
        assert_eq!(32, mem::size_of_val(&test2));

        // Box宽度，即智能指针内存占用情况
        println!("[stack] Box<Test1>: {}", mem::size_of_val(&boxed_test1));
        assert_eq!(8, mem::size_of_val(&boxed_test1));

        println!("[stack] Box<Test2>: {}", mem::size_of_val(&boxed_test2));
        assert_eq!(8, mem::size_of_val(&boxed_test2));

        println!("[stack] Box<Box<Test1>>: {}", mem::size_of_val(&box_double));
        assert_eq!(8, mem::size_of_val(&box_double));


        // 将Box中的数据拆箱
        // 即，将boxed_test1_2中的内容复制到unboxed_test1的内存空间中
        let unboxed_test1 = *boxed_test1_2;
        println!("[stack] unboxed_test1: {}", mem::size_of_val(&unboxed_test1));
        assert_eq!(16, mem::size_of_val(&unboxed_test1));
        // 跟Test1对象的空间长度一致（就是一个Test1对象）
    }
}