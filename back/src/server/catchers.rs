#[rocket::catch(404)]
pub async fn root_404(_req: &rocket::Request<'_>) -> super::response::Response {
    use crate::server::response::Response;
    Response::redirect("/404")
}
