#[rocket::catch(404)]
pub async fn root_404(req: &rocket::Request<'_>) -> super::response::Response {
    use crate::server::response::Response;
    use rocket::{outcome::Outcome, request::FromRequest as _};

    let addr_string = if let Outcome::Success(addr) = crate::data::IpStruct::from_request(req).await
    {
        addr.get_ipv4_string()
            .unwrap_or_else(|| addr.get_ipv6_string())
    } else {
        "UNKNOWN ADDRESS".to_string()
    };

    warn!(
        "[{addr_string}] has hit a 404 with {} at {} {}",
        req.method(),
        req.uri(),
        req.content_type()
            .map(|t| format!("({t})"))
            .unwrap_or_default()
    );
    Response::redirect("/404")
}
