use grammers_client::{Client, Config, InitParams};
use grammers_session::Session;
use log;
use simple_logger::SimpleLogger;
use std::env;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

static mut TELEGRAM_CLIENT: Option<Client> = None;

pub async fn initialize() -> Result {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let api_id = env!("TG_ID").parse().expect("TG_ID invalid");
    let api_hash = env!("TG_HASH").to_string();
    let token = env!("TG_TKN").to_string();

    println!("Connecting to Telegram...");
    let client = Client::connect(Config {
        session: Session::load_file_or_create("perorustbot.session")?,
        api_id,
        api_hash: api_hash.clone(),
        params: InitParams {
            // Fetch the updates we missed while we were offline
            catch_up: true,
            ..Default::default()
        },
    })
    .await?;
    println!("Connected!");

    if !client.is_authorized().await? {
        println!("Signing in...");
        client.bot_sign_in(&token, api_id, &api_hash).await?;
        client.session().save_to_file("perorustbot.session")?;
        println!("Signed in!");
    }

    unsafe {
        TELEGRAM_CLIENT = Some(client);
    }

    Ok(())
}

pub fn client() -> &'static Client {
    unsafe { TELEGRAM_CLIENT.as_ref().unwrap() }
}
