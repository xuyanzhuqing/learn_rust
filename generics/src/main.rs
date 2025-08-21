use util;
// 范型
#[allow(unused)]
fn main() {
    util::tester(
        vec![
            Box::new(|| {
                let nums = vec![4, 5, 3, 7, 3, 1];
                let chars = vec!['a', 'd', 'g', 'e', 'f'];
                // fn largest<T>(list: &[T]) -> T {
                //     let mut largest = list[0];

                //     for &item in list.iter() {
                //         if item > largest { // 报错，不是所有类型都支持算术运算符比较
                //             // 类型
                //             largest = item;
                //         }
                //     }

                //     largest
                // }
            }),
            // 结构体中的范型
            Box::new(|| {
                {
                    #[derive(Debug)]
                    struct Point<T> {
                        x: T,
                        y: T,
                    }

                    // 自动推导类型
                    let p1 = Point { x: 0, y: 0 };

                    let p2: Point<&str> = Point {
                        x: "东经108度",
                        y: "北纬37度",
                    };
                    dbg!(p1);
                    dbg!(p2);
                }

                {
                    #[derive(Debug)]
                    struct Point<T, U> {
                        x: T,
                        y: U,
                    }

                    let p1 = Point { x: 108, y: "39N" };

                    let p2: Point<i32, &str> = Point {
                        x: 108,
                        y: "hello world",
                    };
                    // 方法体的范型
                    impl<T, U> Point<T, U> {
                        fn x(&self) -> &T {
                            &self.x
                        }
                    }

                    println!("x = {}", p1.x());
                }
            }),
            // 枚举中的范型
            Box::new(|| {
                #[derive(Debug)]
                enum Version<T> {
                    Amd(T),
                    Arm(T),
                }

                let amd: Version<&str> = Version::Amd("1.1.1");
                dbg!(amd);

                enum Option<T> {
                    Some(T),
                    None,
                }
            }),
        ],
        None,
    );
}
