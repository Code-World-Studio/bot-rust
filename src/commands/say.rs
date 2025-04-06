use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: Context, msg: Message, content: String) {
    let _ = msg.channel_id.say(&ctx.http, content).await;
}