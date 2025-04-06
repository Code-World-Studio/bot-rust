use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub async fn on_ready(_ctx: Context, ready: Ready) {
    println!("Bot conectado como {}", ready.user.name);
}