use util;

// 切片
fn main() {
    util::tester(
        vec![
            Box::new(|| {
                println!("执行动作 1");
                println!("执行动作 2");
            }),
            Box::new(|| {
                println!("执行动作 3");
                println!("执行动作 4");
            }),
        ],
        None,
    );
}
