// 如何引入库文件

use project::front_of_house as house; // 自定义名称，防止命名冲突
use project::front_of_house::hosting; // 常用
use project::front_of_house::hosting::add_to_waitlist; // 不常用
// use project::*; //glob 语法引入所有
use project::{eat_at_restaurant, front_of_house::Appetizer}; // 嵌套引入

/** 银行存取款业务 */
use project::bank::deposit;
use project::bank::withdraw;

fn main() {
    println!("Hello, world!");
    add_to_waitlist();
    hosting::add_to_waitlist();
    house::hosting::add_to_waitlist();

    deposit::save();
    withdraw::handover();
}
