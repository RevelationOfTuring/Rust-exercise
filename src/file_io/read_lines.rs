/*
    文件读取行

    方法lines()在文件的行上返回一个迭代器。

    File::open 需要一个泛型`AsRef<Path>`。
    这正是 read_lines() 期望的输入。
*/

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io;
    use std::io::BufRead;

    #[test]
    fn test_read_lines() {
        let file = File::open("src/file_io/rust.txt").unwrap();
        // 生成一个行迭代器
        let lines_iterator = io::BufReader::new(file).lines();
        // 迭代出每一行字符串内容
        for (num, line) in lines_iterator.enumerate() {
            println!("Line {}: ", num);
            if let Ok(line_string) = line {
                println!("{}", line_string);
            }
        }
    }

    /*
        注：
            这个过程比在内存中创建String更有效，特别是处理更大的文件。
    */
}