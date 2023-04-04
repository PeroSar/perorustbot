use crate::handler;
use crate::telegram;
use tokio::task;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn start_bot() -> Result {
    telegram::initialize().await?;

    println!("Waiting for messages...");

    while let Some(update) = tokio::select! {
        _ = tokio::signal::ctrl_c() => Ok(None),
        result = telegram::client().next_update() => result,
    }? {
        let handle = telegram::client().clone();
        task::spawn(async move {
            match handler::handle_update(handle, update).await {
                Ok(_) => {}
                Err(e) => eprintln!("Error handling updates!: {}", e),
            }
        });
    }

    println!("Saving session file and exiting...");
    telegram::client()
        .session()
        .save_to_file("perorustbot.session")?;
    Ok(())
}
