/*
    如果你的函数想要返回trait A的实现类，
    除了用Box包裹trait以外，还可以使用impl A这种写法。
    这样可以使你的函数签名更加简洁
*/

use std::iter;
use std::vec;
/*
    函数定义但未使用，编译会产生的warning
    #[allow(dead_code)] 会关闭对函数combine_vecs_explicit_return_type的warning提示
*/
#[allow(dead_code)]
// 该方法将两个Vec<i32>合并在一起，并且在其上面套了一层迭代器（Iterator）
// 可以看一下返回值类型是有多复杂！
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<vec::IntoIter<i32>, vec::IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

#[allow(dead_code)]
// 下面的函数跟combine_vecs_explicit_return_type相同
// 看看其的简化程度
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 更重要的是，一些Rust的类型是写不出来的。
// 比如：每一个闭包都是一个匿名类型，其根本没有所谓的`闭包名`
// 在出现impl Trait 语法之前，程序员必须在堆上分配空间以便返回一个闭包
// 但是现在完全可以静态地返回闭包了
#[cfg(test)]
mod tests {
    fn make_adder_function(n: i32)
                           -> impl Fn(i32) -> i32 {
        // 创建一个闭包
        let closure = move |x: i32| x + n;
        closure
    }

    #[test]
    fn test_impl_trait() {
        // 创建一个闭包
        let add_one_closure = make_adder_function(1);
        assert_eq!(1025, add_one_closure(1024));
    }

    // 同样可以使用 impl Trait来返回一个Iterator的实现类，该实现类使用map或者filter的闭包
    // 这样使得使用map和filter更加简单。
    // 由于闭包是匿名的，如果函数要返回一个满足Iterator约束的闭包，程序员无法显式地写出来的。
    // 但是使用impl Trait就可以轻松做到：
    fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
        numbers
            .iter()
            .filter(|n| n > &&0)
            // 等价于
//            .filter(|n| **n > 0)
            .map(|n| n * 2)
    }

    // 注：impl Iterator<Item=i32> + 'a 指返回值中涉及到的引用的生命周期要>='a

    /*
        函数double_positives的生命周期标记解读：
            fn double_positives<'a>...
                尖括号内的'a为生命周期声明，表示该函数的生命周期为'a
            ... (numbers: &'a Vec<i32>)...
                表示传入参数是个&Vec，并且其生命周期不会比'a短
            ... -> impl Iterator<Item=i32> + 'a ...
                表示返回类型是个Iterator<Item=i32>实现类，其生命周期不会比'a短
    */

    #[test]
    // 功能测试
    fn test_double_positives_function() {
        let v = vec![-1, 2, -3, 4, -5, 6, -7];
        let iterator = double_positives(&v);
        for i in iterator {
            println!("{}", i)
        }
    }

    #[test]
    // 生命周期测试
    fn test_double_positives_lifetime() {
        // 会报错：error[E0597]: `v` does not live long enough
        // 为什么？因为iterator定义在v前面
        // 即double_positives(&v)返回值的生命周期要大于传入参数的生命周期了。
        // 这样在test_double_positives_lifetime函数结束时，v会在iterator前面进行析构
        // 而iterator中包含了&v，等到iterator析构时，v已经不在了。
        // 这波安全检查是真的厉害！
//        let iterator;
        let v = vec![-1, 2, -3, 4, -5, 6, -7];
        // 为了让iterator的生命周期小于v，那么就在声明v后再声明iterator：
        let iterator;
        iterator = double_positives(&v);

        // 即'a已经结束，看iterator中的&Vec<i32>是否还存在
        for i in iterator {
            println!("{}", i)
        }
    }
}