use std::net::SocketAddr;

use rocket::{http::Status, serde::json::serde_json::json, State};

use crate::{
    data::{GroupId, LeagueCordData},
    server::response::{Response, ResponseBuilder},
};

use super::root;

#[rocket::get("/group/<id>")]
pub async fn group(id: GroupId, remote_addr: SocketAddr) -> Response {
    root(remote_addr).await
}

#[rocket::get("/group_data/<id>")]
pub async fn group_data(id: GroupId, lc_data: &State<LeagueCordData>) -> Response {
    let groups_read = lc_data.groups.read().await;
    let Some(group) = groups_read.iter().find(|g| g.id == id) else {
        println!("NOT FOUND");
        return ResponseBuilder::default()
            .with_status(Status::NotFound)
            .build();
    };

    let object = match rocket::serde::json::serde_json::ser::to_string(&group.to_data()) {
        Ok(obj) => obj,
        Err(e) => {
            error!("Failed to serialize group data due to: {e}");
            return ResponseBuilder::default()
                .with_status(Status::InternalServerError)
                .build();
        }
    };
    println!("Requested group data for group: {}", group.id);

    ResponseBuilder::default()
        .with_content(object)
        .with_header("Cache-Control", "max-age=60") // Ask the browser to cache the request for 60 seconds, might help for server load
        .build()
}
