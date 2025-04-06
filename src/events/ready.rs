use serenity::client::Context;
use serenity::model::gateway::Ready;
use tracing::info;

pub async fn on_ready(_ctx: Context, ready: Ready) {
    info!("Bot conectado como {}", ready.user.name);
}