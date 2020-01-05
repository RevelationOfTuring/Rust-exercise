/*
    newtype 惯用法（译注：即为不同种类的数据分别定义新的类型）能保证在编译时，
    提供给程序的都是正确的类型。
*/

//  比如说，实现一个 “年龄认证” 函数，它要求输入必须是 Years 类型。

#[cfg(test)]
mod tests {
    // 下面的Years和Days就是newtype 惯用法
    // 为不同种类的数据分别定义新的类型(struct)，原先的数据类型都是i64.
    struct Years(i64);

    struct Days(i64);

    impl Years {
        // 单位年转换为天
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            // 地板除
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    #[test]
    fn test_generics_new_type_idiom() {
        let age_years = Years(10);
        let age_days = age_years.to_days();

        println!("{}", old_enough(&age_years));
        println!("{}", old_enough(&age_days.to_years()));

        // 编译报错
//        println!("{}", old_enough(&age_days));
    }
}