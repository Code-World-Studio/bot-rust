mod handler;
mod commands;
mod events;

use serenity::prelude::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token não encontrado no .env");

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handler::Handler)
        .await
        .expect("Erro ao criar o cliente");

    if let Err(why) = client.start().await {
        println!("Erro ao iniciar o bot: {:?}", why);
    }
}