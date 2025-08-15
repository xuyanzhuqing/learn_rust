use util;

// 切片
fn main() {
    util::tester(
        vec![
            Box::new(|| {
                let tip = String::from("hello world");
                let len = tip.len();
                println!("{}, {}, {}", &tip[..], &tip[0..], &tip[..len]);

                println!("{}", &tip[2..10]);
            }),
            Box::new(|| {
                let arr = [1, 2, 3, 4, 5];
                println!("{:#?}", &arr[2..4]);
            }),
        ],
        None,
    );
}
