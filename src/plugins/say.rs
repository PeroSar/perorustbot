use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

pub async fn perocmd_say(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    texx: String,
) -> ResponseResult<Message> {
    let msgid = if let Some(reply) = cx.update.reply_to_message() {
        reply.id
    } else {
        cx.update.id
    };
    cx.answer(texx)
        .reply_to_message_id(msgid)
        .parse_mode(Html)
        .await
}
