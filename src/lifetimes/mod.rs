/*
    生命周期

    编译器中的`借用检查器`用生命周期来保证所有的借用都是有效的。
    确切地说，一个变量的生命周期在它创建的时候开始，在它销毁的时候结束。
    虽然生命周期和作用域经常被一起提到，但它们并不相同。

    考虑这种情况，我们通过 & 来借用一个变量。
    该借用拥有一个生命周期，此生命周期由它声明的位置决定。
    于是，只要该借用在出借者（lender）被销毁前结束，借用就是有效的。
    然而，借用的作用域则是由使用引用的位置决定的。
*/
mod explicit_annotation;
mod functions;
mod methods;
mod structs;
mod traits;
mod bounds;
mod coercion;
mod r#static;

#[cfg(test)]
mod tests {
    // 下面使用连线来标注各个变量的创建和销毁，从而显示出生命周期。
    // `x` 的生命周期最长，因为它的作用域完全覆盖了 `borrow1` 和`borrow2` 的。
    // `borrow1` 和 `borrow2` 的周期没有关联，
    // 因为它们各不相交。
    #[test]
    fn test_lifetimes() {
        let x = 1024; // x的生命周期开始
        {
            let borrow1 = &x;   // 借用borrow1的生命周期开始
            println!("borrow1: {}", borrow1);
        }   // 借用borrow1的生命周期结束

        {
            let borrow2 = &x;   // 借用borrow2的生命周期开始
            println!("borrow2: {}", borrow2);
        }   // 借用borrow1的生命周期结束
    }   // x的生命周期结束
}