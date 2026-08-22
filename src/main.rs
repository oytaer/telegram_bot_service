mod utils;
mod routers;
mod middlewares;
mod controllers;
// telegram_core 为完整的 Telegram Bot API 封装库，服务入口尚未接入调用，
// 库内公共类型与方法暂未被引用，故允许存在未使用代码以保持 0 警告
#[allow(dead_code)]
mod telegram_core;

fn main() {
    println!("Hello, world!");
}
