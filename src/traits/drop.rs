/*
    Drop

    Drop trait只有一个方法：drop，当对象离开作用域时会自动调用该方法。
    Drop trait 的主要作用是释放实现者的实例拥有的资源。

    Box，Vec，String，File，以及 Process 是一些实现了 Drop trait 来释放资源的类型。
    同时，Drop trait 也可以为任何自定义数据类型手动实现。
*/
#[cfg(test)]
mod tests {
    struct Test {
        name: &'static str,
    }

    // 在释放资源时，添加打印功能
    impl Drop for Test {
        fn drop(&mut self) {
            println!("Test {} is dropping!", self.name)
        }
    }

    #[test]
    fn test_drop() {
        // 作用域1
        let test1 = Test { name: "test1" };

        {   // 作用域2
            let test2 = Test { name: "test2" };

            {   // 作用域3
                let test3 = Test { name: "test3" };
                let test4 = Test { name: "test4" };
                println!("Exiting Scope 3")
            }
            println!("Exited Scope 3");
            println!("Exiting Scope 2");
        }
        println!("Exited Scope 2");

        // std::mem::drop静态方法，可以主动释放对象
        std::mem::drop(test1);
        println!("Exiting Scope 1");

        // 注：test1不会在离开作用域1时再次销毁
        // 因为它已经被（手动）销毁——std::mem::drop()
    }
}