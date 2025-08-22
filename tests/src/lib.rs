pub mod demo;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

//  单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn failures() {
    //     panic!("I am doom to failure"); // 直接失败
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = demo::demo1::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = demo::demo1::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // 测试相等或者不相等
    #[test]
    fn test_assert_eq_or_assert_ne() {
        assert_eq!(3, add(1, 2));
        assert_ne!(3, add(1, 3));
        // assert!(3 == add(1, 3), "我们是不相等的额 {} == {}", 3, add(1, 3));
    }

    // #[test]
    // #[should_panic(expected = "按照我这个说明来报错，内部的报错闪开先")]
    // fn test_guess_range() {
    //     demo::demo2::Guess::new(101);
    // }

    // Result<T, E>
    #[test]
    fn result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("gun"))
        }
    }
}
