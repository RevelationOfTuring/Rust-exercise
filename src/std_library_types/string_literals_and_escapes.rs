/*
    字面量与转义字符

    书写含有特殊字符的字符串字面量有很多种方法。
    它们都会产生类似的 &str，所以最好选择最方便的写法。

    类似地，字节串（byte string）字面量也有多种写法，它们都会产生 &[u8; N] 类型。

    通常特殊字符是使用反斜杠字符 \ 来转义的。
    这样就可以在字符串中写入各种各样的字符，甚至是不可打印的字符以及不知道如何输入的字符。
    ps.如果需要写出一个反斜杠字符，再用另一个反斜杠来转义它就可以，像这样：\\。

    字面量中出现的字符串或字符定界符必须转义："\""（在字符串字面量中表示一个"）、'\''（字符'的表示）
*/
#[cfg(test)]
mod tests {
    #[test]
    fn test_string_literals_and_escapes() {
        // 通过转义，可以用`十六进制值`来表示字节
        let byte_escape = "I'm writing \x52\x75\x73\x74!";
        assert_eq!(byte_escape, "I'm writing Rust!");
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // 也可以使用 Unicode 码位表示
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
        println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

        // 遇到字面量特别长的&str时：
        // \可以进行代码层面的换行（字面量中并无影响）
        let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
        // 注：在字面量中输入回车，就相当于加入了一个\n
        println!("{}", long_string);
        assert_eq!(long_string,
                   "String literals
                        can span multiple lines.
                        The linebreak and indentation here -><- can be escaped too!"
        );
    }

    #[test]
    fn test_raw_string() {
        // 当有太多需要转义的字符时，直接原样写出会更便利。
        // 这时可以使用原始字符串（raw string）。
        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        // r"..."中的内容会原封不动的存入字面量（同python）
        println!("{}", raw_str);
        assert_eq!(raw_str, "Escapes don't work here: \\x3F \\u{211D}");

        // 如果要在原始字符串中写引号，请在两边加一对 #
        let raw_str_quotation = r#"I said: "This is Michael.W!""#;
        println!("{}", raw_str_quotation);
        assert_eq!(raw_str_quotation, "I said: \"This is Michael.W!\"");

        // 如果字符串中需要写 "#，那就在定界符中使用更多的 #
        // 可使用的 # 的数目`没有限制`
        // 很好理解，如果在raw string中最多有连着的n个#，那么raw string两侧就写n+1个连着的#
        let raw_str_longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", raw_str_longer_delimiter);
        assert_eq!(raw_str_longer_delimiter, "A string with \"# in it. And even \"##!")
    }

    #[test]
    fn test_byte_string() {
        // 切记：&str 和 String 都必须是合法有效的 UTF-8 序列
        // 如果想要非 UTF-8 字符串，或者一个字节数组（字节数组中大部分为文本）
        // 可以使用`字节串`，byte string

        // 这是个字节数组，并不是一个 &str
        // 字符串字面量前加b，表示为其类型未字节数组，并非&str
        let byte_string = b"This is a byte string";

        // 需要注意的是：字节数组没有实现Display trait
//        println!("{}", byte_string);
        println!("{:?}", byte_string);
        // 打印：[84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 121, 116, 101, 32, 115, 116, 114, 105, 110, 103]

        // 字节串可以使用单字节的转义字符
        let escaped = b"\x52\x75\x73\x74 as bytes";
        println!("{:?}", escaped);
        // 打印：[82, 117, 115, 116, 32, 97, 115, 32, 98, 121, 116, 101, 115]

        // 但是不能使用unicode码的转义字符
        // 编译报错：error: unicode escape sequences cannot be used as a byte or in a byte string
//        let escaped_unicode = b"\u{211D} is not allowed";

        // 原始字节串和原始字符串的写法一样，在字面量前面加br
        let raw_byte_string = br"\\\n";
        println!("{:?}", raw_byte_string);
        // 打印：[92, 92, 92, 110]
        // 字面量中的所有字符都没有进行转义，然后放到了字节数组中

        let raw_byte_string = br#"\""\n"#;
        // 字节数组中包含"
        println!("{:?}", raw_byte_string);
        // 打印：[92, 34, 34, 92, 110]

        /***** 把字节串bytestring转换为 &str 可能失败 *****/
        let raw_byte_string = br"\u{211D}";
        println!("{:?}", raw_byte_string);
        // 打印：[92, 117, 123, 50, 49, 49, 68, 125]

        // 字节串 -> &str
        // 成功
        if let Ok(converted_str) = std::str::from_utf8(raw_byte_string) {
            println!("byte string -> &str: {}", converted_str);
            assert_eq!(converted_str, r"\u{211D}");
        }

        // 当字节串不使用utf-8编码时
        // 例子：采用 SHIFT-JIS 编码的 "ようこそ"
        let byte_string_not_uft8 = b"\x82\xe6\x82\xa8\x82\xb1\x82";

        // 再看转换结果
        match std::str::from_utf8(byte_string_not_uft8) {
            Ok(s) => println!("convert successfully: {}", s),
            Err(e) => println!("convert failed: {}", e),
        }
        // 打印：convert failed: invalid utf-8 sequence of 1 bytes from index 0
        // 转换失败
    }

    /*
        ps.若需要在编码间进行转换，请使用 encoding crate（一个第三方crate）
        传送门：https://crates.io/crates/encoding

        Rust手册：详细书写字符串字面量和转义字符的方法
        传送门：https://doc.rust-lang.org/reference/tokens.html
    */
}