/*
    要把文件crates_library.rs创建的库链接到一个crate，必须使用`extern crate`声明。
    这不仅会链接库，还会用一个与库名相同的模块来存放库里面的所有项。
    于模块的可见性规则也适用于库。
*/

// 链接到 `crates_library` 库，导入其中的项。(需要事先生成好rlib类型的库文件)
extern crate crates_library;

fn main(){
    // 调用crates_library库中的函数public_function
    crates_library::public_function();
    // 调用crates_library库中的函数indirect_access
    crates_library::indirect_access();

}

// 打开终端，执行编译(需要指定编译好的库文件路径)：
// $ rustc crate_extern_crate.rs --extern crates_library=libcrates_library.rlib
// 执行编译好的可执行文件：
// ./crate_extern_crate

// 打印输出：
//      called `public_function()`
//      called `indirect_access()`, that
//      > called `private_function()`


