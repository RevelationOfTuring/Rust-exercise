/*
    结构体

    在结构体中标注生命周期也和函数的类似
*/

#[cfg(test)]
mod tests {
    // struct tuple
    // 含有一个指向 `i32` 类型的引用
    // 注：&i32的生命周期必须比Borrowed寿命更长
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // 和前面类似，这里的两个引用都必须比这个结构体长寿
    #[derive(Debug)]
    struct NameBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // 一个枚举类型，其取值不是 i32 类型,就是一个指向 &i32
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    #[test]
    fn test_structs() {
        let (x, y) = (1024, 2048);

        // x和y的引用都分别赋值到下面这些struct对象中
        let single = Borrowed(&x);
        let double = NameBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
    }
}