use util;

fn main() {
    util::tester(
        vec![
            Box::new(|| {
                // 初始化字符串
                let s = String::new();
                let s = "";
                let s = String::from("hello world");

                let s: &str = "";
                let s: &str = &String::new();
            }),
            // 可直接存储utf-8 字符
            Box::new(|| {
                let hello = String::from("السلام عليكم");
                let hello = String::from("Dobrý den");
                let hello = String::from("Hello");
                let hello = String::from("שָׁלוֹם");
                let hello = String::from("नमस्ते");
                let hello = String::from("こんにちは");
                let hello = String::from("안녕하세요");
                let hello = String::from("你好");
                let hello = String::from("Olá");
                let hello = String::from("Здравствуйте");
                let hello = String::from("Hola");
            }),
            // 添加字符串 & 字符
            Box::new(|| {
                {
                    let s1 = "hello,";
                    let s2 = "world";
                    String::from(s1).push_str(&s2);
                    println!("{}, {}", s2, s1);
                }

                {
                    let mut s1 = String::from("lo");
                    s1.push('l');
                    println!("{}", s1);
                }
            }),
            // 字符串拼接
            Box::new(|| {
                {
                    let s1 = String::from("a");
                    let s2 = String::from("b");
                    let s3 = s1 + &s2;
                    println!("{}", s3);
                }
                {
                    let s1 = String::from("a");
                    let s2 = String::from("b");
                    let s3 = String::from("c");

                    let s = format!("{}-{}-{}", s1, s2, s3);

                    println!("{}", s);
                }
            }),
            // 字节（0 ｜ 1），标量值（音标），字形簇（字母）
            // 字符串索引
            Box::new(|| {
                // 错误示范
                {
                    let s = "hello";
                    // println!("{}", s[0]);
                    // println!("{}", &s[0..1]); 获取第一个字节

                    // 非 ASCII 字符值在运行时会报错
                    // let s1 = "中国";
                    // println!("{}", &s1[0..2]);
                }
                {
                    let s = "hello 中国";
                    println!("{:?}", s.chars()); // 返回一个链表
                }
            }),
            // 遍历
            Box::new(|| {
                // 返回字符簇（中间包含发音辅助字母）
                for c in "नमस्ते".chars() {
                    println!("{}", c);
                }

                // 返回字节
                for b in "नमस्ते".bytes() {
                    println!("{}", b);
                }
            }),
        ],
        None,
    );
}
