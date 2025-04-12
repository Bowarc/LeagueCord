//
//   I thought about fairing with hashset of bot ips,
//   but it would just be more work, more wasted CPU cycles
// 
//   Not worth
//

async fn bot_main(
    route: std::path::PathBuf,
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use {
        crate::server::response::Response,
        rocket::http::{ContentType, Status},
    };

    info!("[{ip}] has hit the bot route {route:?}");

    Response::builder()
        .with_content("Get the fuck out")
        .with_content_type(ContentType::Text)
        .with_status(Status::ImATeapot).build()
}

#[rocket::get("/.env")]
pub async fn bot_env(
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use std::path::PathBuf;
    bot_main(PathBuf::from("/.env"), ip).await
}

#[rocket::get("/admin/<p..>")]
pub async fn bot_admin(
    p: std::path::PathBuf,
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use std::path::PathBuf;
    bot_main(PathBuf::from("/admin").join(p), ip).await
}

#[rocket::get("/wordpress/<p..>")]
pub async fn bot_wordpress(
    p: std::path::PathBuf,
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use std::path::PathBuf;
    bot_main(PathBuf::from("/wordpress").join(p), ip).await
}

#[rocket::get("/wp/<p..>")]
pub async fn bot_wp(
    p: std::path::PathBuf,
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use std::path::PathBuf;
    bot_main(PathBuf::from("/wp").join(p), ip).await
}

#[rocket::get("/wp-admin/<p..>")]
pub async fn bot_wp_admin(
    p: std::path::PathBuf,
    ip: crate::data::IpStruct,
) -> crate::server::response::Response {
    use std::path::PathBuf;
    bot_main(PathBuf::from("/wp-admin").join(p), ip).await
}


