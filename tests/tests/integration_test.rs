use tests::add;

mod common;

#[test]
fn it_adds_row() {
    // 执行公共模块
    common::setup();
    assert_eq!(add(1, 2), 3, "1 + 2 == {}", 3);
}
