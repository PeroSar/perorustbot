use crate::plugins::req::get_url;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

pub async fn perocmd_ipi(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    ip: String,
) -> ResponseResult<Message> {
    if ip.trim().is_empty() {
        cx.reply_to("provide a <b>ip address</b>!")
            .parse_mode(Html)
            .await
    } else {
        let url = format!("https://ipinfo.io/{}", ip);
        let response = get_url(url.to_string()).await;
        if response.is_none() {
            cx.reply_to(
                "Something went wrong!
Please try again",
            )
            .await
        } else if &response.clone().unwrap()["status"]
            .to_string()
            .trim_matches('"')
            .to_string()
            == "404"
        {
            cx.reply_to("Please provide a valid IP address").await
        } else {
            let hname = &response.clone().unwrap()["hostname"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let city = &response.clone().unwrap()["city"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let rgn = &response.clone().unwrap()["region"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let ctry = &response.clone().unwrap()["country"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let loc = &response.clone().unwrap()["loc"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let org = &response.clone().unwrap()["org"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let postal = &response.clone().unwrap()["postal"]
                .to_string()
                .trim_matches('"')
                .to_string();
            let tz = &response.clone().unwrap()["timezone"]
                .to_string()
                .trim_matches('"')
                .to_string();
            cx.reply_to(format!(
                "<b>IP</b>: {}
<b>Hostname</b>: {}
<b>City</b>: {}
<b>Region</b>: {}
<b>Country</b>: {}
<b>Lat/Long</b>: {}
<b>Org</b>: {}
<b>Postal</b>: {}
<b>Timezone</b>: {}",
                ip, hname, city, rgn, ctry, loc, org, postal, tz
            ))
            .parse_mode(Html)
            .await
        }
    }
}
