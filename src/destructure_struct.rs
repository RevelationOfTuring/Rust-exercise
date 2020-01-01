//解构 struct

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct S1 {
        field1: i32,
        field2: String,
        field3: (u32, f64),
        field4: S2,
    }

    #[derive(Debug)]
    struct S2 {
        x: u32,
        y: f64,
    }

    #[test]
    fn match_destructure_struct() {
        let s1 = S1 { field1: 1, field2: String::from("michael.w"), field3: (2, 3.3), field4: S2 { x: 4, y: 5.5 } };
        println!("{:?}", s1);
        // 解构结构体的成员
        // field与变量同名
        let S2 { x, y } = s1.field4;
        println!("{} {}", x, y);
        // field与变量不同名
        let S2 { x: i, y: j } = s1.field4;
        println!("{} {}", i, j);

        // 也可以忽略某些变量
        let S1 { field1, .. } = s1;
        println!("{}", field1); // 1
        // 注：只能忽略后面的，不能忽略前面的。即，
        // let S1 { .. , filed4 } = s1; 会报错

    }
}