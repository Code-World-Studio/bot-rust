use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return; // Ignora mensagens de bots
        }

        match msg.content.as_str() {
            "!ping" => {
                let _ = msg.channel_id.say(&ctx.http, "🏓 Pong!").await;
            }
            "!bot" => {
                let _ = msg.channel_id.say(&ctx.http, "🤖 Estou vivo em Rust!").await;
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("✅ Conectado como {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("❌ Token do Discord não encontrado no .env");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("❌ Erro ao criar o client");

    if let Err(why) = client.start().await {
        eprintln!("Erro ao iniciar o bot: {:?}", why);
    }
}
