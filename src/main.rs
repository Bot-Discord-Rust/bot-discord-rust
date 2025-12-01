use std::env;

use serenity::{
    async_trait,
    all::CreateMessage,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let content = msg.content.trim();
        
        if content.eq_ignore_ascii_case("!ping") {
            println!("Received !ping command from {}", msg.author.name);
            
            if let Err(why) = ctx.http
                .send_message(
                    msg.channel_id,
                    Vec::new(),
                    &CreateMessage::new().content("Pong!"),
                )
                .await
            {
                eprintln!("Error sending message: {:?}", why);
            }
        }
        else if let Some(num_faces) = content.strip_prefix("!dé ").and_then(|s| s.trim().parse::<u32>().ok()) {
            if num_faces >= 2 {
                let result = rand::random::<u32>() % num_faces + 1;
                if let Err(why) = ctx.http
                    .send_message(
                        msg.channel_id,
                        Vec::new(),
                        &CreateMessage::new().content(
                            format!("🎲 Résultat du dé à {} faces: {}", num_faces, result)
                        ),
                    )
                    .await
                {
                    eprintln!("Error sending message: {:?}", why);
                }
            } else {
                if let Err(why) = ctx.http
                    .send_message(
                        msg.channel_id,
                        Vec::new(),
                        &CreateMessage::new().content("❌ Le dé doit avoir au moins 2 faces !"),
                    )
                    .await
                {
                    eprintln!("Error sending message: {:?}", why);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = serenity::model::gateway::GatewayIntents::GUILD_MESSAGES
        | serenity::model::gateway::GatewayIntents::DIRECT_MESSAGES
        | serenity::model::gateway::GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}