#[rocket::catch(400)]
pub fn upload_400(_req: &rocket::Request<'_>) -> super::response::Response {
    use crate::server::response::Response;
    use rocket::http::{ContentType, Status};

    Response::builder()
        .with_status(Status::PayloadTooLarge)
        .with_content("Could not understand the given data.")
        .with_content_type(ContentType::Text)
        .build()
}

#[rocket::catch(403)]
pub fn root_403() -> String {
    "403".to_string()
}

#[rocket::catch(404)]
pub async fn root_404(req: &rocket::Request<'_>) -> super::response::Response {
    use crate::server::response::Response;
    // Redirect::to(rocket::uri!("/404"))
    Response::redirect("/404")
}
