// 结构体
use util;

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    married: bool,
    // 定义任意多个孩子
    children: Option<Vec<Box<User>>>,
    gender: bool,
}
fn main() {
    util::tester(
        vec![
            // 结构体定义，复用，生命周期？
            Box::new(|| {
                let name = "ann";
                let age = 28;
                let married = true;
                let gender = false;

                let guo: User = User {
                    name: String::from("果果"),
                    age: 0,
                    married: false,
                    children: None,
                    gender: true,
                };

                // 简写
                let ann: User = User {
                    name: String::from(name),
                    age,
                    married,
                    children: Some(vec![Box::new(guo)]),
                    gender,
                };

                // 复用-导致ann,guo 内的属性被借用了，不能再使用
                let girl: User = User {
                    name: String::from("hu yan"), // 注意结构，覆盖在前
                    ..ann
                };

                println!("name = {}", name);
                println!("age = {}", age);
                println!("married = {}", married);
                println!("gender = {}", gender);

                println!(
                    "{}, {}, {}, {}",
                    girl.age, girl.name, girl.gender, girl.married
                );

                // 复用-导致ann,guo 内的属性被借用了，不能再使用
                // 这语法真他么恶心
                // dbg!(&ann);
                // dbg!(&guo);

                dbg!(&girl.children);
                dbg!(&girl);
            }),
            // 更多类型
            Box::new(|| {
                // 元组
                struct Color(u32, u32, u32);
                struct Point(u32, u32, u32);

                let black = Color(0, 0, 0);
                let origin = Point(0, 0, 0);

                // 没有字段的结构体-类单元结构体
                struct AnyWay;
                let anyway: AnyWay = AnyWay; // () 等于空元组
            }),
            // 定义方法
            Box::new(|| {
                #[derive(Debug)]
                struct Rectangle {
                    width: u32,
                    height: u32,
                }

                impl Rectangle {
                    fn area(&self) -> u32 {
                        self.width * self.height
                    }
                }

                let rect1 = Rectangle {
                    width: 30,
                    height: 40,
                };

                println!("the rectangle area is {}", rect1.area());

                dbg!(&rect1);
            }),
            // 可以定义与属性相同的方法名
            Box::new(|| {
                // 定义方法
                #[derive(Debug)]
                struct Rectangle {
                    width: u32,
                    height: u32,
                }

                impl Rectangle {
                    fn width(&self) -> bool {
                        // 隐式借用
                        // self.width > 0
                        // 显式借用
                        (&self).width > 0
                    }
                }

                let rect1 = Rectangle {
                    width: 100,
                    height: 100,
                };

                print!("{}", rect1.width());
            }),
            // 更多参数的方法
            Box::new(|| {
                #[derive(Debug)]
                struct Rectangle {
                    width: u32,
                    height: u32,
                }

                impl Rectangle {
                    fn area(&self) -> u32 {
                        self.width * self.height
                    }

                    fn can_hold(&self, other: &Rectangle) -> bool {
                        self.width > other.width && self.height > other.height
                    }
                }

                let rect1 = Rectangle {
                    width: 100,
                    height: 200,
                };

                let rect2 = Rectangle {
                    width: 50,
                    height: 50,
                };

                let rect3 = Rectangle {
                    width: 120,
                    height: 100,
                };

                println!("{}", rect1.can_hold(&rect2)); // true
                println!("{}", rect1.can_hold(&rect3)); // false
            }),
            // 关联函数
            Box::new(|| {
                #[derive(Debug)]
                struct Rectangle {
                    width: u32,
                    height: u32,
                }
                impl Rectangle {
                    fn square(size: u32) -> Rectangle {
                        Rectangle {
                            width: size,
                            height: size,
                        }
                    }
                }
                let square = Rectangle::square(100);
                dbg!(square);
            }),
            // 多impl
            Box::new(|| {
                #[derive(Debug)]
                struct Rectangle {
                    width: u32,
                    height: u32,
                }

                impl Rectangle {
                    fn area(&self) -> u32 {
                        self.width * self.height
                    }
                }

                impl Rectangle {
                    fn can_hold(&self, other: &Rectangle) -> bool {
                        self.width > other.width && self.height > other.height
                    }
                }
            }),
        ],
        None,
    );
}
