mod req;
mod help;
mod say;
mod sudo;
mod ctid;
mod urbandict;
mod corona;
mod ipinfo;
mod paste;

use help::perocmd_help;
use say::perocmd_say;
use sudo::{ pero_sudocmd_sh, pero_sudocmd_upload };
use ctid::perocmd_ctid;
use urbandict::perocmd_udi;
use corona::perocmd_cs;
use ipinfo::perocmd_ipi;
use paste::perocmd_paste;

use teloxide::prelude::*;
use teloxide::types::ParseMode::Html;
use teloxide::utils::command::BotCommand;
use teloxide::requests::ResponseResult;

#[derive(BotCommand)]
#[command(rename = "lowercase")]
pub enum Command {
    Help,
    Say { texx: String },
    Ping,
    #[command(prefix = "p.")]
    Sh { cmnd: String },
    Ctid,
    Udi { term: String },
    #[command(prefix = "p.")]
    Upload { file: String },
    Cs { ctry: String },
    Ipi { ip: String },
    Paste { cntnt: String },
}

#[allow(unreachable_patterns)]
#[allow(unused_must_use)]
pub async fn answer(cx: UpdateWithCx<AutoSend<Bot>, Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => perocmd_help(cx).await,
        Command::Say { texx } => perocmd_say(cx, texx).await,
        Command::Ping => cx.reply_to("<i>Pong!</i>").parse_mode(Html).await,
        Command::Sh { cmnd } => pero_sudocmd_sh(cx, cmnd).await,
        Command::Ctid => perocmd_ctid(cx).await,
        Command::Udi { term } => perocmd_udi(cx, term).await,
        Command::Upload { file } => pero_sudocmd_upload(cx, file).await,
        Command::Cs { ctry } => perocmd_cs(cx, ctry).await,
        Command::Ipi { ip } => perocmd_ipi(cx, ip).await,
        Command::Paste { cntnt } => perocmd_paste(cx, cntnt).await,
    };

    Ok(())
}
