use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Carrega variáveis do .env
    dotenv().ok();

    // Inicializa logs
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Lê o token do arquivo .env
    let token = env::var("DISCORD_TOKEN").expect("Token não encontrado no .env");

    // Exemplo básico de client
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .await
        .expect("Erro ao criar client");

    if let Err(e) = client.start().await {
        error!("Erro ao iniciar bot: {:?}", e);
    } else {
        info!("Bot iniciado com sucesso!");
    }
}
