mod variable_bindings;
mod types;
mod conversion;
mod constants;
mod enum_linked_list;
mod enums;
mod tuples;
mod structures;
mod array_and_slice;
mod display_vec;
mod display_formatting;
mod display;
mod tuples_activity;
mod labels;
mod while_control;
mod for_and_iterator;
mod match_tuple;
mod match_enum;
mod match_pointer_and_reference;
mod destructure_struct;
mod match_guard;
mod match_binding;
mod match_if_let;
mod match_while_let;
mod function;
mod method;
mod function_closures;
mod function_closures_capturing;
mod function_closures_as_input_parameters;
mod function_closures_type_anonymity;
mod function_closures_input_functions;
mod function_closures_as_output_parameters;
mod function_closures_examples_in_std;
mod function_higher_order_functions;
mod function_diverging_functions;
mod module_visibility;
mod module_struct_visibility;
mod module_use_declaration;
mod module_super_and_self;
mod module_file_hierarchy;
mod attribute_dead_code;
mod attribute_cfg;
mod generics;
mod generics_function;
mod generics_implementation;
mod generics_trait;
mod generics_bounds;
mod generics_empty_bounds;
mod generics_multiple_bounds;
mod generics_where_clauses;
mod generics_new_type_idiom;
mod generics_associated_items_problem;
mod generics_associated_types;
mod error_handling_panic;
mod error_handling_option_and_unwrap;
mod error_handling_unpacking_options_with_question_mark;
mod error_handling_option_and_unwrap_combinator_map;
mod error_handling_option_and_unwrap_combinator_and_then;
mod error_handling_result;
mod error_handling_result_map;
mod error_handling_result_aliases;
mod error_handling_result_early_returns;
mod error_handling_result_question_mark;
mod error_handling_multiple_error_types;
mod error_handling_multiple_error_types_pull_result_out_of_option;
mod error_handling_multiple_error_types_define_an_error_type;
mod error_handling_multiple_error_types_boxing_errors;
mod error_handling_multiple_error_types_other_uses_of_question_mark;
mod error_handling_multiple_error_types_wrapping_errors;
mod error_handling_multiple_error_types_iterating_over_results;
mod static_and_dynamic_dispatch;
mod trait_object;
mod object_safe;
mod impl_trait;
mod scoping_rules;
mod lifetimes;
mod traits;
mod std_library_types;
mod path;
mod file_io;
mod macro_rules;
mod thread;

fn main() {
    // 用于多线程的demo
    demo_for_thread();
}

// 多线程的demo
fn demo_for_thread() {
    const THREAD_NUM: i32 = 10;
    // 用于存放各个线程handler的Vec
    let mut thread_handlers = vec![];

    for i in 0..THREAD_NUM {
        let handler = std::thread::spawn(
            move || println!("THREAD {} is running", i)
        );
        //handler存入Vec
        thread_handlers.push(handler);
    }

    for handler in thread_handlers {
        // 等待线程结束。返回一个结果
        let _ = handler.join();
    }
}