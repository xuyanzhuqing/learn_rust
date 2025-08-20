use util;
// 存储在堆上

fn main() {
    util::tester(
        vec![
            Box::new(|| {
                // 空序列
                let v: Vec<i32> = Vec::new();

                // 初始化序列
                let v = vec![1, 2, 3];

                // 更新
                let mut v = Vec::new();
                v.push(1);
                v.push(2);
                // <--- 离开作用域时序列被丢弃
            }),
            Box::new(|| {
                let v = vec![1, 2, 3, 4];
                let index: usize = 2;
                let third = &v[index]; // 方式一， 超出范围报错

                println!("{}", third);

                match v.get(index) /* 方式二 */ {
                    None => println!("not existed"),
                    Some(third) => println!("there is a element value is {}", third),
                }
            }),
            // 错误示范，所有权问题
            Box::new(|| {
                let mut v = vec![1, 2, 3, 4];
                let first = &v[0];
                // 由于执行push 后，是将堆上的原序列复制到一个新的地址上，导致 v 上的 first被清空
                // v.push(5);
                println!("{}", first);
            }),
            // 遍历序列
            Box::new(|| {
                {
                    let items: [i64; 5] = [5, 6, 7, 8, 9];
                    let v: Vec<i64> = items.into();

                    for i in &v {
                        println!("{}", i);
                    }
                }
                {
                    let items: [i64; 5] = [5, 6, 7, 8, 9];
                    let mut v: Vec<i64> = items.into();

                    for i in &mut v {
                        *i += 50;
                    }

                    for i in v {
                        println!("{}", i)
                    }
                }
            }),
            // 存储多种类型数据的序列
            Box::new(|| {
                enum SpreadsheetCell {
                    Int(i32),
                    Float(f64),
                    Text(String),
                }

                let row = vec![
                    SpreadsheetCell::Int(3),
                    SpreadsheetCell::Float(100.00),
                    SpreadsheetCell::Text(String::from("hello world")),
                ];
            }),
        ],
        None,
    );
}
