/*
    map() 以链式调用的方式来简化 match 语句。
    然而，如果以返回类型是 Option<T> 的函数作为 map() 的参数，
    会导致出现嵌套形式 Option<Option<T>>。

    这样多层串联 调用就会变得混乱。
    所以有必要引入 and_then()，在某些语言中它叫做 flatmap 。

    and_then() 使用被 Option 包裹的值来调用其输入函数并返回结果。
    如果 Option 是 None，那么它返回 None。
*/

//  在下面例子中，cookable_v2() 会产生一个 Option<Food>。
//  如果在这里使用 map() 而不是 and_then() 将会得到 Option<Option<Food>>，这对 eat() 来说是一个无效类型。

#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }

    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    // 我们没有制作寿司所需的原材料（ingredient）（有其他的原材料）。
    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food)
        }
    }

    // 我们拥有全部食物的食谱，除了法国蓝带猪排（Cordon Bleu）的。
    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food)
        }
    }

    // 要做一份好菜，我们需要原材料和食谱。
    // 我们可以借助一系列 `match` 来表达这个逻辑：
    fn cook_v1(food: Food) -> Option<Food> {
        match have_ingredients(food) {
            // food为Sushi
            None => None,
            Some(i) => match have_recipe(i) {
                // food为CordonBleu
                None => None,
                // 能走到这个分支的时候，food一定是Steak
                Some(i) => Some(i)
            }
        }
    }

    // 也可以使用 `and_then()` 把上面的逻辑改写得更紧凑：
    fn cook_v2(food: Food) -> Option<Food> {
        have_ingredients(food).and_then(have_recipe)
    }
    // 注：适合 `and_then()` 的，是函数参数不是Option，但是函数返回值是Option。

    // 烹饪，食用一条龙
    fn eat(food: Food, day: Day) {
        match cook_v2(food) {
            Some(i) => println!("eat {:?} on {:?}", i, day),
            _ => println!("nothing to eat")
        }
    }

    #[test]
    fn test_error_handling_option_and_unwrap_combinator_and_then() {
        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

        eat(cordon_bleu, Day::Monday);
        eat(sushi, Day::Tuesday);
        eat(steak, Day::Wednesday);
    }
}