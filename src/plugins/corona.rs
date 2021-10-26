use crate::plugins::req::get_url;
use teloxide::prelude::*;
use teloxide::requests::ResponseResult;
use teloxide::types::ParseMode::Html;

pub async fn perocmd_cs(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    ctry: String,
) -> ResponseResult<Message> {
    if ctry.trim().is_empty() {
        cx.reply_to("Provide a country name!").await
    } else {
        let url = format!("https://corona.lmao.ninja/v3/covid-19/countries/{}", ctry);
        let response = get_url(url.to_string()).await;
        if response.is_none() {
            cx.reply_to(
                "Something went wrong!
Please try again.",
            )
            .await
        } else {
            let cname = &response.clone().unwrap()["country"];
            let cases = &response.clone().unwrap()["cases"];
            let tcases = &response.clone().unwrap()["todayCases"];
            let atv = &response.clone().unwrap()["active"];
            let crit = &response.clone().unwrap()["critical"];
            let ded = &response.clone().unwrap()["deaths"];
            let tded = &response.clone().unwrap()["todayDeaths"];
            let rec = &response.clone().unwrap()["recovered"];
            let tsts = &response.clone().unwrap()["tests"];
            let pop = &response.clone().unwrap()["population"];
            cx.reply_to(format!(
                "<b>Country:</b> <code>{}</code>
<b>Cases:</b> <code>{}</code>
<b>Cases today:</b> <code>{}</code>
<b>Active:</b> <code>{}</code>
<b>Critical:</b> <code>{}</code>
<b>Deaths:</b> <code>{}</code>
<b>Today deaths:</b> <code>{}</code>
<b>Recovered:</b> <code>{}</code>
<b>Tests:</b> <code>{}</code>
<b>Population:</b> <code>{}</code>",
                cname.to_string().trim_matches('"').to_string(),
                cases.to_string().trim_matches('"').to_string(),
                tcases.to_string().trim_matches('"').to_string(),
                atv.to_string().trim_matches('"').to_string(),
                crit.to_string().trim_matches('"').to_string(),
                ded.to_string().trim_matches('"').to_string(),
                tded.to_string().trim_matches('"').to_string(),
                rec.to_string().trim_matches('"').to_string(),
                tsts.to_string().trim_matches('"').to_string(),
                pop.to_string().trim_matches('"').to_string()
            ))
            .parse_mode(Html)
            .await
        }
    }
}
