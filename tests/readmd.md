#### 并行串行

```bash
# 默认并行
cargo test

# 串行
cargo test -- --test-threads=1

# 显示测试函数中输出内容
cargo test -- --show-output

# 单独测试
cargo test one_hundred
#测试多个
cargo test test-开头


# 在函数上标注 #[ignore]
cargo test -- --ignored
```

/tests 下集成测试

```bash
cargo test -p tests

# 指定执行某一个集成测试
cargo test -p tests --test integration_test
```