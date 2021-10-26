use crate::plugins::req::get_url;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

pub async fn perocmd_udi(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    term: String,
) -> ResponseResult<Message> {
    if term.is_empty() {
        let url = "http://api.urbandictionary.com/v0/random";
        let response = get_url(url.to_string()).await;
        let word = &response.clone().unwrap()["list"][0]["word"];
        let defin = &response.clone().unwrap()["list"][0]["definition"];
        cx.reply_to(format!(
            "Random definition:
<b>{}</b> : <i>{}</i>",
            word.to_string().trim_matches('"').to_string(),
            defin.to_string().trim_matches('"').to_string()
        ))
        .parse_mode(Html)
        .await
    } else {
        let defin = get_def(&term).await;
        if defin.is_none() {
            cx.reply_to("something went wrong").await
        } else {
            cx.reply_to(format!(
                "Definition for <b>{}</b>:
{}",
                term,
                defin.unwrap()
            ))
            .parse_mode(Html)
            .await
        }
    }
}

async fn get_def(taxt: &String) -> Option<String> {
    let url = format!("https://api.urbandictionary.com/v0/define?term={}", taxt);
    let response = match get_url(url).await {
        Some(val) => val,
        None => return None,
    };
    if response["list"].as_array().unwrap().is_empty() {
        return Some(String::from("No definition found!"));
    }
    let target = &response["list"][0]["definition"];

    Some(target.to_string().trim_matches('"').to_string())
}
