use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn run(ctx: Context, msg: Message) {
    let content = format!("👤 Usuário: {}\n🆔 ID: {}", msg.author.name, msg.author.id);
    let _ = msg.channel_id.say(&ctx.http, content).await;
}