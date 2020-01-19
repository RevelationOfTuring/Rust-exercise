/*
    DRY(不写重复代码)

    通过提取函数或测试集的公共部分，宏可以让你写出DRY的代码。
    （DRY 是 Don't Repeat Yourself 的缩写，意思为 “不要写重复代码”）

    这里给出一个例子：
        对 Vec<T> 实现 并测试了关于 +=、*= 和 -= 等运算符。
*/
#[cfg(test)]
mod tests {
    use std::ops::{Add, Mul, Sub};
    // 判断长度是否相等的宏
    macro_rules! assert_equal_len {
    // 注：`tt`（token tree，标记树）指示符表示运算符和标记。
    // 即，直接写 +=、*=、-= 等要自定义的运算符
        ($a:ident,$b:ident,$func:ident,$op:tt)=>(
            assert!($a.len() == $b.len(),
                    "{:?}: dimension mismatch: {:?} {:?} {:?}",
                    stringify!($func),
                    ($a.len(),),
                    stringify!($op),
                    ($b.len(),));
        )
    }
    #[test]
    fn test_assert_equal_len() {
        let a = "abc";
        let b = "def";
        assert_equal_len!(a,b,hi,+=);
        // 由于a和b的长度相等，所以通过assert！

        let a = "abc";
        let b = "defg";
        assert_equal_len!(a,b,hi,+=)
        // 由于a和b的长度不相等，所以无法通过assert！
        // 运行报错："hi": dimension mismatch: (3,) "+=" (4,)
    }

    macro_rules! op {
    // $bound是trait名，$method是该trait中的方法名
        ($func:ident,$bound:ident,$op:tt,$method:ident)=>(  // => 后面跟()，表示该宏返回的是一个值
            fn $func<T: $bound<T,Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>){
            // 调用上面自定义的宏
                assert_equal_len!(xs,ys,$func,$op);

                for (x,y) in xs.iter_mut().zip(ys.iter()){
                    *x = $bound::$method(*x,*y);
                }

            }
        )
    }

    // 利用op!实现`add_michael`、`mul_michael` 和 `sub_michael` 等函数
    op!(add_michael,Add,+=,add);    // Add是一个trait，add是该trait中的方法名
    op!(mul_michael,Mul,*=,mul);
    op!(sub_michael,Sub,-=,sub);

    #[test]
    fn test_dont_repeat_yourself() {
        // 自定义宏,在当前测试函数内创建对应的测试函数
        macro_rules! test_michael {
            ($func:ident,$x:expr,$y:expr,$z:expr) => {  // => 后面跟{}，表示该宏返回的是一个代码块
                #[test]
                fn $func(){
                    for size in 0usize..=10{
                        let mut x: Vec<_> = std::iter::repeat($x).take(size).collect();
                        let y: Vec<_> = std::iter::repeat($y).take(size).collect();
                        let z: Vec<_> = std::iter::repeat($z).take(size).collect();

                        /*
                            注：iter::repeat(4) 生成一个迭代器，会一直迭代输出4，没有尽头
                               iter::repeat(4).take(4) 生成一个迭代器，只能连续迭代输出4个4，到第5次迭代时，输出None
                        */
                        // 调用mod tests中的同名方法
                        self::$func(&mut x, &y);

                        assert_eq!(x,z)
                    }
                }
            }
        }

        test_michael!(add_michael,1,2,3);
        test_michael!(mul_michael,4,5,20);
        test_michael!(sub_michael,3,2,1);
    }
}