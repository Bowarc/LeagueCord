pub mod catchers;
pub mod error;
pub mod response;
pub mod routes;

pub fn logger_config() -> Vec<logger::Config> {
    const FILTERS: &[(&str, log::LevelFilter)] = &[
        ("bot", log::LevelFilter::Trace),
        ("LeagueCordBot", log::LevelFilter::Trace),
        ("log_panics", log::LevelFilter::Trace),
        ("serenity", log::LevelFilter::Warn),
        ("h2", log::LevelFilter::Error),
        ("tokio", log::LevelFilter::Warn),
        ("hyper", log::LevelFilter::Warn),
        ("tungstenite", log::LevelFilter::Warn),
        ("reqwest", log::LevelFilter::Warn),
        ("rustls", log::LevelFilter::Warn),
    ];

    vec![
        logger::Config::default()
            .level(log::LevelFilter::Off)
            .output(logger::Output::Stdout)
            .colored(true)
            .filters(FILTERS),
        logger::Config::default()
            .level(log::LevelFilter::Trace)
            .output(logger::Output::new_timed_file(
                "./log/server.log",
                std::time::Duration::from_secs(3600), // an hour
            ))
            .colored(false)
            .filters(FILTERS),
    ]
}

pub async fn build_rocket() -> rocket::Rocket<rocket::Ignite> {
    rocket::build()
        .register("/", rocket::catchers![catchers::root_403])
        .register("/upload", rocket::catchers![catchers::upload_400])
        .mount(
            "/",
            rocket::routes![
                routes::root,
                routes::front_js,
                routes::front_bg_wasm,
                routes::index_html,
                routes::static_resource,
                routes::static_css,
                routes::favicon_ico,
            ],
        )
        .ignite()
        .await
        .unwrap()
}
