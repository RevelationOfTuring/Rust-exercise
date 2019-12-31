//Try changing the program so that the index of each element in the vector is also printed. The new output should look like this:
//
//[0: 1, 1: 2, 2: 3]

#[cfg(test)]
mod tests{
    use std::fmt::{Display, Formatter, Error};

    struct List(Vec<i32>);

    impl Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "[")?;

            for (count, v) in self.0.iter().enumerate() {
                if count != 0 {
                    write!(f, ",")?;
                }
                write!(f, "{}:{}", count, v)?;
            }
            write!(f, "]")
        }
    }

    #[test]
    fn test_display_vec(){
        let l = List(vec![10, 20, 30, 40, 50]);
        println!("{}", l);
    }
}