use serenity::all::{Context, EventHandler, Message};

pub struct Basic;

#[serenity::async_trait]
impl EventHandler for Basic {
    async fn message(&self, ctx: Context, message: Message) {
        if message.content.to_lowercase() == "hi" {
            if let Err(why) = message.channel_id.say(&ctx.http, "Hello").await {
                println!("Failed to respond to ping message due to: {why}");
            }
        }
    }
}
