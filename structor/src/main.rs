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
                    name: String::from("hu yan"),
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
        ],
        None,
    );
}
