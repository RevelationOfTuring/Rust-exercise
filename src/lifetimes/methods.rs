/*
    方法

    方法的标注和函数类似

    方法一般是不需要标明生命周期的，因为 self 的生命周期会赋给所有的输出生命周期参数
*/
#[cfg(test)]
mod tests {
    struct S(i32);

    impl S {
        // 标注生命周期，就像独立的函数一样
        fn double<'a>(&'a mut self) {
            self.0 *= 2;
        }

        fn print<'a>(&'a self) {
            println!("print: {}", self.0);
        }
    }

    #[test]
    fn test_methods() {
        let mut s = S(1024);

        s.double();
        s.print();
        // print: 2048
    }
}