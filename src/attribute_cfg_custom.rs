/*
    有部分条件,如 target_os 是由 rustc 隐式地提供的，
    所以编译的时候不需要传递参数给rustc 。

    但是自定义条件（编译条件）必须使用 --cfg 标记来传给 rustc 。
*/

#[cfg(some_condition)]
fn conditional_function() {
    println!("meets condition!")
}

fn main() {
    // 调用自定义编译条件函数
    conditional_function();
}


/*
    下面进行编译：
    *******************************
    如果不使用自定义的cfg标记：
    $ rustc attribute_cfg_custom.rs

    直接报错:cannot find function `conditional_function` in this scope
    *******************************
    使用自定义cfg标记：
    $ rustc --cfg some_condition attribute_cfg_custom.rs
    $ ./attribute_cfg_custom

    输出：meets condition!
*/