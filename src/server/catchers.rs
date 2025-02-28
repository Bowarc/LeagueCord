use rocket::response::Redirect;

#[rocket::catch(400)]
pub fn upload_400(_req: &rocket::Request<'_>) -> super::response::Response {
    use rocket::http::{ContentType, Status};

    super::response::ResponseBuilder::default()
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
pub async fn root_404(req: &rocket::Request<'_>) -> Redirect {
    Redirect::to(rocket::uri!("/404"))
}
