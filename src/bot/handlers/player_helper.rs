pub struct PlayerHelper;

#[serenity::async_trait]
impl serenity::all::EventHandler for PlayerHelper {
    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        super::module_command(&ctx, "PlayerHelper", message.clone()).await;
        help_message(ctx, message).await;
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

pub async fn help_message(ctx: serenity::all::Context, message: serenity::all::Message) {
    use crate::{
        bot::command,
        data::{Group, LeagueCordData},
    };
    use serenity::all::{
        Channel, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, GuildChannel,
    };

    let Some(args) = command::parse(
        &message,
        "help",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let ctx_data_storage = ctx.data.clone();
    let ctx_data_storage_read = ctx_data_storage.read().await;
    let Some(data) = ctx_data_storage_read.get::<LeagueCordData>() else {
        error!("Could not get tracked invites from data");
        if let Err(e) = message
            .reply(ctx.http, "An error has occured, please contact an admin")
            .await
        {
            error!("Player helper failed to send error message to user due to: {e}");
        }
        return;
    };

    // Simple message to use as backup if an error occured while creating the normal one
    let simple_message =  CreateMessage::new()
                .embed(
                    CreateEmbed::new()
                        .author(CreateEmbedAuthor::new("Leaguecord"))
                        .color((153, 170, 187)).title("Leaguecord, a voice chat for league")
                        .description("Hi and welcome to leaguecord.\nAn internal error occured during the wellbeing check of your account, for more information, contact an admin"),
                );

    // Small sanity check
    let Ok(player) = data
        .ids
        .guild
        .member(ctx.http.clone(), message.author.id)
        .await
    else {
        message
            .channel_id
            .send_message(ctx.http.clone(), simple_message)
            .await
            .unwrap();
        return;
    };

    let mut embed = CreateEmbed::new()
        // .author(CreateEmbedAuthor::new("Leaguecord"))
        .color((36, 219, 144))
        .title("Leaguecord, a voice chat for league")
        .description("Hi and welcome to leaguecord.\n");

    let mut fields = vec![
        ("Groups", "Groups are temporary, when your activity is done, please leave the server and create a new group with your new teammates !", false),
        ("Creating a group", "To create a group please use the website at <http://192.168.1.39:42069>.", false),
        ("Joining a group", "To join a group, simply join any Leaguecord link with an id at the end (like <http://192.168.1.39:42069/12345678901234567890>).", false),
        ("Leaving a group", "To leave a group, simply leave the server.", false),
        ("Group Permissions", "Each member of a group only has the ability to see and interact with it's own group. This way, you can enjoy a focused and private environment for your team without distractions from other groups.", false),
        ("Secure groups (TODO)", "Secure groups offer the possiblity to filter who can and cannot join your group.", false),
        ("Deleting a group", "To delete a group, simply leave the server, the group will be automatically cleaned up.", false),
        ("Help and support", "I havn't done anything specific for this (yet), so just send me a dm !", false)
    ];

    'channel_specific: {
        let Ok(Channel::Guild(channel)) = message.channel(ctx.http.clone()).await else {
            break 'channel_specific;
        };

        let Some(id) = Group::id_for_name(channel.name()) else {
            break 'channel_specific;
        };

        fields.push(("Channel specific", "This channel is a group channel, it will stay up until every member of it's group has left.", false));
    };

    embed = embed.fields(fields);

    message
        .channel_id
        .send_message(ctx.http.clone(), CreateMessage::new().embed(embed))
        .await
        .unwrap();
}
