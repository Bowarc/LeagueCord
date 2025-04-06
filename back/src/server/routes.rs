#[path = "routes/bots.rs"]
mod bot_routes;
#[path = "routes/create_group.rs"]
mod create_group_route;
#[path = "routes/group_not_found.rs"]
mod group_not_found_route;
#[path = "routes/group.rs"]
mod group_route;

pub use bot_routes::{bot_admin, bot_env, bot_wp, bot_wp_admin, bot_wordpress};
pub use create_group_route::create_group;
pub use group_not_found_route::group_not_found;
pub use group_route::{group, group_data};

#[rocket::get("/404")]
pub async fn notfound(ip_addr: std::net::IpAddr) -> super::response::Response {
    root(ip_addr).await
}

#[rocket::get("/")]
pub async fn root(ip_addr: std::net::IpAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("index.html", ContentType::HTML, ip_addr).await
}

// #[rocket::get("/test")]
// pub async fn test(req: rocket::Request) -> super::response::Response{
//     use super::response::Response;
//     use rocket::http::Status;

//     Response::builder().with_status(Status::Ok).with_content("Hi :3")
// }

#[rocket::get("/front.js")]
pub async fn front_js(ip_addr: std::net::IpAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/front.js", ContentType::JavaScript, ip_addr).await
}

#[rocket::get("/front_bg.wasm")]
pub async fn front_bg_wasm(ip_addr: std::net::IpAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/front_bg.wasm", ContentType::WASM, ip_addr).await
}

#[rocket::get("/index.html")]
pub async fn index_html(ip_addr: std::net::IpAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("/index.html", ContentType::HTML, ip_addr).await
}

#[rocket::get("/favicon.ico")]
pub async fn favicon_ico(ip_addr: std::net::IpAddr) -> super::response::Response {
    use rocket::http::ContentType;

    static_file_response("favicon.ico", ContentType::Icon, ip_addr).await
}

// The goal of this method, is to not use FileServer (because i wanna make sure of what file i serve)
// but i can't do #[rocket::get("/<file>")] as i want to use the get root path for the download api
#[rocket::get("/resources/<file>")]
pub async fn static_resource(file: &str, ip_addr: std::net::IpAddr) -> super::response::Response {
    use super::response::Response;
    use rocket::http::Status;

    #[rustfmt::skip]
    const ALLOWED_FILES: &[&str] = &[
        "github.webp"
    ];

    if !ALLOWED_FILES.contains(&file) {
        return Response::builder().with_status(Status::NotFound).build();
    }

    serve_static("/resources", file, ip_addr).await
}

#[rocket::get("/css/<file>")]
pub async fn static_css(file: &str, ip_addr: std::net::IpAddr) -> super::response::Response {
    use crate::server::response::Response;
    use rocket::http::Status;

    const ALLOWED_FILES: &[&str] = &[
        "contact.css",
        "home.css",
        "notification.css",
        "style.css",
        "theme.css",
    ];

    if !ALLOWED_FILES.contains(&file) {
        return Response::builder().with_status(Status::NotFound).build();
    }

    serve_static("/css", file, ip_addr).await
}

pub async fn serve_static(
    path: &str,
    file: &str,
    ip_addr: std::net::IpAddr,
) -> super::response::Response {
    use rocket::http::ContentType;

    #[inline]
    fn ext(file_name: &str) -> Option<&str> {
        if !file_name.contains(".") {
            return None;
        }

        let dot_index = file_name.rfind(".").unwrap();

        Some(&file_name[(dot_index + 1)..file_name.len()])
    }

    let content_type = ext(file)
        .and_then(ContentType::from_extension)
        .unwrap_or_else(|| {
            error!("Could not infer content type of file: {file}, requested in {path}");
            ContentType::Any
        });

    info!("Serving {path}/{file} w/ type: {content_type:?}");

    static_file_response(&format!("{path}/{file}"), content_type, ip_addr).await
}

async fn static_file_response(
    path: &str,
    content_type: rocket::http::ContentType,
    ip_addr: std::net::IpAddr,
) -> super::response::Response {
    use crate::server::response::Response;
    use rocket::http::Status;
    use tokio::io::AsyncReadExt as _;

    async fn read_static(path: &str, ip_addr: std::net::IpAddr) -> Option<Vec<u8>> {
        let mut buffer = Vec::new();

        let size = rocket::tokio::fs::File::open(format!("./static/{path}"))
            .await
            .ok()?
            .read_to_end(&mut buffer)
            .await
            .ok()?;

        trace!("Static file query from {ip_addr}: {path} ({size} bytes)");
        Some(buffer)
    }

    // here we could maybe use streaming
    match read_static(path, ip_addr).await {
        Some(bytes) => Response::builder()
            .with_status(Status::Ok)
            .with_content(bytes)
            .with_content_type(content_type)
            .build(),
        None => Response::builder().with_status(Status::NotFound).build(),
    }
}
