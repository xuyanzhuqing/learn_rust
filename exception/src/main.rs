use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use util;
// panic 不可恢复错误
// result 可恢复错误

fn main() {
    util::tester(
        vec![
            // panic! 与不可恢复的错误
            // 主动触发一个 panic
            Box::new(|| {
                println!("Hello, world!");
                panic!("boom, sha ka la ka");
            }),
            // 使用 panic! 的 backtrace
            //  RUST_BACKTRACE=1 cargo run -p exception --release
            //  RUST_BACKTRACE=full cargo run -p exception --release
            Box::new(|| {
                let v = vec![1, 2, 3];
                let _not_exist = v.get(99);
                v[99];
            }),
            // Result 与可恢复的错误
            Box::new(|| {
                // 主动报错
                let f = File::open("hello.txt");
                let f = match f {
                    Ok(file) => file,
                    Err(error) => {
                        panic!("文件打开失败{:?}", error)
                    }
                };
            }),
            // 匹配不同类型的错误 (基础款)
            Box::new(|| {
                let f = File::open("hello.txt");

                let f = match f {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create("hello.txt") {
                            Ok(file) => file,
                            Err(error) => panic!("文件创建失败 {:?}", error),
                        },
                        ErrorKind::PermissionDenied => panic!("无权限"),
                        other => panic!("文件打开失败 {:?}", other),
                    },
                };
            }),
            // 匹配不同类型的错误 (升级款)
            Box::new(|| {
                let f = File::open("hello.txt").unwrap_or_else(|error| {
                    if error.kind() == ErrorKind::NotFound {
                        File::create("hello.txt").unwrap_or_else(|err| {
                            panic!("文件创建失败{:?}", err);
                        })
                    } else {
                        panic!("文件打开失败 {:?}", error);
                    }
                });
            }),
            // 简要报错
            // unwrap no good
            // expect good
            Box::new(|| {
                let f = File::open("hello.txt").unwrap(); // 不建议使用
                let f = File::open("hello.txt").expect("file not found"); // 推荐，唯一标识，方便排查
            }),
            // 传播错误
            Box::new(|| {
                fn read_file() -> Result<String, io::Error> {
                    let f = File::open("hello.txt");

                    let mut f = match f {
                        Ok(file) => file,
                        Err(error) => return Err(error), // 这里
                                                         // Err(error) => panic!("{:?}", error),
                    };

                    let mut s = String::new();

                    match f.read_to_string(&mut s) {
                        Ok(_) => Ok(s),
                        Err(e) => Err(e),
                    } // 注意这里没有分号，这里 return 了
                }

                // 简化版 ？
                fn read_username_from_file() -> Result<String, io::Error> {
                    let mut f = File::open("hello.txt")?;
                    let mut s = String::new();
                    f.read_to_string(&mut s)?;
                    Ok(s)
                }

                fn read_username_from_file1() -> Result<String, io::Error> {
                    let mut s = String::new();

                    File::open("hello.txt")?.read_to_string(&mut s)?;

                    Ok(s)
                }
            }),
        ],
        None,
    );
}
