fn main() {
    println!("{}", if_else_if_function(100));
    simple_if_else();
    // loop_function();
    println!("{}", loop_return_function());
    loop_continue_function();
    loop_for_function();
}


fn if_else_if_function (num: i32) -> &'static str {
    if num > 100 {
        "num is greater than 100"
    } else if num < 100 {
        "num is less than 100"
    } else {
        "num is equal to 100"
    }
}

// 三元表达式
fn simple_if_else () {
    let flag: i8 = if true { 10 } else { -10 };
    println!("{}", flag)
}


// fn loop_function () {
//     loop {
//         println!("fight to the end!!");
//     }
// }

fn loop_return_function () ->i8 {
    let mut i: i8 = 0;
    loop {
        if i >= 100 {
            break i
        }
        i += 1;
    }
}

fn loop_continue_function () {
    let mut i: i8 = 0;

    while i < 100 {
        i += 1;
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
}

fn loop_for_function () {
    let mut sum: i32 = 0;
    let arr: [i32; 5] = [1,2,3,4,5];

    for n in arr {
        println!("{}", n);
    }
}