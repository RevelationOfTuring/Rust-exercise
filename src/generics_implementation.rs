/*
    和函数类似，impl块要想实现泛型，也需要很仔细。
*/

/*
    // 具体类型 `S`
    struct S;
    // 泛型类型 `GenericVal`
    struct GenericVal<T>(T,);

    // GenericVal的 `impl`，此处显式地指定了类型参数
    // 显式指定`f64`类型
    impl GenericVal<f64>{}
    // 显式指定`S`类型
    impl GenericVal<S>{}
    // `<T>` 必须在类型之前写出来，以使类型 `T` 代表泛型。
    impl <T> GenericVal<T>{}
*/
#[cfg(test)]
mod tests {
    //  具体类型 `S`
    struct S {
        val: f64,
    }

    struct GenS<T> {
        gen_val: T,
    }

    // S 的 `impl`
    impl S {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // GenS 的 `impl`，指定 `T` 是泛型类型
    impl<T> GenS<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }


    #[test]
    fn test_generics_implementation() {
        let s = S { val: 10.24 };
        let gen_s = GenS { gen_val: "michael.w" };
        println!("{}", s.val);
        println!("{}", gen_s.gen_val);
    }
}
