// tail == None 执行最后定义的函数
// tail == n 执行指定index的函数
// tail == MAX 执行所有函数
pub fn tester(fns: Vec<Box<dyn Fn()>>, tail: Option<usize>) {
    let last_index: usize = fns.len() - 1;
    let tail = tail.unwrap_or(last_index);
    if tail == usize::MAX {
        for _fn in fns {
            _fn();
        }
    } else {
        fns[tail]();
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_tester() {
        let all = usize::MAX;
        tester(
            vec![
                Box::new(|| {
                    println!("执行动作 1");
                    println!("执行动作 2");
                }),
                Box::new(|| {
                    println!("执行动作 3");
                    println!("执行动作 4");
                }),
            ],
            Some(all),
        );
    }
}
