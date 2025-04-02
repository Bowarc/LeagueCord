#[rocket::catch(404)]
pub async fn root_404(req: &rocket::Request<'_>) -> super::response::Response {
    use crate::server::response::Response;
    warn!(
        "[{}] has hit a 404 with {} at {} ({})",
        req.real_ip()
            .map(|ip| ip.to_string())
            .or_else(|| req.remote().map(|remote| remote.to_string()))
            .unwrap_or_else(|| "UNKNOWN ADDR".to_string()),
        req.method(),
        req.uri(),
        req.content_type().map(|t| t.to_string()).unwrap_or_default()
    );
    Response::redirect("/404")
}
