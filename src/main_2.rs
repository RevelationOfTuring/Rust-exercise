//After checking the output of the above example, use the Point2D struct as a guide to add
// a Complex struct to the example. When printed in the same way, the output should be:
//
//Display: 3.3 + 7.2i
//Debug: Complex { real: 3.3, imag: 7.2 }

use std::fmt::{Display, Formatter, Error};

fn main_2() {
    let c = Complex { real: 3.3, imag: 7.2 };
    println!("{}", c);
    println!("{:?}", c);
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}