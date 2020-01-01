// 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
#[allow(dead_code)]
#[cfg(test)]
mod tests {
    enum Color {
        // 这三个取值仅由它们的名字（而非类型）来指定。
        Red,
        Blue,
        Green,
        // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    #[test]
    fn test_match_enum() {
        let color = Color::HSL(122, 17, 40);

        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) =>
                println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) =>
                println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) =>
                println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) =>
                println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) =>
                println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                         c, m, y, k),
            // 不需要其它分支，因为所有的情形都已覆盖
        }
    }
}