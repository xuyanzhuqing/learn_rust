// 核心概念
// 所有权
// 作用域
// 引用｜借用
// slice

fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
    demo6();
    demo7();
    demo8();
    demo9();
}

// 堆栈
fn demo1() {
    // 自面量, 栈中存值-快-内存大小不变-不可变
    let str1 = "hello world";
    // 引用类型，内存堆中-慢-内存大小不固定-可变
    let mut str2 = String::from("hello world");

    println!("{}", str1);
    // 栈中，不可变，代码报错
    // str1.push_str("!");
    // 堆中，可变
    str2.push_str("!");

    println!("{}", str2);
}

// 作用域
// 变量离开作用域后就被自动释放
fn demo2() {
    let str1 = "我是底层变量";
    {
        let str2 = "hello world";

        println!("{}", str1);
        println!("{}", str2);
        // str2 的作用域结束，变量被回收
    }
    // cannot find value `str2` in this scope
    // println!("{}", str2);

    println!("{}", str1);
    // str1 的作用域结束，变量被回收
}

// 数据移动
fn demo3() {
    let str1 = "hello world";
    let str2 = String::from("hello world");

    // 移动
    let str_1 = str1;
    println!("{}, {}", str1, str_1);

    let str_2 = str2; // 所有权转移了，str2 的堆内存所有权转移到 str_2
    println!("{}", str_2);
    // str_2.push_str("!"); // 不可借用
    // println!("{}", str2); // 不可访问
}

// 拷贝-克隆
fn demo4() {
    // 拷贝-不可变的类型
    // 整型，浮点数，bool，自面量，char, 元组
    {
        // 直接复制栈中的值，所有权没有发生转移
        let n1 = 100;
        let n2 = n1;

        let f1 = 100.00;
        let f2 = f1;

        let b1 = false;
        let b2 = b1;

        let str1 = 'c';
        let str2 = str1;

        let t1 = (1, 'c', false);
        let t2 = t1;

        // 预期结果正常
        println!("{}, {}, {}, {}, {:?}", n1, f1, b1, str1, t1);
        println!("{}, {}, {}, {}, {:?}", n2, f2, b2, str2, t2);
    }

    // 克隆-可变类型
    {
        let str1 = String::from("hello world");
        let mut str2 = str1.clone();
        // 可行，说明是在不同内存堆下的不同数据了
        str2.push_str("!");

        println!("{}", str2);
    }
}

// 函数中所有权的来龙去脉
fn demo5() {
    fn get_owner() -> String {
        let res = String::from("I am safe"); // 所有权在函数内
        // 所有权被 return 出去
        res
    }

    let str = get_owner(); // 所有权交给 str

    fn through_owner(desc: String) -> String {
        println!("{}", desc); // 当前作用域下获取到所有权
        desc // 返回所有权
    }

    let passed = through_owner(str);
    // 注意注意注意，此处所有权转移到了 through_owner 函数内部，会报错
    // println!("{:?}", str);

    // 安全合法
    println!("{}", passed);
}

// 完璧归赵
fn demo6() {
    fn length(str: String) -> (String, usize) {
        let len = str.len();
        (str, len)
    }

    let str = String::from("length");
    let (attr, length) = length(str);
    println!("{}, {}", attr, length);
}

// 引用-借用
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。
fn demo7() {
    fn length(str: &String) -> usize {
        str.len()
    }

    let tip = String::from("love and peace");
    // & 表示借用，不占所有权，只有使用权
    let len = length(&tip);
    println!("{}, {}", tip, len);

    // error 不可变引用，不能改变原有的值
    {
        // fn concat_sign(str: &String) -> usize {
        //     str.push_str("!");
        //     str.len()
        // }
        // concat_sign(&tip);
    }

    // 将上述改为可变引用即可在函数内部改变外部可变变量
    {
        let mut tip = String::from("love and peace");
        fn concat_sign(str: &mut String) -> usize {
            str.push_str("!");
            str.len()
        }
        let len = concat_sign(&mut tip);
        println!("{}, {}", tip, len);
    }
}

// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
fn demo8() {
    let mut str = String::from("hello world");

    // 只能有一个可变引用 error
    {
        // let str1 = &mut str;
        // let str2 = &mut str;

        // println!("{}, {}", str1, str2);
    }

    // 只能有多个不可变引用 error
    {
        // let str1 = &mut str;
        // let str2 = &str;

        // println!("{}, {}", str1, str2);
    }
}

// 悬垂引用 dangling references
// 对象被销毁了，指针还在
fn demo9() {
    // 错误定义 悬垂引用
    // fn dangle() -> &String {
    //     let s = String::from("hello");

    //     &s // 指针还在
    // }

    // 正确示范
    fn dangle() -> String {
        let s = String::from("hello");

        s
    }
}
