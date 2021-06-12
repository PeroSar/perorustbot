use crate::plugins::req::get_url;
use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;
use teloxide::requests::ResponseResult;

pub async fn perocmd_udi(cx: UpdateWithCx<AutoSend<Bot>, Message>, term: String) -> ResponseResult<Message> {
    if term.is_empty() {
        cx.reply_to("provide a <b>word</b> to get definition!").parse_mode(Html).await
    } else {
        let defin = get_def(&term).await;
        if defin.is_none() {
            cx.reply_to("something went wrong").await
        } else {
            cx.reply_to(format!("Definition for <b>{}</b>:
{}", term, defin.unwrap())).parse_mode(Html).await
        }
    }
}

async fn get_def(taxt: &String) -> Option<String> {
    let url = format!("https://api.urbandictionary.com/v0/define?term={}",taxt);
    let response = match get_url(url).await {
        Some(val) => val,
        None => return None,
    };
    if response["list"].as_array().unwrap().is_empty() {
        return Some(String::from("No definition found!"));
    }
    let target = &response["list"][0]["definition"];

    Some(target.to_string().trim_matches('"').to_string())}
