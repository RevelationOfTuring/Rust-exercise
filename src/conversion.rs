#[cfg(test)]
mod tests {
//    use super::*;

    // 类型转换
// Rust 使用 trait 解决类型之间的转换问题。
// 最一般的转换会用到 From 和 into 两个 trait。
// 不过，即便常见的情况也可能会用到特别的 trait，尤其是从 String 转换到别的类型，以及把别的类型转换到 String 时。
    #[test]
    fn test_conversion() {
// From 和 Into 两个 trait 是内在地联系着的，事实上这是它们的实现的重要部分：
// 如果能把类型 A 转换成类型 B，那么很容易相信我们也能把类型 B 转换成类型 A。

//      From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，
//      因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，
//      规定了原生类型及其他常见类型的转换功能。

        // 将str转换成String
        let my_str = "Michael.W";
        let my_string = String::from(my_str);
        println!("{}", my_string);

        // 自定义类型转换机制
        use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        // 将i32转换为自定义的Number类型
        impl From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        let n = Number::from(1024);
        println!("{:?}", n);

//        Into trait 就是把 From trait 倒过来而已。也就是说，如果你为你的类型实现了 From，那么同时你也就免费获得了 Into。
        // 需要自己手动指明要转换的类型——Number
        let n1: Number = 1024.into();
        println!("{:?}", n1)

// 使用 Into trait 通常要求指明要转换到的类型，因为编译器大多数时候不能推断它。不
// 过考虑到我们免费获得了 Into，这点代价不值一提。
    }

    // ToString和FromStr
    #[test]
    fn test_ToString_and_FromStr() {
        // 要把任何类型转换成 String，只需要实现那个类型的 ToString trait。
        struct Circle {
            radius: i32
        }

        impl std::string::ToString for Circle {
            fn to_string(&self) -> String {
                format!("Circle of radius {:?}", self.radius)
            }
        }

        let c = Circle { radius: 12 };
        println!("{}", c.to_string());

        // 我们经常需要把字符串转成数字。完成这项工作的标准手段是用 parse 函数。
        // 我们得提供要转换到的类型，这可以通过不使用类型推断，或者用 “涡轮鱼” 语法（turbo fish，<>）实现。
        // 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型。
        // 标准库中已经给无数种类型实现了 FromStr。如果要转换到用户定义类型，只要手动实现 FromStr 就行。

        // 需要手动提供要转换成的类型
        let parsed: i32 = "1024".parse().unwrap();
        println!("{}", parsed);

        // 实现FromStr trait
        use std::num::ParseIntError;
        impl std::str::FromStr for Circle {
            type Err = ParseIntError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let strs: Vec<&str> = s.trim().split(' ').collect();
                let r = strs[3].parse::<i32>()?;
                Ok(Circle { radius: r })
            }
        }

        let parsed_circle: Circle = "Circle of radius 1025".parse().unwrap();
        println!("{}", parsed_circle.radius);
    }
}