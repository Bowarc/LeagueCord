#[rocket::get("/create_group")]
pub async fn create_group(
    lc_data: &rocket::State<crate::data::LeagueCordData>,
    http: &rocket::State<std::sync::Arc<serenity::all::Http>>,
    remote_addr: std::net::SocketAddr,
    spam_tracker: &rocket::State<crate::data::GroupCreationSpamTracker>,
) -> super::super::response::Response {
    use {
        super::super::response::Response, crate::data::Group, rocket::http::Status,
        serenity::all::CacheHttp as _,
    };

    let ids = &lc_data.ids;
    spam_tracker.update().await;

    if let Some(group_id) = spam_tracker.has(remote_addr.ip()).await {
        if let Some(group) = lc_data
            .groups
            .read()
            .await
            .iter()
            .find(|group| group.id == group_id)
        {
            return Response::builder()
                .with_content(format!("http://discord.gg/{}\n", group.invite_code))
                .build();
        }
        warn!("Fail to short-circuit group creation due to: Could not find a group with id: {group_id}");
    }

    match Group::create_new(http.http(), ids).await {
        Ok(group) => {
            let invite_code = group.invite_code.clone();
            debug!("Created group '{}' (asked by {remote_addr})", invite_code);

            spam_tracker.register(remote_addr.ip(), group.id).await;

            // Don't forget to update lc_data
            lc_data
                .invites
                .write()
                .await
                .update(http.http(), &lc_data.ids)
                .await
                .unwrap();
            lc_data.groups.write().await.push(group);

            Response::builder()
                .with_content(format!("http://discord.gg/{}\n", invite_code))
                .build()
        }
        Err(e) => {
            error!("Failed to create group for {remote_addr} due to: {e}");
            Response::builder()
                .with_status(Status::InternalServerError)
                .with_content("Failed to create a group".to_string())
                .build()
        }
    }
}
