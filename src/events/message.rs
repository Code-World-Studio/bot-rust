use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::commands::{ping, say, userinfo};

pub async fn on_message(ctx: Context, msg: Message) {
    if msg.author.bot { return; }

    let content = msg.content.trim();

    if content == "!ping" {
        ping::run(ctx, msg).await;
    } else if content.starts_with("!say ") {
        let text = content.replacen("!say ", "", 1);
        say::run(ctx, msg, text).await;
    } else if content == "!userinfo" {
        userinfo::run(ctx, msg).await;
    }
}