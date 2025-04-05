#[rocket::get("/create_group")]
pub async fn create_group(
    lc_data: &rocket::State<crate::data::LeagueCordData>,
    http: &rocket::State<std::sync::Arc<serenity::all::Http>>,
    ip_addr: std::net::IpAddr,
    spam_tracker: &rocket::State<crate::data::GroupCreationSpamTracker>,
) -> super::super::response::Response {
    use {
        super::super::response::Response, crate::data::Group, rocket::http::Status,
        serenity::all::CacheHttp as _, serenity::all::CreateMessage,
    };

    let ids = &lc_data.ids;
    spam_tracker.update().await;

    if let Some(group_id) = spam_tracker.has(ip_addr).await {
        if lc_data.groups.read().await.iter().any(|g| g.id == group_id) {
            return Response::builder()
                .with_content(group_id.to_string())
                .build();
        }
        warn!("Fail to short-circuit group creation due to: Could not find a group with id: {group_id}");
        spam_tracker.remove(ip_addr).await;
    }

    match Group::create_new(http.http(), ids).await {
        Ok(group) => {
            let group_id = group.id;
            let group_text_channel_id = group.text_channel.get();

            spam_tracker.register(ip_addr, group.id).await;

            // Don't forget to update lc_data
            lc_data
                .invites
                .write()
                .await
                .update(http.http(), &lc_data.ids)
                .await
                .unwrap();
            lc_data.groups.write().await.push(group);

            debug!("Created group {group_id} for [{ip_addr}]");

            if let Err(e) = lc_data
                .ids
                .bot_log_channel
                .send_message(
                    http.http(),
                    CreateMessage::new().content(format!(
                        "Created group <#{group_text_channel_id}> for [{ip_addr}]"
                    )),
                )
                .await
            {
                error!("Failed to send group creation message in bot log channel due to: {e}");
            }

            Response::builder()
                .with_content(group_id.to_string())
                .build()
        }
        Err(e) => {
            error!("Failed to create group for {ip_addr} due to: {e}");
            Response::builder()
                .with_status(Status::InternalServerError)
                .with_content("Failed to create a group".to_string())
                .build()
        }
    }
}
