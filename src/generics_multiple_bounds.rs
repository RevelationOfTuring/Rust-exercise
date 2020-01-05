/*
    多重约束（multiple bounds）可以用 `+` 连接。
    和平常一样，类型之间使用 `,` 隔开
*/
#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display};

    // 要求传入的参数必须同时实现Debug和Display
    fn multiple_prints<T: Debug + Display>(t: &T) {
        println!("Debug: {:?}", t);
        println!("Display: {}", t);
    }

    // T和U两个类型都需要各自实现Debug
    fn multiple_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: {:?}", t);
        println!("u: {:?}", u);
    }

    #[test]
    fn test_generics_multiple_bounds() {
        let string = "michael.w";
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let vec = vec![1, 2, 3, 4, 5];

        // str类型已经实现了Display和Debug
        multiple_prints(&string);
        // 数组类型没有实现Display trait，所以编译报错
//        multiple_prints(&arr);

        // vec和数组都实现了Debug
        multiple_types(&arr, &vec);
    }
}