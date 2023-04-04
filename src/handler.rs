use crate::plugins;
use grammers_client::Client;
use grammers_client::Update;
use std::error::Error;

type Result = std::result::Result<(), Box<dyn Error>>;

pub async fn handle_update(_client: Client, update: Update) -> Result {
    match update {
        Update::NewMessage(message) if !message.outgoing() => {
            let mtext = message.text();

            if let Some(cmd) = get_cmd(message.text()) {
                match cmd {
                    "start" => plugins::start::perocmd_start(message).await?,
                    "say" => {
                        plugins::say::perocmd_say(
                            message.clone(),
                            get_arg(mtext, 0)
                                .unwrap_or("Say what? Give me some text to say!".to_string()),
                        )
                        .await?
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn get_cmd(input: &str) -> Option<&str> {
    if let Some(stripped) = input.strip_prefix('/') {
        let input = stripped;

        if let Some(index) = input.find(|c| c == ' ' || c == '_') {
            let inp = &input[..index];
            Some(inp.split("@perotestbot").next().unwrap_or(input))
        } else {
            Some(input.split("@perotestbot").next().unwrap_or(input))
        }
    } else {
        None
    }
}

fn get_arg(input: &str, n: usize) -> Option<String> {
    let mut words = input.split(|c| c == ' ' || c == '_');

    if n == 0 {
        words.next();
        let w = words.collect::<Vec<_>>().join(" ");
        return if w.is_empty() { None } else { Some(w) };
    }

    words.skip(1).nth(n - 1).map(|s| s.to_string())
}
