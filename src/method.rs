// 方法（method）是依附于对象的函数。
// 这些方法通过关键字 self 来访问对象中的数据和 其他。
// 方法在 impl 代码块中定义。

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        // 这是一个静态方法（static method）
        // 静态方法不需要被实例调用
        // 这类方法一般用作构造器（constructor）
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        // 另外一个静态方法，需要两个参数：
        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        // 这是一个实例方法（instance method）
        // `&self` 是 `self: &Self` 的语法糖（sugar），
        // 其中 `Self` 是方法调用者的类型。
        // 在这个例子中 `Self` = `Rectangle`
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2).abs() + (y1 - y2).abs()) * 2.0
        }

        // 这个方法要求调用者是可变的
        // `&mut self` 为 `self: &mut Self` 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    #[test]
    fn match_method_1() {
        let rectangle = Rectangle {
            p1: Point::origin(),
            p2: Point::new(3.0, 4.0),
        };

        // 实例方法通过点运算符来调用
        // 注意第一个参数 `&self` 是隐式传递的，亦即：
        // `rectangle.perimeter()` 等价于 `Rectangle::perimeter(&rectangle)`
        println!("{} {}", rectangle.perimeter(), rectangle.area());

        // square是可改变的
        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };

        square.translate(1.0, 0.0);
        println!("{:?}", square);
    }

    #[derive(Debug)]
    // `Pair` 拥有资源：两个堆分配的整型
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 这个方法会 “消耗” 调用者的资源
        // `self` 为 `self: Self` 的语法糖
        fn destroy(self) {
            // 模式匹配
            let Pair(first_box, second_box) = self;
            println!("{} {}", first_box, second_box);
            // `first_box` 和 `second_box` 离开作用域后释放
        }
    }

    #[test]
    fn match_method_2() {
        let pair = Pair(Box::new(1), Box::new(2));
        pair.destroy();

        // 编译报错！前面的 `destroy` 调用 “消耗了” `pair`
        // println!("{:?}",pair);
    }
}