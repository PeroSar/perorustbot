use tokio::runtime;
pub mod handler;
mod init;
pub mod plugins;
pub mod telegram;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn main() -> Result {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(init::start_bot())
}
