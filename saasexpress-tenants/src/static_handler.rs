use axum::{
    body::{self, Full},
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ui/"]
struct StaticAssets;

pub(crate) async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .body(body::boxed(Full::from(content.data)))
                .unwrap()
        }
        None => {
            // If the path doesn't exist, try to serve index.html (for SPA routing)
            if let Some(content) = StaticAssets::get("index.html") {
                Response::builder()
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(body::boxed(Full::from(content.data)))
                    .unwrap()
            } else {
                // Truly not found
                (StatusCode::NOT_FOUND, "Not Found").into_response()
            }
        }
    }
}
