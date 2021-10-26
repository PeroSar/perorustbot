use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

pub async fn perocmd_ctid(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> ResponseResult<Message> {
    if let Some(reply) = cx.update.reply_to_message() {
        cx.reply_to(format!(
            "chat id: <code>{}</code>
your id: <code>{}</code>
{}'s id: <code>{}</code>",
            cx.update.chat_id(),
            cx.update.from().unwrap().id,
            reply.from().unwrap().first_name,
            reply.from().unwrap().id
        ))
        .parse_mode(Html)
        .await
    } else {
        cx.reply_to(format!(
            "chat id: <code>{}</code>
your id: <code>{}</code>",
            cx.update.chat_id(),
            cx.update.from().unwrap().id
        ))
        .parse_mode(Html)
        .await
    }
}
