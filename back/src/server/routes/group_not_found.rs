#[rocket::get("/group_not_found")]
pub async fn group_not_found(ip_addr: std::net::IpAddr) -> super::super::response::Response {
    use super::root;

    root(ip_addr).await
}
