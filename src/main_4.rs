/*
Activity
Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:

( 1.1 1.2 )
( 2.1 2.2 )
You may want to refer back to the example for print display.

Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:

println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
results in the output:

Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
*/
use std::fmt::{Display, Formatter, Error};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// Activity1
impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

// Activity2
fn transpose(matrix: &Matrix) -> Matrix {
    Matrix { 0: matrix.0, 1: matrix.2, 2: matrix.1, 3: matrix.3 }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // Activity1
    println!("{:?}", matrix);
    println!("{}", matrix);

    // Activity2
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(&matrix));
}