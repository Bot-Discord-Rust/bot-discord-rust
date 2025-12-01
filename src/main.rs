use std::env;

use serenity::{
    async_trait,
    builder::CreateMessage,
    model::{channel::Message, gateway::Ready, gateway::GatewayIntents},
    prelude::{Context, EventHandler},
    Client,
};
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = ctx
                .http
                .send_message(
                    msg.channel_id,
                    Vec::new(),
                    &CreateMessage::new().content("Pong!"),
                )
                .await
            {
                error!("Failed to reply: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Bot connected as {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN env var");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {why:?}");
    }
}
