/*
Activity1
Add a function rect_area which calculates the area of a rectangle (try using nested destructuring).

Activity2
Add a function square which takes a Point and a f32 as arguments,
and returns a Rectangle with its lower left corner on the point,
and a width and height corresponding to the f32.
*/

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }


    // Activity1_1
//fn rect_area(Rectangle { top_left: Point { x: x1, y: y1 }, bottom_right: Point { x: x2, y: y2 } }: Rectangle) -> f32 {
//    (x2 - x1) * (y2 - y1)
//}

    // Activity1_2
    fn rect_area(r: &Rectangle) -> f32 {
        (r.bottom_right.x - r.top_left.x) * (r.bottom_right.y - r.top_left.y)
    }

    // Activity2
    fn square(p: Point, area: f32) -> Rectangle {
        let width = area / 2f32;
        let x_new = p.x + width;
        Rectangle { top_left: p, bottom_right: Point { x: x_new, y: 2f32 } }
    }

    #[test]
    fn test_tuples() {
        let r = Rectangle { top_left: Point { x: 1.1, y: 1.1 }, bottom_right: Point { x: 2.2, y: 4.4 } };
        // Activity1
        println!("{}", rect_area(&r));

        // Activity2
        println!("{:?}", square(Point { x: 1f32, y: 2f32 }, 100f32))
    }
}

