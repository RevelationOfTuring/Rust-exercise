// for in 结构能以几种方式与 Iterator 互动。
// 如果没有特别指定，for 循环会对给出的集合应用 into_iter 函数,，把它转换成 一个迭代器。
// 这并不是把集合变成迭代器的唯一方法，其他的方法有 iter 和 iter_mut 函数。
#[cfg(test)]
mod tests {
    #[test]
    fn test_for_and_iterator() {
        let names = vec!["Bob", "Frank", "Ferris"];
        // 三个函数会以不同的方式返回集合中的数据

        // 函数1 iter()
        // iter - 在每次迭代中'借用'集合中的一个元素。这样集合本身不会被改变，循环之后仍可以使用。
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        // 函数2 into_iter()
        // into_iter - 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 move 了。

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        // 由于 into_iter()将集合的所有权move掉了，需要重新创建一个集合
        let mut names = vec!["Bob", "Frank", "Ferris"];

        // 函数3 iter_mut()
        // iter_mut - 可变地（mutably）借用集合中的每个元素，从而允许集合被就地修改。
        for name in names.iter_mut() {
            match name {
                e @ &mut "Ferris" => {
                    println!("There is a rustacean among us!");
                    *e = "michael.w";
                }
                e @ _ => {
                    println!("Hello {}", e);
                    *e = "Iversion";
                }
            }
        }

        println!("{:?}", names);
        // ["Iversion", "Iversion", "michael.w"]

        // 可见names中的数据已经被修改
    }
}