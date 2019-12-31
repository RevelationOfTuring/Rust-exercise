// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

#[cfg(test)]
mod tests {
    use std::borrow::ToOwned;

    enum WebEvent {
        // An `enum` may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and returns nothing.
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    #[test]
    fn test_enums() {
        let pressed = WebEvent::KeyPress('w');
        let paste = WebEvent::Paste("michael,w".to_owned());
        let click = WebEvent::Click { x: 1024, y: 2048 };
        let (page_load, page_unload) = (WebEvent::PageLoad, WebEvent::PageUnload);

        inspect(page_load);
        inspect(page_unload);
        inspect(pressed);
        inspect(paste);
        inspect(click);
    }

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // Creates a type alias
    type V = VeryVerboseEnumOfThingsToDoWithNumbers;

    // The most common place you'll see this is in impl blocks using the Self alias.
    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                // Self is alias of VeryVerboseEnumOfThingsToDoWithNumbers
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    enum Work {
        Multiply,
        Divide,
    }

    // enum can also be used as C-like enums.
    enum Number {
        Zero,
        One,
        Two,
    }

    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    #[test]
    fn test_type_aliases() {
        // We can refer to each variant via its alias, not its long and inconvenient name
        let x = V::Subtract;

        println!("{}", x.run(10, 5));

        // Explicitly `use` each name so they are available without manual scoping.
        use VeryVerboseEnumOfThingsToDoWithNumbers::{Add, Subtract};
        let _x = Add;
        let _y = Subtract;
        // Automatically `use` each name inside `Work`.
        use Work::*;
        let _x = Divide;
        let _y = Multiply;

        // `enums` can be cast as integers.
        println!("{:?}", Number::Zero as i32);  // 0
        println!("{:?}", Number::One as i32);   // 1
        println!("{:?}", Number::Two as i32);   // 2
        println!("{:06x}", Color::Red as i32);  // ff0000
        println!("{:06x}", Color::Blue as i32); // 0000ff
        println!("{:06x}", Color::Green as i32);// 00ff00
    }
}

