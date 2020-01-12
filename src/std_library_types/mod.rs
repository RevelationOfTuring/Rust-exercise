/*
    标准库提供了很多自定义类型，在原生类型基础上进行了大量扩充:

        - 可增长的 String（字符串）: "rust"
        - 可增长的向量（vector）: [1, 2, 3]
        - 选项类型（optional types）: Option<i32>
        - 错误处理类型（error handling types）: Result<i32, i32>
        - 堆分配的指针（heap allocated pointers）: Box<i32>
*/

mod box_stack_heap;
mod vector;
mod strings;
mod string_literals_and_escapes;
mod option;
mod result;
mod result_with_question_mark;
mod panic;
mod hashmap;
mod alternate_or_custom_key_types;
mod hashset;
