pub mod catchers;
pub mod error;
pub mod response;
pub mod routes;

#[rocket::get("/404")]
async fn notfound(remote_addr: std::net::SocketAddr) -> response::Response {
    routes::root(remote_addr).await
}

pub async fn build_rocket(
    http: std::sync::Arc<serenity::all::Http>,
    data: crate::data::LeagueCordData,
) -> rocket::Rocket<rocket::Ignite> {
    use crate::data::GroupCreationSpamTracker;

    rocket::build()
        .manage(http)
        .manage(data)
        .manage(GroupCreationSpamTracker::default())
        .register(
            "/",
            rocket::catchers![catchers::root_403, catchers::root_404],
        )
        .mount(
            "/",
            rocket::routes![
                routes::root,
                notfound,
                routes::create_group,
                routes::group,
                routes::group_data,
                routes::group_not_found,
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
