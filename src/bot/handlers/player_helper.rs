pub struct PlayerHelper;

#[serenity::async_trait]
impl serenity::all::EventHandler for PlayerHelper {
    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        super::module_command(&ctx, "PlayerHelper", message).await;
    }

    async fn interaction_create(&self, ctx: serenity::all::Context, i: serenity::all::Interaction) {
        use serenity::all::{
            CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse,
            CreateInteractionResponseMessage, Interaction,
        };

        let Interaction::Command(c) = i else {
            return;
        };

        for option in c.data.options.iter() {
            println!("{option:?}");
        }

        if let Err(e) = c
            .create_response(
                ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Content")
                        .embed(
                            CreateEmbed::new()
                                .author(CreateEmbedAuthor::new("Me"))
                                .title("This is a title")
                                .description("Simple description")
                                .footer(CreateEmbedFooter::new("Footer")),
                        )
                        .ephemeral(true),
                ),
            )
            .await
        {
            error!(
                "Failed to send a reponse to command {} due to: {e}",
                c.data.name
            )
        }
    }
}
