use std::ops::Not;

use generics::call_trait;
use generics::call_trait::Summary as CallSummary;
use generics::default_trait;
use generics::default_trait::Summary as DefaultSummary;
use generics::normal_trait;
use generics::normal_trait::Summary as NormalSummary;
use util;

// 类似接口 interface
fn main() {
    util::tester(
        vec![
            Box::new(|| println!("hello trait")),
            Box::new(|| {
                let news = normal_trait::NewsArticle {
                    headline: String::from("日本投降了"),
                    location: String::from("芷江"),
                    author: String::from("新华日报"),
                    content: String::from("日本正式投降，递交投降书"),
                };

                let tweet = normal_trait::Tweet {
                    username: String::from("倭国仁裕"),
                    content: String::from("日本投降了"),
                    reply: true,
                    retweet: true,
                };

                println!("{}, {}", news.summarize(), tweet.summarize());
            }),
            // 默认 trait demo
            Box::new(|| {
                let news = default_trait::NewsArticle {
                    headline: String::from("日本投降了"),
                    location: String::from("芷江"),
                    author: String::from("新华日报"),
                    content: String::from("日本正式投降，递交投降书"),
                };

                println!("{}", news.summarize());
            }),
            // 内部调用接口的其他方法
            Box::new(|| {
                let news = call_trait::NewsArticle {
                    headline: String::from("日本投降了"),
                    location: String::from("芷江"),
                    author: String::from("新华日报"),
                    content: String::from("日本正式投降，递交投降书"),
                };

                println!("{}", news.summarize());
            }),
            // trait 作为参数
            Box::new(|| {
                // 类似多态
                fn my_print(news: impl NormalSummary) -> String {
                    news.summarize()
                }

                let news = normal_trait::NewsArticle {
                    headline: String::from("日本投降了"),
                    location: String::from("芷江"),
                    author: String::from("新华日报"),
                    content: String::from("日本正式投降，递交投降书"),
                };
                let tweet = normal_trait::Tweet {
                    username: String::from("倭国仁裕"),
                    content: String::from("日本投降了"),
                    reply: true,
                    retweet: true,
                };
                println!("{}", my_print(news));
                println!("{}", my_print(tweet));
            }),
            //  trait bound
            Box::new(|| {
                trait Display {
                    fn web() -> String;
                    fn mobile() -> String;
                }

                fn notify<T: NormalSummary>(item: T) {
                    println!("Breaking news! {}", item.summarize());
                }

                // 指定多个
                fn check<T: NormalSummary + Display, U: Clone>(item: T) -> i32 {
                    // item.summarize();
                    // item.web();
                    123
                }

                // 上面的变体
                fn check1<T, U>(item: T, obj: U) -> i32
                where
                    T: NormalSummary + Display,
                    U: Clone,
                {
                    123
                }
            }),
            Box::new(|| {
                // fn returns_summarizable(switch: bool) -> impl NormalSummary {
                //     if switch {
                //         normal_trait::Tweet {
                //             username: String::from("horse_ebooks"),
                //             content: String::from(
                //                 "of course, as you probably already know, people",
                //             ),
                //             reply: false,
                //             retweet: false,
                //         }
                //     } else {
                //         normal_trait::NewsArticle {
                //             headline: String::from("Penguins win the Stanley Cup Championship!"),
                //             location: String::from("Pittsburgh, PA, USA"),
                //             author: String::from("Iceburgh"),
                //             content: String::from(
                //                 "The Pittsburgh Penguins once again are the best
                //         hockey team in the NHL.",
                //             ),
                //         }
                //     }
                // }
            }),
            Box::new(|| {
                fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
                    let mut largest = list[0];

                    for &item in list.iter() {
                        if item > largest {
                            largest = item;
                        }
                    }

                    largest
                }
            }),
            // 使用 trait bound 有条件地实现方法
            Box::new(|| {
                use std::fmt::Display;

                struct Pair<T> {
                    x: T,
                    y: T,
                }

                impl<T> Pair<T> {
                    fn new(x: T, y: T) -> Self {
                        Self { x, y }
                    }
                }

                impl<T: Display + PartialOrd> Pair<T> {
                    fn cmp_display(&self) {
                        if self.x >= self.y {
                            println!("The largest member is x = {}", self.x);
                        } else {
                            println!("The largest member is y = {}", self.y);
                        }
                    }
                }

                {
                    let p1 = Pair { x: 1, y: 1 };
                    let p2 = Pair::new(100, 100);
                    p1.cmp_display(); // 因为是数字类型，所以可以比较大小
                }

                {
                    let p1 = Pair {
                        x: "hello",
                        y: "world",
                    };
                    let p2 = Pair::new("hello", "world");
                    p1.cmp_display();
                }

                {
                    struct Point {
                        direction: bool,
                        number: u32,
                    }
                    let p1 = Pair {
                        x: Point {
                            direction: false,
                            number: 100,
                        },
                        y: Point {
                            direction: true,
                            number: 101,
                        },
                    };
                    // 由于Point 没有实现 PartialOrd，下面会报错
                    // p1.cmp_display();
                }

                {
                    struct Point {
                        direction: bool,
                        number: u32,
                    }
                    let p1 = Pair {
                        x: Point {
                            direction: false,
                            number: 100,
                        },
                        y: Point {
                            direction: true,
                            number: 101,
                        },
                    };
                    // 由于Point 没有实现 PartialOrd，下面会报错
                    // p1.cmp_display();
                }
            }),
            // 字符串, 逐个字符比较
            Box::new(|| {
                let x = "bb";
                let y = "bba";

                println!("{}", x < y);
            }),
        ],
        None,
    );
}
