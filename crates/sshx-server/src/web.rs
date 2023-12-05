//! HTTP and WebSocket handlers for the sshx web interface.

use std::sync::Arc;

use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

use tracing::error;

use crate::{ServerState, ServerOptions};

pub mod protocol;
mod socket;

/// Returns the web application server, routed with Axum.
pub fn app(options: ServerOptions) -> Router<Arc<ServerState>> {

    // Serves static SvelteKit build files.
    let web_dir_path = options.web_dir.unwrap_or_default();
    if !web_dir_path.is_dir() {
         error!("failed to serve {}, directory does not exist", web_dir_path.display());
    }
    let spa_file = ServeFile::new(web_dir_path.join("/spa.html"))
        .precompressed_gzip()
        .precompressed_br();
    let static_files = ServeDir::new(web_dir_path)
        .precompressed_gzip()
        .precompressed_br()
        .fallback(spa_file);

    Router::new()
        .nest("/api", backend())
        .fallback_service(get_service(static_files))
}

/// Routes for the backend web API server.
fn backend() -> Router<Arc<ServerState>> {
    Router::new().route("/s/:name", get(socket::get_session_ws))
}
