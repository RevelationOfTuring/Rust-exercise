/*
结构体的字段也是一个可见性的层次。
字段默认拥有私有的可见性，也可以加上 pub 修饰语来重载该行为。
只有从结构体被定义的模块之外访问其字段时，这个可见性才会起作用，其意义是隐藏信息（即封装，encapsulatoin）。
*/

mod my_mod {
    // 一个公有的结构体，带有一个公有的字段（类型为泛型 `T`）
    pub struct OpenBox<T> {
        // 公有的字段contents,类型T
        pub contents: T,
    }

    // 一个公有的结构体，带有一个私有的字段（类型为泛型 `T`）
    pub struct ClosedBox<T> {
        // 私有字段
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 一个公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::module_struct_visibility::my_mod;

    #[test]
    fn test_module_struct_visibility() {
        // 带有公有字段的公有结构体，可以像平常一样直接创建
        let open_box = my_mod::OpenBox { contents: "public information" };
        // 并且它们的字段可以正常访问到
        println!("{}", open_box.contents);

        // 带有私有字段的公有结构体不能使用字段名来构造。
        // 报错！`ClosedBox` 含有私有字段contents。
//        let close_box = my_mod::ClosedBox { contents: "classified information" };

        // 不过带有私有字段的结构体可以使用公有的构造器来创建。
        let _close_box = my_mod::ClosedBox::new("classified information");

        // 并且一个结构体中的私有字段不能访问到。
        // 报错！`content` 字段是私有的。
//        println!("{}",close_box.contents);
    }
}