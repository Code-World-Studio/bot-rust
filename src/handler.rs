use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use crate::events;
use crate::commands;
use serenity::http::CacheHttp;

pub struct Handler;


impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::on_ready(ctx, ready).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            commands::ping::run(ctx, msg).await;
        } else {
            events::message::on_message(ctx, msg).await;
        }
    }
}