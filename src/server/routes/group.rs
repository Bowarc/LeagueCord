use std::net::SocketAddr;

use crate::{data::GroupId, server::response::{Response, ResponseBuilder}};

use super::root;


#[rocket::get("/group/<id>")]
pub async fn group(id: GroupId, remote_addr: SocketAddr) -> Response{

    root(remote_addr).await
    // ResponseBuilder::default().build()
}
