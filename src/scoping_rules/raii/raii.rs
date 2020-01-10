/*
    可以使用 valgrind 对内存错误进行仔细检查
    注：valgrind是一个内存管理和线程管理的检测工具

    安装：
    $ brew install valgrind
    检查内存：
    $ rustc raii.rs && valgrind ./raii

    输出如下：
    ==6264== Memcheck, a memory error detector
    ==6264== Copyright (C) 2002-2017, and GNU GPL'd, by Julian Seward et al.
    ==6264== Using Valgrind-3.15.0 and LibVEX; rerun with -h for copyright info
    ==6264== Command: ./raii
    ==6264==
    --6264-- run: /usr/bin/dsymutil "./raii"
    ==6264==
    ==6264== HEAP SUMMARY:
    ==6264==     in use at exit: 18,682 bytes in 165 blocks
    ==6264==   total heap usage: 1,196 allocs, 1,031 frees, 31,403 bytes allocated
    ==6264==
    ==6264== LEAK SUMMARY:
    ==6264==    definitely lost: 3,584 bytes in 56 blocks
    ==6264==    indirectly lost: 0 bytes in 0 blocks
    ==6264==      possibly lost: 72 bytes in 3 blocks
    ==6264==    still reachable: 321 bytes in 7 blocks
    ==6264==         suppressed: 14,705 bytes in 99 blocks
    ==6264== Rerun with --leak-check=full to see details of leaked memory
    ==6264==
    ==6264== For lists of detected and suppressed errors, rerun with: -s
    ==6264== ERROR SUMMARY: 0 errors from 0 contexts (suppressed: 1 from 1)
*/
fn create_box() {
    // 在堆上分配一个整型数据
    let _box1 = Box::new(3i32);

    // `_box1` 在这里被销毁，内存得到释放
}

fn main() {
    // 在堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域：
    {
        // 在堆上分配一个整型数据
        let _box3 = Box::new(4i32);

        // `_box3` 在这里被销毁，内存得到释放
    }

    // 创建一大堆 box（只是因为好玩）。
    // 完全不需要手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` 在这里被销毁，内存得到释放
}