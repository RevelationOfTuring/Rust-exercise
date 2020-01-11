/*
    特性trait

    trait 是对未知类型 Self 定义的方法集。
    该类型也可以访问同一个 trait 中定义的其他方法。

    对任何数据类型都可以实现 trait。
*/
mod derive;
mod returning_traits_with_dyn;
mod operator_overloading;
mod drop;
mod iterators;
mod impl_trait;
mod clone;
mod super_traits;

#[cfg(test)]
mod tests {
    struct Sheep {
        naked: bool,
        name: &'static str,
    }

    // 定义trait
    trait Animal {
        // 静态方法签名；`Self` 表示实现者类型（implementor type）
        fn new(name: &'static str) -> Self;

        // 实例方法签名；这些方法将返回一个&str
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // trait可以提供默认的方法定义
        fn talk(&self) {
            // trait中的方法可以调用同一个trait中的其他方法
            println!("{} talks like {}", self.name(), self.noise());
        }
    }

    // 定义Sheep类的"类方法"
    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // 实现者可以使用它的 trait 方法
                println!("{} is already naked...", self.name());
            } else {
                // 不用trait中的name方法，而是直接用name成员
                println!("{} gets a haircut!", self.name);
                self.naked = true;
            }
        }
    }

    // struct Sheep实现trait
    impl Animal for Sheep {
        // `Self` 是实现者类型：`Sheep`
        fn new(name: &'static str) -> Self {
            Sheep {
                naked: false,
                name,
            }
        }

        fn name(&self) -> &'static str { self.name }

        fn noise(&self) -> &'static str {
            // 实现trait的方法中也可以调用struct的独自的类方法
            if self.is_naked() {
                "baaaaaaah?"
            } else {
                "baaaaaaah!"
            }
        }

        // 如果不想使用trait已默认定义好的talk方法，可以在这里重写
        fn talk(&self) {
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    #[test]
    fn test_traits() {
        // 这种情况需要类型标注
        // 因为Animal::new方法是一个抽象方法，直接调用并不知道返回值是那种实现类。
        // 需要手动标明类型
        let mut sheep: Sheep = Animal::new("Michael.W");
        sheep.talk();
        sheep.shear();
        sheep.talk();

        // 直接用Sheep类来调用静态方法，编译器是可以识别返回类型的
        let mut sheep = Sheep::new("Michael.W");
        sheep.talk();
        sheep.shear();
        sheep.talk();
    }
}