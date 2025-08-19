use util;

// 枚举
fn main() {
    util::tester(
        vec![
            // 枚举的基本应用
            Box::new(|| {
                #[derive(Debug)]
                enum IPAddressKind {
                    V4,
                    V6,
                }

                let four = IPAddressKind::V4;
                let six = IPAddressKind::V6;

                println!("{:?}", four);
                println!("{:?}", six);
            }),
            // 结合方法
            Box::new(|| {
                enum IPAddressKind {
                    V4,
                    V6,
                }
                fn rout(kind: IPAddressKind) {}
            }),
            // 结合结构体
            Box::new(|| {
                #[derive(Debug)]
                enum IPAddressKind {
                    V4,
                    V6,
                }

                #[derive(Debug)]
                struct IPAddress {
                    kind: IPAddressKind,
                    address: String,
                }

                let localhost = IPAddress {
                    kind: IPAddressKind::V4,
                    address: String::from("127.0.0.1"),
                };

                let loop_ip = IPAddress {
                    kind: IPAddressKind::V6,
                    address: String::from("::1"),
                };

                dbg!(localhost);
                dbg!(loop_ip);
            }),
            // 自定义枚举值的类型 - 可以是任意类型, 甚至结构体，甚至枚举
            Box::new(|| {
                {
                    enum IPAddress {
                        V4(String),
                        V6(String),
                    }

                    let home = IPAddress::V4(String::from("127.0.0.1"));
                    let loop_back = IPAddress::V6(String::from("::1"));
                }

                {
                    enum IPAddress {
                        V4(u32, u32, u32, u32), // 元组
                        V6(String),
                    }

                    let home = IPAddress::V4(127, 0, 0, 1);
                    let loop_back = IPAddress::V6(String::from("::1"));
                }
                // 可以是任意类型
                {
                    enum Message {
                        Quit,
                        Move { x: i32, y: i32 },
                        Write(String),
                        ChangeColor(i32, i32, i32),
                    }
                }
            }),
            // 在枚举中定义方法 impl
            Box::new(|| {
                #[derive(Debug)]
                enum Message {
                    Quit,
                    Move { x: i32, y: i32 },
                    Write(String),
                    ChangeColor(i32, i32, i32),
                }

                impl Message {
                    fn call(&self) {
                        match self {
                            Message::Quit => println!("Quit variant, no properties"),
                            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
                            Message::Write(s) => println!("Write message: {}", s),
                            Message::ChangeColor(r, g, b) => {
                                println!("Change color to ({}, {}, {})", r, g, b)
                            }
                        }
                    }
                }

                let m = Message::Write(String::from("hello world"));
                m.call();
            }),
        ],
        None,
    );
}
