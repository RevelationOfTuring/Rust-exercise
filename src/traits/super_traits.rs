/*
    Rust中没有继承关系。

    但是可以定义一个trait，使它成为其他trait的超集（superset）。
*/
#[cfg(test)]
mod tests {
    trait Person {
        fn name(&self) -> String;
    }

    // Student是Person的supertrait
    // 实现Student trait的struct将必须同时实现Person trait
    trait Student: Person {
        fn university(&self) -> String;
    }

    trait Programmer {
        fn programming_language(&self) -> String;
    }

    // super trait of both Programmer and Student
    trait ComputerScienceStudent: Programmer + Student {
        fn is_996(&self) -> bool;
    }

    fn computer_science_student_info(student: &dyn ComputerScienceStudent) -> String {
        format!(
            "NAME: {}\nUNIVERSITY: {}\nLANGUAGE: {}\nIS_996: {}",
            student.name(),
            student.university(),
            student.programming_language(),
            student.is_996(),
        )
    }

    #[test]
    fn test_super_traits() {
        // 实现类
        struct S<'a> {
            name: &'a str,
        }

        impl<'a> ComputerScienceStudent for S<'a> {
            fn is_996(&self) -> bool {
                true
            }
        }

        // 注：不能在impl<'a> ComputerScienceStudent for S<'a>的代码块中实现其他的trait方法
        // 必须分别impl各个trait
        /*
            如下实现也不可以，编译会报错：
            impl<'a> S<'a> {
                各个trait的方法实现...
            }
        */

        // 必须分开单独实现！！！
        impl<'a> Student for S<'a> {
            fn university(&self) -> String {
                String::from("Blue Fly")
            }
        }

        impl<'a> Programmer for S<'a> {
            fn programming_language(&self) -> String {
                String::from("Rust")
            }
        }

        impl<'a> Person for S<'a> {
            fn name(&self) -> String {
                String::from(self.name)
            }
        }

        let student = S { name: "Michael.W" };
        // 由于函数签名中参数类型为 &dyn ComputerScienceStudent，所以实参需要传一个引用
        println!("{}", computer_science_student_info(&student));
    }
}