mod handler;
mod commands;
mod events;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;
use tracing_subscriber;
use serenity::http::CacheHttp;


#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("info").init();

    let token = env::var("DISCORD_TOKEN").expect("Token não encontrado no .env");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await
        .expect("Erro ao criar client");

    if let Err(why) = client.start().await {
        println!("Erro ao iniciar bot: {:?}", why);
    }
}