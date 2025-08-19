pub mod front_of_house {
    // 定义公有的属性，方法，枚举，结构体
    pub enum Appetizer {
        Soup,
        Salad,
    }

    /* 公有模块 */
    pub mod hosting {
        pub fn add_to_waitlist() {
            // 相对路径
            super::serving::take_order();
        }

        fn seat_at_table() {}
    }

    // 私有模块
    mod serving {
        pub fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

pub mod bank;
