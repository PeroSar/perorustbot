use librustbin::Client;
use reqwest::Url;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup, ReplyMarkup,
};

pub async fn perocmd_paste(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    mut ext: String,
) -> ResponseResult<Message> {
    if let Some(reply) = cx.update.reply_to_message() {
        if !ext.is_empty() {
            ext = format!(".{}", ext);
        }

        let url = format!(
            "{}{}",
            Client::new()
                .paste_highlight(reply.text().unwrap().to_string())
                .unwrap()
                .trim(),
            ext
        );

        cx.reply_to(format!("I have pasted this text! <a href='{}'>ㅤ</a>", url))
            .reply_markup(ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(
                vec![vec![InlineKeyboardButton::new(
                    "Paste URL",
                    InlineKeyboardButtonKind::Url(Url::parse(&url).unwrap()),
                )]],
            )))
            .parse_mode(Html)
            .await
    } else {
        cx.reply_to("Please reply to a message to paste it!").await
    }
}
