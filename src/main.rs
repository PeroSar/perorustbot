#![allow(deprecated)]

mod plugins;

use crate::plugins::answer;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Pero Rust Bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, "Pero_Rust_Bot", answer).await
}
