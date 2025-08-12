use std::io;

fn main() {
    // 不可变变量，不能赋值
    // {
    //     println!("Hello, world!");
    //     let x = 3;
    //     x + 1;
    //     println!("x = {}", x);
    // }

    // 可变变量
    // {
    //     let mut x = 3;
    //     x = 5;
    //     println!("x = {}", x);
    // }

    // 常量
    // {
    //     const MAX_POINTS: u32 = 100_000;
    //     println!("MAX_POINTS = {}", MAX_POINTS);
    // }

    //  遮蔽
    // {   let x = 1;
    //     println!("x = {}", x);
    //     let x = "123";
    //     println!("x = {}", x);
    // }

    // 数据类型
    // 标量
    // 整型、浮点型、布尔型和字符
    // 复合类型
    // 元组，数组

    // 整数
    // 长度	有符号类型	无符号类型
    // 8 位	i8	u8
    // 16 位	i16	u16
    // 32 位	i32	u32
    // 64 位	i64	u64
    // 128 位	i128	u128
    // arch	isize	usize

    // let x = 2;
    // let y = 3;

    // let z = x + y;
    // println!("z = {}", z);

    // let z = x - y;
    // println!("z = {}", z);

    // let z = x * y;
    // println!("z = {}", z);

    // let z = x / y; // 整数除法向下取整
    // println!("z = {}", z);

    // let z = x as f64 / 3.0; // 浮点数除法
    // println!("z = {}", z);

    // let z = x % y;
    // println!("z = {}", z);


    // 整数溢出 overflowing_add overflowing_sub overflowing_mul overflowing_div
    // let max_u8 = u8::MAX;
    // let (result, overflowed) = max_u8.overflowing_add(1);
    // println!("u8 加法: {} + 1 = {}, 溢出: {}", max_u8, result, overflowed);

    // 浮点数 默认 64 位
    // 32 位	f32
    // 64 位	f64


    // 布尔型
    // let t = true;
    // let f: bool = false;
    // println!("t = {}", t);
    // println!("f = {}", f);

    // 字符型 单引号
    // let c = 'a';
    // println!("c = {}", c);

    // 字符串型 双引号
    // let s = "hello";
    // println!("s = {}", s);


    // 复合类型
    // 元组
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println!("tup = {:?}", tup);

    // 数组
    // let a = [1, 2, 3];
    // println!("a = {:?}", a);

    // 数组 长度固定 类型相同
    // let a = [1, 2, 3];
    // let a: [i32; 3] = [1, 2, 3];
    // println!("a = {:?}", a);

    // 填充默认值 （默认值；length）
    // let a = [3; 5];
    // println!("a = {:?}", a);
    get_element_at_index();
}

fn get_element_at_index() {
    let items: [i64; 5] = [5, 6, 7, 8, 9];

    println!("please insert a number");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index).expect("please insert index");

    let index: u32 = match index.trim().parse() {
        Ok(index) => index,
        Err(error) => {
            println!("error: {}", error);
            return;
        },
    };

    let result: i64 = items[index as usize];
    println!("items = {:?}; index = {}, item = {}", items, index, result);
}
