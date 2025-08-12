新项目
cargo new hello_cargo

新模块
cargo new xxx --lib

运行指定项目目录
cargo run -p xxx

编译项目
cargo build

运行项目
cargo run

查看项目依赖
cargo tree

查看项目依赖图
cargo tree --graph

查看项目依赖图（包含所有依赖）
cargo tree --graph --all

编译项目（发布模式）
cargo build --release

运行项目（发布模式）
cargo run --release


let 变量, 可重复命名，类型可变（遮蔽）let x = 1; let x = '123'
const 常量, 必须为表达式 const SECONDS_IN_ONE_HOUR = 60 * 60
mut 可变变量, 可赋值，类型不可变
