use {
    crate::bot::command,
    serenity::all::{Channel, Context, EventHandler, GetMessages, Message, MessageId},
};

pub struct Purge;

#[serenity::async_trait]
impl EventHandler for Purge {
    async fn message(&self, ctx: Context, message: Message) {
        let Some(args) = command::parse(
            &message,
            "purge",
            command::Case::Insensitive,
            command::Prefix::Yes,
        ) else {
            return;
        };

        if args.len() != 1 {
            if let Err(why) = message
                .reply(
                    &ctx.http,
                    "Expected 1 argument, please specify a number of message to purge",
                )
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        }

        // This unwrap is fine as we just checked the len of the args
        let Ok(count) = args.first().unwrap().parse::<u8>() else {
            if let Err(why) = message
                .reply(
                    &ctx.http,
                    "Could not parse count argument, make sure it's a positive integer and less than 100",
                )
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        let Ok(channel) = message.channel(&ctx.http).await else {
            if let Err(why) = message.channel_id.say(&ctx.http, "Error").await {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        let Channel::Guild(guild_channel) = channel else {
            if let Err(why) = message
                .reply(&ctx.http, "Private channels are not supported yet")
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        let Ok(messages) = guild_channel
            .messages(
                &ctx.http,
                GetMessages::new().before(message.id).limit(count),
            )
            .await
        else {
            if let Err(why) = message
                .reply(
                    &ctx.http,
                    "Failed to fetch the recent messages for this channel",
                )
                .await
            {
                error!("Could not send error message due to: {why}");
            }
            return;
        };

        // Delete the purge request message
        if let Err(why) = message.delete(&ctx.http).await {
            error!("Failed to delete purge request message due to: {why}");
            return;
        }

        if let Err(why) = message
            .channel_id
            .delete_messages(
                &ctx.http,
                messages
                    .iter()
                    .map(|msg| msg.id)
                    .collect::<Vec<MessageId>>(),
            )
            .await
        {
            error!("Failed to delete messages due to: {why}");
            return;
        }

        let confirmation_message = match message
            .channel_id
            .say(&ctx.http, format!("Deleted {count} messages"))
            .await
        {
            Ok(msg) => msg,
            Err(why) => {
                error!("Failed to send confirmation message due to: {why}");
                return;
            }
        };

        tokio::task::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            if let Err(why) = confirmation_message.delete(&ctx.http).await {
                error!("Failed to delete confimation message due to: {why}");
            }
        });
    }
}
