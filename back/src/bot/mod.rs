mod command;
mod handlers;

pub async fn run_threaded(// dispatcher_sender: Option<std::sync::mpsc::Sender<(Context, FullEvent)>>,
) -> (
    std::sync::Arc<serenity::all::Http>,
    std::sync::Arc<tokio::sync::RwLock<serenity::prelude::TypeMap>>,
    tokio::task::JoinHandle<()>,
) {
    use serenity::all::{ActivityData, ApplicationId, Client, GatewayIntents};

    // Login with a bot token from the environment
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    // let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let intents = GatewayIntents::all();

    // Create a new instance of the Client, logging in as a bot.
    let cb = Client::builder(&token, intents)
        // .event_handler(handlers::Basic)
        // .event_handler(handlers::Logger)
        .event_handler(handlers::LeagueCord)
        .event_handler(handlers::PlayerHelper)
        .event_handler(handlers::Door)
        .event_handler(handlers::Debug)
        .application_id(ApplicationId::new(1338410037953040466))
        .event_handler(handlers::Purge)
        .status(serenity::all::OnlineStatus::DoNotDisturb)
        .activity(ActivityData::listening(format!(
            "{}help",
            command::DEFAULT_PREFIX
        )));

    let mut client = cb.await.unwrap();

    (
        client.http.clone(),
        client.data.clone(),
        tokio::task::spawn(async move {
            if let Err(why) = client.start().await {
                println!("Client error: {why:?}");
            }
        }),
    )
}
