use serenity::client::Context;
use serenity::model::prelude::*;
use serenity::http::CacheHttp;


pub async fn run(ctx: Context, msg: Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong! 🏓").await {
        println!("Erro ao responder: {:?}", why);
    }
}