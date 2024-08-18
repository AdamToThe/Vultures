use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use serenity::framework::standard::macros::{
    // check,
    command,
    group,
    // help,
    // hook
};

use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::{AttachmentType, Message};
use serenity::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct YeQuote {
    quote: String
}

#[group]
#[commands(ping, kanye, screenshot)]
pub struct Fun;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong!!").await?;
    Ok(())
}



#[command]
async fn kanye(ctx: &Context, msg: &Message) -> CommandResult {

    let client = reqwest::Client::new();

    let res = client
        .get("https://api.kanye.rest/")
        .send()
        .await
        .unwrap();

    let q = match res.json::<YeQuote>().await {
        Ok(parsed) => parsed.quote,
        _ => "Not now, try later".into()
    };

    msg.reply(&ctx.http, &q).await?;

    Ok(())

}

#[command]
pub async fn screenshot(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut url = args.single::<String>()?;


    if !(url.starts_with("http://") || url.starts_with("https://")) {
        url = format!("https://{url}");
    }
;

    let chunk = reqwest::Client::new()
            .get(format!("https://image.thum.io/get/width/1080/crop/760/{url}"))
            .send()
            .await?
            .bytes()
            .await?
            .to_vec();

    let b = Cow::from(chunk.as_slice());
    
    msg.channel_id.send_message(&ctx.http,
         move |m| {
            m.add_file(
                AttachmentType::Bytes { 
                    data: b,
                    filename: String::from("ss.png") 
                }
            )
         }).await?;

    Ok(())
}
