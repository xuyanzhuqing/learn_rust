use std::collections::HashMap;

use util;

fn main() {
    util::tester(
        // 所有的键必须是相同类型，值也必须都是相同类型
        vec![
            Box::new(|| {
                let mut scores = HashMap::new();
                scores.insert(String::from("red"), 100);
                scores.insert(String::from("blue"), 100);

                dbg!(scores);
            }),
            // 用队伍列表和分数列表创建哈希 map
            Box::new(|| {
                let teams = vec![String::from("Blue"), String::from("Yellow")];
                let initial_scores = vec![10, 50];

                let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
                println!("{:?}", scores); // {"Yellow": 50, "Blue": 10}
            }),
            Box::new(|| {
                let field_name = String::from("Favorite color");
                let field_value = String::from("Blue");

                let mut map = HashMap::new();
                map.insert(field_name, field_value);
                // 这里 field_name 和 field_value 不再有效，
                // 尝试使用它们看看会出现什么编译错误！
                // borrow of moved value: `field_value`
                // println!("{}, {}", field_name, field_value);
            }),
            // 获取元素
            Box::new(|| {
                let mut scores = HashMap::new();
                scores.insert(String::from("red"), 100);
                scores.insert(String::from("blue"), 100);

                let key = String::from("red");
                println!("{:?}", scores.get(&String::from("red")));
                println!("{:?}", scores.get(&key));
                let value = scores.get(&key);
                println!("{}", value.unwrap());
            }),
            // 遍历
            Box::new(|| {
                let mut scores = HashMap::new();
                scores.insert(String::from("red"), 100);
                scores.insert(String::from("blue"), 100);
                scores.insert(String::from("blue"), 200); // 覆盖

                // 只在没有对应 key 时插入
                scores.entry(String::from("red")).or_insert(101);
                scores.entry(String::from("yellow")).or_insert(101); //

                for (key, value) in &scores {
                    println!("key = {}, value = {}", key, value);
                }
            }),
        ],
        None,
    );
}
