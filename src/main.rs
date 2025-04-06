use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use dotenv::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                println!("Erro ao enviar mensagem: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Conectado como {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Token não encontrado");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Erro ao criar client");

    if let Err(why) = client.start().await {
        println!("Erro ao iniciar o bot: {:?}", why);
    }
}