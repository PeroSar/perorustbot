use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

use spamprotection::info;

pub async fn perocmd_spb(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    usr: String,
) -> ResponseResult<Message> {
    let user = info::full(usr);
    cx.reply_to(format!(
        "<b>SPB Info</b>

<b>PTID</b>: <code>{}</code>
<b>Entity type</b>: <code>{}</code>
<b>Blacklisted</b>: {} ({})
<b>Ham prediction</b>: {}
<b>Spam prediction</b>: {}
<b>Language prediction</b>: {} ({})",
        user.get_ptid(),
        user.get_type(),
        user.get_bl(),
        user.get_bl_reason(),
        user.get_ham_predict(),
        user.get_spam_predict(),
        user.get_lang(),
        user.get_lang_probabiility()
    ))
    .parse_mode(Html)
    .await
}
