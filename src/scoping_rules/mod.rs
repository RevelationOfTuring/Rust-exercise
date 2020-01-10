/*
    作用域在所有权（ownership）、借用（borrow）和生命周期（lifetime）中起着重要作用。
    也就是说，作用域告诉编译器什么时候借用是合法的、什么时候资源可以释放、以及变量何时被创建或销毁。
*/
mod raii;
mod ownership_and_moves;
mod borrowing;