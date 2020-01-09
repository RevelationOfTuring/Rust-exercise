/*
    trait object就是：指向trait的指针。
    假如Bird是一个trait的名称， 那么dyn Bird就是一个`DST`动态大小类型。
    那么：
        - &dyn Bird
        - &mut dyn Bird
        - Box<dyn Bird>
        - * const dyn Bird
        - *mut dyn Bird
        - Rc<dyn Bird> 等等
    都是trait object。

    dyn 是一个新引入的上下文关键字。
    impl Trait for Trait 这样的语法同样会被改为 impl Trait for dyn Trait。
    这样能更好地跟 impl Trait 语法对应起来。

    当指针指向trait的时候，这个指针就变成一个“胖指针”。
    按照上面的例子：
        Bird只是一个trait的名字，符合这个trait的具体类型可能有多种，
        这些类型并不具备同样的大小，因此使用`dyn Bird`来表示`满足Bird约束的DST类型`。
*/

// 指向DST的指针理所当然也应该是一个 “胖指针”。
// 它里面包含了两个成员 都是指向单元类型的`裸指针`。

trait Bird {
    fn fly(&self);
}

struct Duck;

impl Bird for Duck {
    fn fly(&self) {
        println!("Duck flies!")
    }
}

struct Swan;

impl Bird for Swan {
    fn fly(&self) {
        println!("Swan flies!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 参数是 trait object 类型 ，p是一个胖指针
    fn print_trait_object(p: &dyn Bird) {
        // 使用transmute执行强制类型转换，把变量p的内部数据取出来
        let (data, pointer): (usize, *const usize) = unsafe { std::mem::transmute(p) };
        println!("TraitObject [data:{}, pointer:{:p})", data, pointer);
        unsafe {
            // 打印出指针 pointer 指向的内存区间的值
            println!("data in pointer [{}, {}, {}, {}]",
                     *pointer,
                     *pointer.offset(1),
                     *pointer.offset(2),
                     *pointer.offset(3),
            )
        }
    }

    #[test]
    fn test_trait_object() {
        use std::mem;
        // 尝试一下使用unsafe代码，如果把它里面的数值当成整数拿出来会是什么结果:
        let duck = Duck {};
        let p_duck = &duck;
        // 转成trait object
        let p_bird = p_duck as &dyn Bird;
        // 看一下指针长度
        println!("{} {}",
                 mem::size_of_val(&p_duck), // 8
                 mem::size_of_val(&p_bird), // 16
        );
        // 可见trait object确实是一个胖指针

        // 看一下Duck和Swan的fly方法的入口地址(需要显示类型转换成usize)
        let duck_fly_func = Duck::fly as usize;
        let swan_fly_func = Swan::fly as usize;
        println!("{} {}", duck_fly_func, swan_fly_func);
        // 4303742864 4303742928

        // 最后看一下trait object 胖指针中到底是啥？
        print_trait_object(p_bird);
        //  TraitObject [data:123145443656664, pointer:0x1008fd2f8)
        //  data in pointer [4303762128, 0, 1, 4303742864]
        //  (pointer中最后的打印内容4303742864，恰好就是Duck的fly函数的入口地址)
        let swan = Swan {};
        print_trait_object(&swan as &dyn Bird);
        // TraitObject [data:123145443656952, pointer:0x1008fd348)
        // data in pointer [4303761600, 0, 1, 4303742928]
        // (pointer中最后的打印内容4303742928，恰好就是Swan的fly函数的入口地址)

        /*
            总结：
                如果指向trait的指针只包含了对象的地址，那么它就没办法实现针对不同的具体类型调用不同的函数了。
                所以，它不仅要包含一个指向真实对象的指针，还要有一个指向所谓的“虚函数表”的指针。
                我们把虚函数表里面的内容打印出来可以看到，里面有我们需要被调用的具体函数的地址。

            所以在Rust里面， 对象本身不包含指向虚函数表的指针，这个指针是存在于trait object指针里面。
            如果一个类型实现了多个trait，那么不同的trait object指向的虚函数表也不一样 。
        */
    }
}