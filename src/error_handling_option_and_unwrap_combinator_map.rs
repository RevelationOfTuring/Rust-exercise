/*
    match 是处理 Option 的一个可用的方法，但大量使用它会很繁琐。
    特别是当操作只对`一种`输入是有效的时。
    这时，可以使用组合算子（combinator），以模块化的风格来管理控制流。

    Option 有一个内置方法 map()，这个组合算子可用于 Some -> Some 和 None -> None 这样的简单映射。
    即，输入是Option，输出也是Option的情况。
    多个不同的 map() 调用可以串起来，这样更加灵活。
*/

// 在下面例子中，process() 轻松取代了前面的所有函数，且更加紧凑。

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    struct Peeled(Food);

    struct Chopped(Food);

    #[derive(Debug)]
    struct Cooked(Food);

    // 下面是用match来写逻辑：

    // 削皮。如果没有食物，就返回 `None`。否则返回削好皮的食物。
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(i) => Some(Peeled(i)),
            None => None
        }
    }

    // 切食物。如果没有食物，就返回 `None`。否则返回切好的食物。
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            // 利用match的模式匹配将里层的Food掏出来
            Some(Peeled(i)) => Some(Chopped(i)),
            None => None
        }
    }

    // 烹饪食物。煮的原料必须是进过切割的食物。
    // 这里，我们使用 `map()` 来替代 `match` 以处理各种情况。
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        // 利用Chopped(i)将food模式匹配出来，并生成返回值Cooked(i)
        chopped.map(|Chopped(i)| Cooked(i))
    }

    // 这个函数会完成削皮切块烹饪一条龙。
    // 把 `map()` 串起来，以简化代码。
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|i| Peeled(i))             // Option中现在是Peeled()
            .map(|Peeled(i)| Chopped(i))    // Option中现在是Chopped()
            .map(|Chopped(i)| Cooked(i))    // Option中现在是Cooked()
    }

    // 在尝试吃食物之前确认食物是否存在是非常重要的！
    fn eat(cooked_food: Option<Cooked>) {
        match cooked_food {
            Some(i) => println!("good cooked food {:?}", i),
            None => println!("there is no cooked food")
        }
    }

    #[test]
    fn test_error_handling_option_and_unwrap_combinator_map() {
        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = Some(Food::Potato);

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = cook(chop(peel(carrot)));
        println!("{:?} {:?}", cooked_apple, cooked_carrot);

        // 烹饪一条龙
        let cooked_potato = process(potato);
        println!("{:?}", cooked_potato);

        // 开吃
        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
}