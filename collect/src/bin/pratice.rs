use std::collections::HashMap;

use util;

fn average(items: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let len = items.len();
    for item in items {
        sum += item;
    }
    let res = (sum as f64) / (len as f64);
    res
}

fn most(items: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for item in items {
        let count = map.entry(*item).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut result = 0;

    for (key, value) in &map {
        // let val = map.get(&key);
        // let unwrap = val.unwrap();
        if value > &max {
            max = *value;
            result = *key;
        }
    }
    result
}

fn mid(items: &Vec<i32>) -> i32 {
    let mut cloned = items.clone();
    cloned.sort();
    let len = cloned.len();
    let mid = len / 2;

    let result = cloned[mid];
    result
}

// 练习题
fn main() {
    util::tester(
        vec![
            // 给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
            Box::new(|| {
                #[derive(Debug, Clone)]
                enum Stat {
                    Average(f64),
                    Mid(i32),
                    Most(i32),
                }

                let list = vec![5, 4, 6, 9, 1, 4, 2, 1, 4];

                let average = average(&list);
                let mid = mid(&list);
                let most = most(&list);

                let result = vec![Stat::Average(average), Stat::Mid(mid), Stat::Most(most)];

                dbg!(result);
            }),
            // 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
            // Box::new(|| {}),
            // 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。
            // Box::new(|| {}),
        ],
        None,
    );
}
