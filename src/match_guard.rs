//可以加上 match 守卫（guard） 来过滤分支。
#[cfg(test)]
mod tests {
    #[test]
    fn test_match_guard() {
        let pair = (1024, -1024);

        match pair {
            (x, y) if x == y => println!("These are twins"),
            // ^ `if` 条件部分是一个守卫
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _ => println!("No correlation..."),
        }
    }
}