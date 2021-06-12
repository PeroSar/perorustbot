use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;
use teloxide::requests::ResponseResult;

pub async fn perocmd_help(cx: UpdateWithCx<AutoSend<Bot>, Message>) -> ResponseResult<Message> {
    cx.reply_to(format!("Hello! {},

I'm a bot made by <a href='tg://user?id=1502313477'>Pero Sar</a> in <a href='https://rust-lang.org'>rust</a>
Here's a list of my commands:
<code>/help</code> - <i>display this text.</i>
<code>/say [text to echo]</code> - <i>make me echo any text.</i>
<code>/ping</code> - <i>Pong!</i>
<code>/start</code> - <i>start me.</i>
<code>/ctid</code> - <i>get the current chat's id.</i>
<code>/follow [url]</code> - <i>follow a url until it redirects.</i>
x<code>/ani [anime name]</code> - <i>search a anime.</i>
<code>/cs [country]</code> - <i>Corona stats for a country.</i>
<code>/udi [term]</code> - <i>get definition of a word from urbandictionary.</i>
<code>/ipi [IP]</code> - <i>get ip address info.</i>", cx.update.from().unwrap().first_name)).parse_mode(Html).await
}
