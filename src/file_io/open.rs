/*
    打开文件open

    open 静态方法能够以只读模式（read-only mode）打开一个文件。
    File 拥有资源，即文件描述符（file descriptor），它会在自身被 drop 时关闭文件。
*/
#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs::File;
    use std::error::Error;
    use std::io::Read;

    #[test]
    fn test_open() {
        // 创建指向所需的文件的Path
        let path = Path::new("/Users/oker/CLionProjects/rust-exercise/src/file_io/rust.txt");
        let display = path.display();

        // 以只读方式打开路径，返回io::Result<File>
        let mut file = match File::open(&path) {
            // io::Error 的 description 方法返回一个描述错误的字符串。
            Err(err) => panic!("file {} is opened failed: {}", display, err.description()),
            Ok(file) => file
        };

        // 读取文件内容到一个字符串中，返回io::Result<usize>
        let mut text = String::new();
        match file.read_to_string(&mut text) {
            Err(err) => panic!("file {} is read failed: {}", display, err.description()),
            Ok(_) => print!("text in {} is: \n{}", display, text),
        }

        // file离开作用域时，hello.txt文件将被关闭。
    }
}