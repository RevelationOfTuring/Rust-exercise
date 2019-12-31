#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[test]
    fn test_structures() {
        let point = Point { x: 1.024, y: 2.048 };
        // Make a new point by using struct update syntax to use the fields of our other one
        let point1 = Point { x: 4.096, ..point };
        // `point1.y` will be the same as `point.y` because we used that field from `point`
        println!("{:?}", point1);
    }
}

