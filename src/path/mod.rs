/*
    Path

    Path结构体代表了底层文件系统的文件路径。
    Path分为两种：
        - posix::Path，针对类UNIX系统
        - 以及 windows::Path，专门用于Windows。

    prelude会选择并输出符合平台类型的Path种类
    注：prelude是Rust自动地在每个程序中导入的一些通用的东西，这样我们就不必每写一个程序就手动导入一番。

    Path可从OsStr类型创建，并且它提供数种方法，用于获取路径指向的文件/目录的信息。

    需要注意的是：Path在内部并不是用UTF-8字符串表示的，而是存储为Vec<u8>。
    因此，将Path转化成&str并非零开销的（free），且可能失败（因此它返回一个Option）。
*/
#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_path() {
        // 利用&str创建一个Path
        // 当前路径
        let path = Path::new(".");
        println!("{:?}", path);
        // 打印输出："."

        // display方法返回一个可显示（showable）的结构体
        let display = path.display();
        println!("{:?}", display);
        // 打印输出："."


        // 绝对路径
        let path = Path::new("/Users/oker/CLionProjects/rust-exercise/src");
        println!("{:?}", path);
        let display = path.display();
        // "/Users/oker/CLionProjects/rust-exercise/src"
        println!("{:?}", display);
        // "/Users/oker/CLionProjects/rust-exercise/src"


        // join方法使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
        let new_path = path.join("a").join("b");
        println!("{:?}", new_path);
        // 打印："/Users/oker/CLionProjects/rust-exercise/src/a/b"

        // 此时的new_path类型是PathBuf,可理解为一个Vec<u8>，并不是&str。
        // 路径转为&str，不是零开销！
        match new_path.to_str() {
            // to_str()返回值为Option<&str>类型
            Some(s) => println!("new path is {}", s),
            None => panic!("new path is not a valid UTF-8 sequence")
        }
    }
}