/*
    Rc指针

    Box包装的指针，所有权只能有一个。给了这个struct，那么之前的就被move了。
    当需要`多份`所有权的时候，就可以使用Rc（Reference Counting）指针。

    Rc会对其内部包含的value的所有权进行追踪，记录着对该value的引用总数：

    当Rc被clone时，其Reference Counting就会加一；当被clone的对象析构时，
    Reference Counting便会减一。
    当Rc的引用计数变成0的时候，意味着已经没有对其内部value保持所有权的owner了，
    Rc和其内部的value便将同时析构。

    Clone一个Rc绝对不是一个deep copy。Clone的过程仅仅是产生了另一个新的指向
    其内部value的指针，并且增加了Rc内部的reference计数。

    注：Rc指针所在mod: std::rc::Rc;
*/
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    #[test]
    fn test_rc() {
        // 创建一个String作为Rc的内部value
        let value_string = "Reference Counting".to_string();

        {   // 作用域2
            println!("************* rc_1 is creating *************");
            // 对value_string创建Rc指针1，创建方式跟Box指针一样
            let rc_1 = Rc::new(value_string);

            // 查看Rc的计数器
            // 利用Rc struct的静态方法
            println!("Reference Count of rc_1: {}", Rc::strong_count(&rc_1));

            {   // 作用域3
                println!("************* rc_1 is cloned to rc_2 *************");
                // clone一份Rc指针，相当于又创建了一个所有权
                let rc_2 = Rc::clone(&rc_1);

                // 分别查看两个Rc指针的引用计数器
                println!("Reference Count of rc_2: {}", Rc::strong_count(&rc_2));
                assert_eq!(Rc::strong_count(&rc_2), 2);
                println!("Reference Count of rc_1: {}", Rc::strong_count(&rc_1));
                assert_eq!(Rc::strong_count(&rc_2), 2);

                // 如果两个Rc底层指向的value相等，那么eq()方法的结果为true
                assert!(rc_1.eq(&rc_2));
                // 再创建一个新的rc_3
                let rc_3 = Rc::new(String::from("Reference Counting"));
                assert!(rc_1.eq(&rc_3));    // true

                // 通过rc指针，可以直接使用其内部value的内联方法
                println!("Length of the value inside rc_1: {}", rc_1.len());
                println!("Value of rc_2: {}", rc_2);

                println!("************* rc_2 is dropped out of scope *************")
            }

            //  rc_2被析构后，再来查看rc_1的引用计数
            println!("Reference Count of rc_1: {}", Rc::strong_count(&rc_1));
            assert_eq!(Rc::strong_count(&rc_1), 1);

            println!("************* rc_1 is dropped out of scope *************")
        }

        // 由于rc_1被析构，引用数为0，Rc内的value会被同时析构掉
        // 所以下面编译报错，因为value_string已经被连带着析构了。
//        println!("value_string: {}", value_string);
    }
}