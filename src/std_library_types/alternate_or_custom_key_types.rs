/*
    任何实现了 Eq 和 Hash trait 的类型都可以充当 HashMap 的键。这包括:
        - bool （当然这个用处不大，因为只有两个可能的键）
        - int，unit，以及其他整数类型
        - String 和 &str
        （友情提示：如果使用 String 作为键来创建 HashMap，则可以将 &str 作为散列表的 .get() 方法的参数，以获取值）

        注：f32 和 f64 没有实现 Hash，这是由于若使用浮点数作为散列表的键，浮点精度误差会很容易导致错误。

    对于所有的集合类（collection class），如果它们包含的类型都分别实现了 Eq 和 Hash，
    那么这些集合类也就实现了 Eq 和 Hash。例如，若 T 实现了 Hash，则 Vec<T> 也实现了 Hash。

    对`自定义类型`可以轻松地实现 Eq 和 Hash，只需加上一行代码：#[derive(PartialEq, Eq, Hash)]。
*/

// 尝试做一个非常简单的用户登录系统

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    // 定义账户类
    // 因为做hashmap的key，需要实现PartialEq, Eq, Hash trait
    #[derive(Eq, PartialEq, Hash)]
    struct Account<'a> {
        username: &'a str,
        password: &'a str,
    }

    // 定义账户信息类
    struct Info<'a> {
        name: &'a str,
        email: &'a str,
    }

    // 出于便利，给HashMap起别名
    type AccountBook<'a> = std::collections::HashMap<Account<'a>, Info<'a>>;


    // 登录功能
    fn log_on<'a>(username: &'a str, password: &'a str, account_book: &AccountBook<'a>) {
        println!("
            Username    : {}
            Password    : {}
            Attempting logon...", username, password);

        let account = Account { username, password };

        match account_book.get(&account) {
            Some(info) => println!("
            Login successfully!
            Info:
                Name    : {}
                Email   : {}", info.name, info.email
            ),
            _ => println!("
            Login failed")
        }
    }

    #[test]
    fn test_alternate_or_custom_key_types() {
        // 利用别名创建HashMap
        let mut account_book = HashMap::new();

        let account = Account { username: "RustKing", password: "12345678" };
        let info = Info { name: "Michael.W", email: "1234567@qq.com" };

        // 登录失败
        log_on("RustKing", "12345678", &account_book);

        // 添加KV对
        account_book.insert(account, info);

        // 成功登录
        log_on("RustKing", "12345678", &account_book);
    }
}