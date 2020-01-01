/*
闭包天生就是灵活的，它会自动满足函数功能的要求，使得闭包不需要类型说明就可以工作。
这允许变量捕获（capture）灵活地适应使用场合，既可移动（move）又可借用（borrow）变量。

-----------------------------------------------------------------------
闭包可以通过以下手段捕获变量：
    - 通过引用：&T
    - 通过可变引用：&mut T
    - 通过值：T

闭包更倾向于通过引用来捕获变量，并且只在被要求时才使用其他手段。
*/

#[cfg(test)]
mod tests {
    #[test]
    fn test_function_closures_capturing_1() {
        let color = "green";
        // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和闭包本身存储到 `print` 变量中。
        // `color` 会一直保持被借用状态直到`print` 离开作用域。
        // `println!` 只需传引用就能使用，而这个闭包捕获的也是`变量的引用`，因此无需进一步处理就可以使用 `println!`。
        let closure_print = || println!("color : {}", color);

        // 调用闭包，闭包又借用 `color`。
        closure_print();
        closure_print();


        let mut count = 0;
        // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
        // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
        // 该闭包立即借用 `count`。
        //
        // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
        // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
        let mut closure_inc = || {
            count += 1;
            println!("{}", count)
        };

        // 调用闭包
        closure_inc();  //1
        closure_inc();  //2

        // 变量count通过打印证明其值确实被修改了
//        println!("{}", count);  //2

        let reborrow = &mut count;
        // 下面在执行闭包会报错，因为count的可变借用给reborrow了，再调用closure_inc闭包时候，无法再使用
        // 保存在闭包中的&mut count。
        // 因为一个变量在同一时刻只允许存在一个&mut。
//        closure_inc();
    }

    #[test]
    fn test_function_closures_capturing_2() {
        use std::mem;
        let movable = Box::new(1024);
        // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。
        // 这种情况下，可复制类型将会复制给闭包，从而原始值不受影响。
        // 不可复制类型必须移动（move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。

        let consume = || {
            println!("{}", movable);
            // movable是Box指针，为不可复制类型。
            mem::drop(movable);
            // 执行mem::drop(movable);后，将movable变量本身move到了闭包中
        };

        // `consume` 消耗了该变量，所以该闭包只能调用一次。
        consume();
        // 再调用闭包consume时，已经无法捕获到movable了。编译报错。
//        consume();
    }

    #[test]
    fn test_function_closures_capturing_3() {
        /*
        在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权
        */

        // `Vec` 在语义上是不可复制的
        let v = vec![1, 2, 3];

        // v的所有权被强制转移给闭包contains
        let contains = move |number| v.contains(number);

        println!("{}", contains(&1));    // true
        println!("{}", contains(&100));  // false

        // 在闭包外已经无法再使用v了（无论读写）。因为借用检查不允许在变量被move之后继续使用它
//        println!("{:?}", v);
    }

    #[test]
    fn test_function_closures_capturing_4() {
        // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `v`，即&v。
        // 因此之后`v` 仍然可用，取消上面的注释也不会导致错误。
        let v = vec![1, 2, 3];
        let contains = |number| v.contains(number);
        println!("{}", contains(&1));    // true
        println!("{}", contains(&100));  // false

        // v变量仍可用
        println!("{:?}", v);
    }
}
