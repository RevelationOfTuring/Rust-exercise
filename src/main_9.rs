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

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient name
    let x = V::Subtract;

    println!("{}", x.run(10, 5));

    // Explicitly `use` each name so they are available without manual scoping.
    use crate::VeryVerboseEnumOfThingsToDoWithNumbers::{Add, Subtract};
    let x = Add;

    // Automatically `use` each name inside `Work`.
    use crate::Work::*;
    let x = Divide;

    // `enums` can be cast as integers.
    println!("{:?}", Number::Zero as i32);  // 0
    println!("{:?}", Number::One as i32);   // 1
    println!("{:?}", Number::Two as i32);   // 2
    println!("{:06x}", Color::Red as i32);  // ff0000
    println!("{:06x}", Color::Blue as i32); // 0000ff
    println!("{:06x}", Color::Green as i32);// 00ff00

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