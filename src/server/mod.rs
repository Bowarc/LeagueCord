use std::sync::Arc;

use serenity::all::Http;

use crate::data::LeagueCordData;

pub mod catchers;
pub mod error;
pub mod response;
pub mod routes;

pub async fn build_rocket(
    http: Arc<Http>,
    data: LeagueCordData,
) -> rocket::Rocket<rocket::Ignite> {
    rocket::build()
        .manage(http)
        .manage(data)
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
