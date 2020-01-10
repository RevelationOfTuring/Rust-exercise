/*
    借用

    多数情况下，我们更希望能访问数据，同时不取得其所有权。（需要只读权限）
    为实现这点，Rust 使用了借用（borrowing）机制。

    对象可以通过引用（&T）来传递，从而取代通过值（T）来传递。

    编译器（通过借用检查）静态地保证了引用总是指向有效的对象。
    也就是说，当存在引用指向一个对象时，该对象`不能被销毁`。
*/
mod mutability;
mod freezing;
mod aliasing;

#[cfg(test)]
mod tests {
    // 此函数取得一个 box 的所有权并销毁它
    fn eat_box(b: Box<i32>) {
        println!("Destroying box that contains {}", b);
    }

    // 此函数借用了一个 i32 类型
    fn borrow_i32(i: &i32) {
        println!("This int is: {}", i);
    }

    #[test]
    fn test_borrowing() {
        // 创建一个装箱的 i32 类型，以及一个存在栈中的 i32 类型。
        let (box_i32, i) = (Box::new(1024), 2048);

        // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用。
        // 译注：请注意函数自身就是一个作用域，因此下面两个函数运行完成以后，在函数中临时创建的引用也就不复存在了。

        borrow_i32(&box_i32);
        borrow_i32(&i);
        // borrow_i32的参数是借用，所以box_i32和i在函数外还可以使用
        println!("box_i32: {}, i: {}", box_i32, i);

        {
            // 取得一个对 box 中数据(i32)的引用
            let ref_to_box: &i32 = &box_i32;

            // 报错：由于前面ref_to_box借用了box_i32，
            // 所以在eat_box的传参时，不能将box_i32的所有权转移到函数内
//            eat_box(box_i32);


            // 注：如果此处不借用，则在上一行的代码中，
            // eat_box(boxed_i32)可以将 `boxed_i32` 销毁。
            borrow_i32(ref_to_box);

            // ref_to_box 离开作用域且不再被借用
        }

        // ref_to_box已经被释放，所以对box_i32的借用已经不存在
        eat_box(box_i32);
    }
}