use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::events::{ready::on_ready, message::on_message};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        on_ready(ctx, ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        on_message(ctx, msg).await;
    }
}