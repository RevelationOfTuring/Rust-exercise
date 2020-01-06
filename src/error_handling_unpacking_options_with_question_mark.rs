/*
    可以通过 match 语句取出Option中的变量，但是通过操作符 ？可以更加简单。
    如果 x 是一个Option，那么 x? 将返回其中的值（前提：x是Some）,
    如果 x 是None，那么会直接结束当前函数且返回None。
*/

#[cfg(test)]
mod tests {
    fn next_birthday(current_age: Option<u8>) -> Option<String> {
        // 如果 current_age 是 None， 则返回 None；
        // 如果 current_age 是 Some， 则将u8转成String返回

        // 使用?操作符
        let next_age = current_age?;
        // 如果current_age为None,直接结束函数并返回。

        // 将u8转成String
        Some(format!("{}", next_age))
    }

    #[test]
    fn test_error_handling_unpacking_options_with_question_mark() {
        println!("{:?}", next_birthday(Some(18)));
        println!("{:?}", next_birthday(None));
    }

    // 可以链式使用多个?操作符来增强代码的可读性

    struct Person {
        job: Option<Job>,
    }

    // 要使用 ? 操作符的Option中的载体必须实现Copy trait
    #[derive(Copy, Clone)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    #[derive(Copy, Clone)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }

    impl Person {
        // 获取一个人的工作手机号的area_code，如果这个area_code存在的话
        fn work_phone_area_code(&self) -> Option<u8> {
            // 如果不使用 ? 操作符，那么将使用多层嵌套的 match
            self.job?.phone_number?.area_code
        }
    }

    #[test]
    fn test_error_handling_unpacking_options_with_question_mark_readable() {
        let person = Person {
            job: None
        };
        // 返回None,由于self.job?直接返回None
        println!("{:?}", person.work_phone_area_code());

        let person = Person {
            job: Some(Job {
                phone_number: None,
            }),
        };
        // 返回None,由于self.job?.phone_number?直接返回None
        println!("{:?}", person.work_phone_area_code());

        let person = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: None,
                    number: 1024,
                })
            }),
        };
        // 返回None,由于self.job?.phone_number?.area_code直接返回None
        println!("{:?}", person.work_phone_area_code());

        let person = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(1),
                    number: 1024,
                })
            }),
        };
        // 返回Some(1)
        println!("{:?}", person.work_phone_area_code());
    }
}