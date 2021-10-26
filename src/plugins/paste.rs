use crate::plugins::req::get_url;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup,
};

pub async fn perocmd_paste(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    cntnt: String,
) -> ResponseResult<Message> {
    if let Some(reply) = cx.update.reply_to_message() {
        let url = format!(
            "https://api.itayki.com/paste?content={}",
            reply.text().unwrap()
        );
        let response = get_url(url.to_string()).await;
        let pasteu = &response.clone().unwrap()["paste_url"]
            .to_string()
            .trim_matches('"')
            .to_string();
        cx.reply_to(format!("I have pasted this text!"))
            .reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(
                vec![vec![InlineKeyboardButton::new(
                    "Paste URL",
                    InlineKeyboardButtonKind::Url(pasteu.into()),
                )]],
            )))
            .await
    } else {
        if cntnt.is_empty() {
            cx.reply_to("Please provide some text or reply to a message to paste it!")
                .await
        } else {
            let url = format!("https://api.itayki.com/paste?content={}", cntnt);
            let response = get_url(url.to_string()).await;
            let pasteu = &response.clone().unwrap()["paste_url"]
                .to_string()
                .trim_matches('"')
                .to_string();
            cx.reply_to(format!("I have pasted this text!"))
                .reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(
                    vec![vec![InlineKeyboardButton::new(
                        "Paste URL",
                        InlineKeyboardButtonKind::Url(pasteu.into()),
                    )]],
                )))
                .await
        }
    }
}
