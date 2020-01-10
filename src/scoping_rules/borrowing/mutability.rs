/*
    可变性

    可变数据可以使用 `&mut T` 。

    这叫做可变引用（mutable reference），它使借用者可以读/写数据。
    相反，&T 通过不可变引用（immutable reference）来借用数据，借用者可以读数据而不能更改数据。
*/

mod tests {
    // `&'static str` 是一个对分配在`只读内存区`的字符串的引用
    #[derive(Copy, Clone)]
    struct Book {
        author: &'static str,
        title: &'static str,
        year: u32,
    }

    // 此函数接受一个对 Book 类型的引用
    fn borrow_book(book: &Book) {
        println!("I immutably borrowed {} - {} - {} edition", book.author, book.title, book.year)
    }

    // 此函数接受一个对可变的 Book 类型的引用，它把年份 `year` 改为 2020 年
    fn new_edition(book: &mut Book) {
        book.year = 2020;
        println!("I mutably borrowed {} - {} - {} edition", book.author, book.title, book.year)
    }

    #[test]
    fn test_mutability() {
        // 创建一个名为 `immutabook` 的不可变的 Book 实例
        let immutabook = Book {
            author: "Michael.W",
            title: "Bad Girl",
            year: 2000,
        };

        // 创建一个 `immutabook` 的可变拷贝
        // Book需要实现Copy，Clone trait
        let mut mutabook = immutabook;

        // 不可变地借用一个不可变对象
        borrow_book(&immutabook);

        // 不可变地借用一个可变对象
        borrow_book(&mutabook);

        // 可变地借用一个可变对象
        new_edition(&mut mutabook);

        // 报错！不能可变地借用一个不可变对象
//        new_edition(&mut immutabook);
    }
}