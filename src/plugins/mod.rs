mod corona;
mod ctid;
mod help;
mod ipinfo;
mod paste;
mod req;
mod say;
mod spb;
mod sudo;
mod urbandict;

use corona::perocmd_cs;
use ctid::perocmd_ctid;
use help::perocmd_help;
use ipinfo::perocmd_ipi;
use paste::perocmd_paste;
use say::perocmd_say;
use spb::perocmd_spb;
use sudo::{pero_sudocmd_sh, pero_sudocmd_upload};
use urbandict::perocmd_udi;

use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;
use teloxide::utils::command::BotCommand;

#[derive(BotCommand)]
#[command(rename = "lowercase")]
pub enum Command {
    Help,
    Say {
        texx: String,
    },
    Ping,
    #[command(prefix = "p.")]
    Sh {
        cmnd: String,
    },
    Ctid,
    Udi {
        term: String,
    },
    #[command(prefix = "p.")]
    Upload {
        file: String,
    },
    Cs {
        ctry: String,
    },
    Ipi {
        ip: String,
    },
    Spb {
        usr: String,
    },
    Paste {
        cntnt: String,
    },
    Start,
}

#[allow(unreachable_patterns)]
#[allow(unused_must_use)]
pub async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => perocmd_help(cx).await,
        Command::Start => {
            cx.reply_to("Hello, type /help@Pero_Rust_Bot to see a list of my commands!")
                .await
        }
        Command::Say { texx } => perocmd_say(cx, texx).await,
        Command::Ping => cx.reply_to("<i>Pong!</i>").parse_mode(Html).await,
        Command::Sh { cmnd } => pero_sudocmd_sh(cx, cmnd).await,
        Command::Ctid => perocmd_ctid(cx).await,
        Command::Udi { term } => perocmd_udi(cx, term).await,
        Command::Upload { file } => pero_sudocmd_upload(cx, file).await,
        Command::Cs { ctry } => perocmd_cs(cx, ctry).await,
        Command::Ipi { ip } => perocmd_ipi(cx, ip).await,
        Command::Spb { usr } => perocmd_spb(cx, usr).await,
        Command::Paste { cntnt } => perocmd_paste(cx, cntnt).await,
    };

    Ok(())
}
