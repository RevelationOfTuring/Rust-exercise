//Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:

// const: An unchangeable value (the common case).
// static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.

#[cfg(test)]
mod tests {
    // An unchangeable value
    static LANGUAGE: &str = "Michael.w";
    // A possibly mutable variable with 'static lifetime.
    const THRESHOLD: i32 = 1024;
    // global mutable variable
    static mut G1: i32 = 1;


    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    #[test]
    fn test_constants() {
        let n = 1025;

        println!("{}", LANGUAGE);
        println!("{}", THRESHOLD);
        println!("{} is {} ", n, if is_big(n) { "big" } else { "small" });

        // change global variable
        unsafe {
            println!("{}", G1); // 1
            G1 = 2;
            println!("{}", G1); // 2
        }

        // compile error
//    THRESHOLD = 1;
    }
}