use teloxide::prelude::*;
use std::process::Command as ExeCmd;
use std::path::{ Path, PathBuf };
use teloxide::types::InputFile;
use teloxide::types::ParseMode::Html;
use teloxide::requests::ResponseResult;

pub async fn pero_sudocmd_sh(cx: UpdateWithCx<AutoSend<Bot>, Message>, cmnd: String) -> ResponseResult<Message> {
    if is_sudo(&cx) { 
        let shout  = ExeCmd::new("bash").arg("-c").arg(cmnd).output().expect("FAIL");
        cx.reply_to(format!("<i>{}</i>
<code>{}</code>
<code>{}</code>", shout.status, String::from_utf8_lossy(&shout.stdout), String::from_utf8_lossy(&shout.stderr))).parse_mode(Html).await
    } else {
        cx.reply_to(format!("Retards like {} are not allowed to use this command.", cx.update.from().unwrap().first_name)).await
    }
}

pub async fn pero_sudocmd_upload(cx: UpdateWithCx<AutoSend<Bot>, Message>, fname: String) -> ResponseResult<Message> {
    if is_sudo(&cx) {
        if Path::new(&fname).exists() {
            let file = InputFile::File(PathBuf::from(fname));
            cx.requester.send_document(cx.update.chat_id(), file).await
        } else {
            cx.reply_to("That file doesn't exist!").await
        }
    } else {
        cx.reply_to("You are not allowed to use this!").await
    }
}


fn is_sudo(cx: &UpdateWithCx<AutoSend<Bot>, Message>) -> bool {
    let sudousrs = vec![1502313477]; // will add more users later
    if sudousrs.contains(&cx.update.from().unwrap().id) {
        return true;
    } else {
        return false;
    }
}
