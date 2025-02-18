#[macro_use(trace, debug, info, warn, error)]
extern crate log;

mod bot;
mod server;

fn setup_loggers() {
    use log::LevelFilter;
    use std::time::Duration;

    const DEP_FILTERS: &[(&str, LevelFilter)] = &[
        ("log_panics", LevelFilter::Trace),
        ("serenity", LevelFilter::Warn),
        ("h2", LevelFilter::Error),
        ("tokio", LevelFilter::Warn),
        ("hyper", LevelFilter::Warn),
        ("tungstenite", LevelFilter::Warn),
        ("reqwest", LevelFilter::Warn),
        ("rustls", LevelFilter::Warn),
        ("rocket", LevelFilter::Warn),
    ];

    logger::init([
        logger::Config::default()
            .output(logger::Output::Stdout)
            .colored(true)
            .level(LevelFilter::Trace)
            .filters(DEP_FILTERS),
        logger::Config::default()
            .output(logger::Output::new_timed_file(
                "./log/server.log",
                Duration::from_secs(3600), // 1h
            ))
            .colored(false)
            .level(log::LevelFilter::Off) // Disable everything by default to make it server-specific
            .filters(DEP_FILTERS)
            .filter("server", LevelFilter::Trace),
        logger::Config::default()
            .output(logger::Output::new_timed_file(
                "./log/bot.log",
                Duration::from_secs(3600), // 1h
            ))
            .colored(false)
            .level(log::LevelFilter::Off) // Disable everything by default to make it bot-specific
            .filters(DEP_FILTERS)
            .filter("bot", LevelFilter::Trace),
    ])
}

// Needed for tests

#[rocket::main]
async fn main() {
    setup_loggers();

    dotenv::dotenv().unwrap();

    bot::run_threaded().await;

    // Small print to show the start of the program log in the file
    trace!(
        "\n╭{line}╮\n│{message:^30}│\n╰{line}╯",
        line = "─".repeat(30),
        message = "Program start"
    );

    let rocket = server::build_rocket().await;

    display_config(rocket.config(), rocket.routes(), rocket.catchers());

    rocket.launch().await.unwrap();
}

/// Displays the config in the console
fn display_config<'a>(
    rocket_cfg: &rocket::Config,
    rocket_routes: impl Iterator<Item = &'a rocket::Route>,
    rocket_catchers: impl Iterator<Item = &'a rocket::Catcher>,
) {
    let profile = rocket_cfg.profile.as_str().as_str();
    let address = rocket_cfg.address;
    let port = rocket_cfg.port;
    let workers = rocket_cfg.workers;
    // let max_blocking = cfg.max_blocking;
    let indent = rocket_cfg.ident.as_str().unwrap_or("[ERROR] Undefined");
    let ip_headers = rocket_cfg
        .ip_header
        .as_ref()
        .map(|header| header.as_str())
        .unwrap_or("[ERROR] Undefined");
    let limits = ["bytes", "data-form", "file", "json", "msgpack", "string"]
        .iter()
        .map(|limit_name| {
            format!(
                "{limit_name}: {}",
                rocket_cfg
                    .limits
                    .get(limit_name)
                    .unwrap_or(rocket::data::ByteUnit::from(0))
            )
        })
        .collect::<Vec<String>>();
    let keep_alive_s = rocket_cfg.keep_alive;
    let shutdown_mode = &rocket_cfg.shutdown;

    let routes = rocket_routes
        .map(|route| {
            let uri = route.uri.origin.to_string();
            let name = route
                .name
                .as_ref()
                .map(std::borrow::Cow::as_ref)
                .unwrap_or("[ERROR] Undefined");
            let method = route.method.as_str();
            format!("{method:<5} {uri:<20} {name}")
        })
        .collect::<Vec<String>>();

    let catchers = rocket_catchers
        .map(|catcher| {
            let base = catcher.base.to_string();
            let name = catcher
                .name
                .as_ref()
                .map(std::borrow::Cow::as_ref)
                .unwrap_or("[ERROR] Undefined");
            let code = catcher
                .code
                .map(|code| code.to_string())
                .unwrap_or("[ERROR] Undefined".to_string());

            format!("{code:<5} {base:<20} {name}")
        })
        .collect::<Vec<String>>();

    let display_vec = |data: Vec<String>| -> String {
        use std::fmt::Write as _;
        let mut out = String::new();
        out.push_str("[\n");
        out.push_str(&data.iter().fold(String::new(), |mut output, d| {
            writeln!(output, "    {d}").unwrap(); // Writing to a string cannot fail
            output
        }));
        out.push(']');
        out
    };

    info!("\nConfig:\nUsing profile: {profile}\nAddress: {address}:{port}\nWorkers: {workers}\nIndent: {indent}\nHeaders: {ip_headers}\nLimits: {formatted_limits}\nConnection lifetime: {keep_alive_s}s\nShutdown mode: {shutdown_mode}\nRoutes: {formatted_routes}\nCatchers: {formatted_catchers}",
        formatted_limits = display_vec(limits),
        formatted_routes = display_vec(routes),
        formatted_catchers = display_vec(catchers)
    );
}
