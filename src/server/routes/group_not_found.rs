#[rocket::get("/group_not_found")]
pub async fn group_not_found(
    remote_addr: std::net::SocketAddr,
) -> super::super::response::Response {
    use super::root;

    root(remote_addr).await
}
