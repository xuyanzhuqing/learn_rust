use util;

// match 的常见用法
fn main() {
    util::tester(
        vec![
            // match 的基础用法
            Box::new(|| {
                enum Coin {
                    Penny,
                    Nickel,
                    Dime,
                    Quarter,
                }

                fn value_in_cents(coin: Coin) -> u8 {
                    match coin {
                        Coin::Penny => 1,
                        Coin::Nickel => 5,
                        Coin::Dime => 10,
                        Coin::Quarter => {
                            println!("{}", "quarter is one fourth");
                            25
                        }
                    }
                }
                println!("{}", value_in_cents(Coin::Quarter));
            }),
            // 绑定值的模式
            Box::new(|| {
                #[derive(Debug)]
                enum UsState {
                    Alabama, // 阿拉巴马州
                    Alaska,  // 阿拉斯加州
                }

                enum Coin {
                    Penny,
                    Nickel,
                    Dime,
                    Quarter(UsState),
                }

                fn value_in_cents(coin: Coin) -> u8 {
                    match coin {
                        Coin::Penny => 1,
                        Coin::Nickel => 5,
                        Coin::Dime => 10,
                        Coin::Quarter(state) => {
                            dbg!(state);
                            25
                        }
                    }
                }
                println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
                println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
            }),
            // 匹配 Option<T>
            Box::new(|| {
                fn plus_one(n: Option<i32>) -> Option<i32> {
                    match n {
                        Some(n) => Some(n + 1),
                        None => None,
                    }
                }

                let five = Some(5);
                let six = plus_one(five);
                let none = plus_one(None);
                println!("{:?}, {:?}, {:?}", five, six, none);
            }),
            // 通配符和占位符
            Box::new(|| {
                fn move_left() {
                    println!("move left")
                }
                fn move_right() {
                    println!("move right")
                }
                fn move_up() {
                    println!("move up")
                }
                fn move_down() {
                    println!("move down")
                }

                fn do_nothing() {
                    println!("nothing to do");
                }

                fn move_step(c: char) {
                    match c {
                        'a' => move_left(),
                        's' => move_down(),
                        'd' => move_right(),
                        'w' => move_up(),
                        _other => do_nothing(), // 通配符
                                                // _ => (), // 占位符
                    }
                }

                move_step('a');
                move_step('h')
            }),
            // if let
            Box::new(|| {
                let some_u8_value = Some(0u8);
                {
                    match some_u8_value {
                        Some(3) => println!("three"),
                        _ => (),
                    }
                }

                // if () {}, 垃圾还不如上面看着简洁
                {
                    if let Some(3) = some_u8_value {
                        println!("three");
                    }
                }
            }),
        ],
        None,
    );
}
