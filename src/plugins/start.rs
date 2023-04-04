use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn perocmd_start(message: Message) -> Result {
    message
        .reply("Hello, type /help@perotestbot to see a list of my commands!")
        .await?;
    Ok(())
}
