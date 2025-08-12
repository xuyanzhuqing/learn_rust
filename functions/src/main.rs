use rand::random;

fn main() {
    default_function();
    param_function(
        return_double_function(
            return_hundred_function()
        ),
        "LOVE"
    );

    println!("{}", return_any_time_function(100));
}

fn default_function () {
    println!("a function");
}

fn param_function (time: i32, action: &str) {
    println!("I {} U {} times", action, time);
}

fn return_hundred_function () -> i32 {
    100
}

fn return_double_function (num: i32) -> i32 {
    let x = 2;
    x * num // 返回值不要写分号，mmp, 骚包
}

// 保底 50 随机数
fn return_any_time_function (num: i32) -> f64 {
    let x = random::<f64>() * num as f64;

    if x < 50.0 {
        50.0 + x
    } else {
        x
    }
}