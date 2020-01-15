/*
    创建文件create

    create静态方法以`只写`模式（write-only mode）打开一个文件。

    若文件已经存在，则旧内容将被销毁。否则，将创建一个新文件。
*/
static TEXT: &str = "This is a file for create test!!";

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::fs::File;
    use std::error::Error;
    use std::io::Write;
    use crate::file_io::create::TEXT;

    #[test]
    fn test_create() {
        // 创建路径
        let path = Path::new("file_write.txt");
        // 注：该文件会在与Cargo.lock和Cargo.toml文件同级目录
        let display = path.display();

        // 以只写模式打开文件，返回io::Result<File>
        let mut file = match File::create(&path) {
            Err(err) => panic!("file {} is created failed: {}", display, err.description()),
            Ok(file) => file
        };

        // 将TEXT字符串写进file，返回io::Result<()>
        // 注：&str对象的.as_bytes方法返回胖指针：&[u8]
        match file.write_all(TEXT.as_bytes()) {
            Err(err) => panic!("write to file {} failed: {}", display, err),
            Ok(_) => println!("write to {} successfully", display)
        }
    }
}