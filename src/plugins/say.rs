use grammers_client::types::Message;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn perocmd_say(message: Message, text: String) -> Result {
    message.reply(text).await?;
    Ok(())
}
