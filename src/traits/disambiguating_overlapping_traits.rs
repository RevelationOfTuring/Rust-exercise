/*
    trait的二义性消除

    一种struct可以实现许多个trait。但是如果两个不同的trait中定义了同名方法怎么办？
    由于每个trait的实现都有自己对应的 impl 代码块，
    程序员从代码上是很容易得知：不同trait的同名方法究竟是在做什么逻辑操作。

    但是如果调用时遇到同名方法该怎么办？
    可以使用完全限定语法（Fully Qualified Syntax）来区别这些同名方法
*/

#[cfg(test)]
mod tests {
    trait UserNameWidget {
        // 同名方法
        fn get(&self) -> String;
    }

    trait AgeWidget {
        // 同名方法
        fn get(&self) -> u32;
    }

    // 实现类
    struct Form<'a> {
        user_name: &'a str,
        age: u32,
    }

    // 分别实现了UserNameWidget和AgeWidget
    impl<'a> UserNameWidget for Form<'a> {
        fn get(&self) -> String {
            String::from(self.user_name)
        }
    }

    impl<'a> AgeWidget for Form<'a> {
        fn get(&self) -> u32 {
            self.age
        }
    }

    #[test]
    fn test_disambiguating_overlapping_traits() {
        let form = Form { user_name: "Michael.W", age: 18 };

        // 编译报错：error[E0034]: multiple applicable items in scope
//        println!("{}", form.get());

        // 此时只能使用完全限定语法，类似静态函数的调用方法
        let user_name = <Form as UserNameWidget>::get(&form);
        // 注：&str和String之间的转换：
        //  &str -> String : 使用"xxx".to_owned()
        //  String -> &str : 使用String.as_str()
        assert_eq!("Michael.W".to_owned(), user_name);
        assert_eq!(18, <Form as AgeWidget>::get(&form));
    }
}