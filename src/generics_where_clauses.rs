/*
    约束也可以使用 where 分句来表达，它放在 { 的前面，而不需写在类型第一次出现之前。
    另外 where 从句可以用于`任意类型`的限定，而不局限于`类型参数`本身。
*/

/*
    where 在下面一些情况下有很用：
        1. 当分别指定泛型的类型和约束:
            impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
            // 使用`where`从句来表达约束关心
            impl <A,D> MyTrait<A,D> for YourType where
                A: TraitB + TraitC,
                D: TraitE + TraitF{}

        2. 当使用 where 从句比正常语法更有表现力时。
           下面例子中的 impl 如果不用 where 从句，就无法直接表达：
*/

#[cfg(test)]
mod tests {
    // 针对情况2的例子：

    use std::fmt::Debug;

    trait PrintInOption {
        fn print_in_option(self);
    }

    // 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
    // 或者改用另一种间接的方法。
    impl<T> PrintInOption for T where
        Option<T>: Debug {
        // 将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
        // 表明Option<T>要满足Debug约束，而不是单单的T满足Debug
        // 否则会给出错误的约束。
        fn print_in_option(self) {
            println!("{:?}", Some(self))
        }
    }
    // 其实上面的impl的意思是：每个类型T都能调用PrintInOption方法。
    // 但是这个类型T存在要求约束：Option<T>必须满足Debug的约束。


    #[test]
    fn test_generics_where_clauses() {
        let vec = vec![1, 2, 3, 4, 5];

        vec.print_in_option();
    }
}