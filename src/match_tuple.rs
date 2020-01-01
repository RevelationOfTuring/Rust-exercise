//元组可以在 match 中解构
#[cfg(test)]
mod tests {
    #[test]
    fn test_match() {
//        let pair = (1, 1);
        let pair = (0, 1);
        match pair {
            (0, x) => println!("First is `0` and `y` is `{:?}`", x),
            (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
            _ => println!("It doesn't matter what they are"),
        }
    }
}