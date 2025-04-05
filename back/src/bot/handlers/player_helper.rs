pub struct PlayerHelper;

#[serenity::async_trait]
impl serenity::all::EventHandler for PlayerHelper {
    async fn message(&self, ctx: serenity::all::Context, message: serenity::all::Message) {
        super::module_command(&ctx, "PlayerHelper", message.clone()).await;
        help_message(ctx, message).await;
    }
}

pub async fn help_message(ctx: serenity::all::Context, message: serenity::all::Message) {
    use {
        crate::{
            bot::command,
            data::Group,
        },
        serenity::all::{CacheHttp as _, Channel, CreateEmbed, CreateMessage},
    };

    let Some(_args) = command::parse(
        &message,
        "help",
        command::Case::Insensitive,
        command::Prefix::Yes,
    ) else {
        return;
    };

    let mut embed = CreateEmbed::new()
        .color((36, 219, 144))
        .title("Leaguecord, a voice chat for league")
        .description("Hi and welcome to leaguecord.\n");

    let mut fields = vec![
        ("Groups", "Groups are temporary, when your activity is done, please leave the server and create a new group with your new teammates !", false),
        ("Creating a group", "To create a group please use the website at <https://leaguecord.com/>.", false),
        ("Joining a group", "To join a group, simply join any Leaguecord link with an id at the end (like `https://leaguecord.com/group/12345678901234567890`).", false),
        ("Leaving a group", "To leave a group, simply leave the server.", false),
        ("Group Permissions", "Each member of a group only has the ability to see and interact with it's own group. This way, you can enjoy a focused and private environment for your team without distractions from other groups.", false),
        ("Secure groups (TODO #9)", "Secure groups offer the possiblity to filter who can and cannot join your group.", false),
        ("Deleting a group", "To delete a group, simply leave the server, the group will be automatically cleaned up.", false),
        ("Help and support", "I havn't done anything specific for this (yet), so just send me a dm !", false)
    ];

    'channel_specific: {
        let Ok(Channel::Guild(channel)) = message.channel(ctx.http()).await else {
            break 'channel_specific;
        };

        let Some(_id) = Group::id_for_name(channel.name()) else {
            break 'channel_specific;
        };

        fields.push(("Channel specific", "This channel is a group channel, it will stay up until every member of it's group has left.", false));
    };

    embed = embed.fields(fields);

    message
        .channel_id
        .send_message(ctx.http(), CreateMessage::new().embed(embed))
        .await
        .unwrap();
}
