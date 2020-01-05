/*
    泛型（generic）是关于泛化类型和函数功能，以扩大其适用范围。
    泛型极大地减少了代码的重复，

    泛型最简单和常用的用法是用于类型参数。
   注：定义泛型类型或泛型函数之类的东西时，我们会用 <A> 或者 <T> 这类标记作为类型的代号，就像函数的形参一样。
      在使用时，为把 <A>、<T> 具体化，我们会把类型说明像实参一样使用，像是 <i32> 这样。
      这两种把（泛型的或具体的）类型当作参数的用法就是类型参数。
    
*/


// 例如定义一个泛型函数generics_function，它可接受类型为T的任何参数arg：
fn generics_function<T>(arg: T) {
    println!("generics_function");
}

// 因为使用了泛型类型参数 <T>，所以这里的 (arg: T) 中的 T 就是泛型类型。
// 即使 T 在之前被定义为 struct，这里的 T 仍然代表泛型。

#[cfg(test)]
mod tests {
    // 泛型语法的使用的例子
// 一个具体类型 `S`
    #[derive(Debug)]
    struct S;

    // 在定义类型 `Single` 时，第一次使用类型 `S` 之前没有写 `<S>`。
// 因此，`Single` 是个具体类型，`S` 取上面的定义。
    #[derive(Debug)]
    struct Single(S);
//            ^ 这里是 `Single` 对类型 `S` 的第一次使用。

    // 此处 `<T>` 在第一次使用 `T` 前出现，所以 `SingleGen` 是一个泛型类型。
// 因为 `T` 是泛型的，所以它可以是任何类型，包括在上面定义的具体类型 `S`。
    #[derive(Debug)]
    struct SingleGen<T>(T);

    #[test]
    fn test_generics() {
        // `Single` 是具体类型，并且显式地使用类型 `S`。
        let s = Single(S);
        println!("{:?} {:?}", s, s.0);

        // 创建一个 `SingleGen<char>` 类型的变量 `char_object`，并令其值为 `SingleGen('a')`
        // 这里的 `SingleGen` 的类型参数是显式指定的。
        let char_object: SingleGen<char> = SingleGen('a');

        // `SingleGen` 的类型参数也可以`隐式地`指定。
        let s_object = SingleGen(S);
        let i = SingleGen(1024);
        let str_object = SingleGen("michael.w");

        println!("{:?} {:?} {:?} {:?}", char_object, s_object, i, str_object);
    }
}