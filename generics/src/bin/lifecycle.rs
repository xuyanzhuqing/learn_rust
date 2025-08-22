use util;

// 生命周期 'a
// 借用检查器

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}
fn main() {
    util::tester(
        vec![
            Box::new(|| {
                let r;

                {
                    let x = 5;
                    r = &x;
                    println!("r: {}", r); // x 的生命可以走到这里
                }
                // 悬垂引用
                // println!("r: {}", r); // x 的生命走不到这里，解开注释就会报错
            }),
            // 函数中的范型生命周期
            Box::new(|| {
                let hello = String::from("hello");
                let world = "world !";

                println!("{}", longest(&hello, world));
            }),
            // 悬垂引用-作用域
            Box::new(|| {
                // let string1 = String::from("long string is long");
                // let result;
                // {
                //     let string2 = String::from("xyz");
                //     result = longest(string1.as_str(), string2.as_str()); // 报错 string2 命太短了
                // }
                // println!("The longest string is {}", result);
            }),
            // 悬垂引用-返回函数内的变量
            Box::new(|| {
                // fn longest<'a>(x: &str, y: &str) -> &'a str {
                //     let result = String::from("really long string");
                //     result.as_str()
                // }
            }),
            // 结构体中的生命周期
            Box::new(|| {
                // struct Person {
                //     name: String,
                // }

                // let ann = String::from("ann");
                // let p1 = Person { name: ann }; // 这里是引用，意味着 ann 所有权转移

                // println!("{}", ann); //报错
            }),
            // 生命周期标注
            Box::new(|| {
                struct Person<'a> {
                    name: &'a String,
                }

                let ann = String::from("ann");
                let p1 = Person { name: &ann }; // 注意这里是借用，所有权没有转移

                println!("{}", ann); // 不报错
            }),
            Box::new(|| {
                struct Person<'a> {
                    name: &'a str,
                }

                let ann = String::from("a.n..n");
                let first_char = ann.split('.').next().expect("Could not found a '.'");
                let p1 = Person { name: first_char }; // 注意这里是借用，所有权没有转移

                println!("{}", ann); // 不报错
            }),
            // 生命周期省略
            Box::new(|| {
                /*
                第一条规则是每一个是引用的参数都有它自己的生命周期参数
                第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
                第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method)， 那么所有输出生命周期参数被赋予 self 的生命周期
                */
            }),
            // 结合泛型类型参数、trait bounds 和生命周期
            Box::new(|| {
                use std::fmt::Display;

                fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
                where
                    T: Display,
                {
                    println!("Announcement! {}", ann);
                    if x.len() > y.len() { x } else { y }
                }
            }),
            // 静态生命周期- 自面量 生命周期能够存活于整个程序期间
            Box::new(|| {
                let s: &'static str = "hello world";
                let b = s;
                let c = s;
                // 静态自面量始终可用
                println!("{}", s);
            }),
        ],
        None,
    );
}
