// 列出几个标准库中使用闭包的例子
mod tests {
    #[test]
    fn test_function_closures_iter_any() {

//        Iterator::any是一个函数，若传给它一个迭代器（iterator），
//        当其中任一元素满足谓词(predicate)时它将返回true，否则返回false
//        (译注：谓词是闭包规定的， true/false是闭包作用在元素上的返回值)。
        /*
                Iterator::any的签名如下：

                pub trait Iterator {
                    // 被迭代的类型。
                    type Item;

                    // `any` 接受 `&mut self` 参数（译注：回想一下，这是 `self: &mut Self` 的简写）
                    // 表明函数的调用者可以被借用和修改，但不会被消耗。
                    fn any<F>(&mut self, f: F) -> bool where
                    // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
                    // `Self::Item` 指明了被捕获变量的类型（译注：是迭代器的元素本身的类型）
                        F: FnMut(Self::Item) -> bool {}

                    // 译注：原文说 `Self::Item` 表明变量是通过`值`传递给闭包的，这是说错了。
                    // `FnMut` 就表示闭包只能通过引用捕获变量。把类型为 `T` 的变量作为闭包
                    // 的参数不代表闭包会拿走它的值，也可能是拿走它的引用。
                }
        */

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        // 对vec的 `iter()` 举出 `&i32`。（通过用 `&x` 匹配）把它解构成 `i32`。
        // 译注：注意 `any` 方法会自动地把 `vec.iter()` 举出的迭代器的元素一个个地
        // 传给闭包。因此闭包接收到的参数是 `&i32` 类型的。
        println!("{}", v1.iter().any(|&x| x == 2));

        // 对 vec 的 `into_iter()` 举出 `i32` 类型。无需解构。
        println!("{}", v2.into_iter().any(|x| x == 2));

        let array1 = [1, 2, 3];
        let array2 = [4, 5, 6];
        // 对数组的 `iter()` 举出 `&i32`。
        println!("{}", array1.iter().any(|&x| x == 2));

        // 对数组的 `into_iter()` 通常举出 `&i32`。与vec不同
        println!("{}", array2.into_iter().any(|&x| x == 2));
    }

    #[test]
    fn test_function_closures_iter_find() {
        // Iterator::find是一个函数，在传给它一个迭代器时，将用`Option类型`返回`第一个`满足谓词的元素。
        /*
        Iterator::any的签名如下：
        pub trait Iterator {
            // 被迭代的类型。
            type Item;

            // `find` 接受 `&mut self` 参数，表明函数的调用者可以被借用和修改，
            // 但不会被消耗。
            fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
            // `FnMut` 表示被捕获的变量最多只能被修改，而不能被消耗。
            // `&Self::Item` 指明了被捕获变量的类型（译注：是对迭代器元素的引用类型）
                P: FnMut(&Self::Item) -> bool {}
        }

        */

        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        // 对v1的`iter()`举出`&i32`类型
        let mut iter = v1.iter();
        // 对v2的`into_iter()`举出`i32`类型
        let mut into_iter = v2.into_iter();

        // 对迭代器举出的元素的引用是 `&&i32` 类型。解构成`i32`类型。
        // 译注：注意 `find` 方法会把迭代器元素的引用传给闭包。
        // 迭代器元素自身是`&i32`类型，所以传给闭包的是`&&i32`类型。
        println!("{:?}", iter.find(|&&x| x == 2));  // Some(2)

        // 对迭代器举出的元素的引用是 `&i32` 类型。解构成 `i32` 类型。
        println!("{:?}", into_iter.find(|&x| x == 2));  // None

        let array1 = [1, 2, 3];
        let array2 = [4, 5, 6];

        // 对数组的 `iter()` 举出 `&&i32`。
        println!("{:?}", array1.iter().find(|&&x| x == 2));
        // 对数组的 `into_iter()` 通常举出 `&i32`。
        println!("{:?}", array2.into_iter().find(|&&x| x == 2));
    }
}
