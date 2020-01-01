//  在 match 中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。
// match 提供了 @ 符号来绑定变量到名称：

#[cfg(test)]
mod tests {

    fn age() -> u32 {
        19
    }

    #[test]
    fn test_match_binding() {
        match age(){
            0             => println!("I'm not born yet I guess"),
            // 可以直接 `match` 1 ... 12，但怎么把岁数打印出来呢？
            // 相反，在 1 ... 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
            n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
            n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
            // 不符合上面的范围。返回结果。
            n             => println!("I'm an old person of age {:?}", n),
        }
    }
}