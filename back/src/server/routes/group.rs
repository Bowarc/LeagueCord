#![allow(clippy::crate_in_macro_def)]

#[rocket::get("/group/<id>")]
pub async fn group(
    id: crate::data::GroupId,
    ip_addr: crate::data::IpStruct,
    lc_data: &rocket::State<crate::data::LeagueCordData>,
) -> super::super::response::Response {
    use super::super::response::Response;
    use super::root;

    if !lc_data
        .groups
        .read()
        .await
        .iter()
        .any(|group| group.id == id)
    {
        return Response::redirect("/group_not_found");
    }

    root(ip_addr).await
}

#[rocket::get("/group_data/<id>")]
pub async fn group_data(
    id: crate::data::GroupId,
    ip_addr: crate::data::IpStruct,
    lc_data: &rocket::State<crate::data::LeagueCordData>,
) -> super::super::response::Response {
    use {super::super::response::Response, rocket::http::Status};

    debug!("Request of group {id}'s data by [{ip_addr}]");

    let groups_read = lc_data.groups.read().await;
    let Some(group) = groups_read.iter().find(|g| g.id == id) else {
        error!("Could not find any info about group {id}, request from [{ip_addr}]");
        return Response::builder().with_status(Status::NotFound).build();
    };

    let object = match rocket::serde::json::serde_json::ser::to_string(&group.to_data()) {
        Ok(obj) => obj,
        Err(e) => {
            error!("Failed to serialize group data due to: {e}");
            return Response::builder()
                .with_status(Status::InternalServerError)
                .build();
        }
    };

    Response::builder()
        .with_content(object)
        .with_header("Cache-Control", "max-age=30") // Ask the browser to cache the request for 30 seconds, might help for server load
        .build()
}
