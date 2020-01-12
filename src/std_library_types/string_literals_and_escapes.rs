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
}