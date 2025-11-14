use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use tracing::error;

macro_rules! render {
    ($template:path) => {{
        use $crate::http::render::Render;
        Render(|o| $template(o))
    }};
    ($template:path, $($arg:expr),* $(,)*) => {{
        use $crate::http::render::Render;
        Render(move |o| $template(o, $($arg),*))
    }}
}

pub(crate) struct Render<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>>(pub T);

impl<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>> IntoResponse for Render<T> {
    fn into_response(self) -> axum::response::Response {
        let mut buf = Vec::new();
        match self.0(&mut buf) {
            Ok(()) => Html(buf).into_response(),
            Err(err) => {
                error!(%err, "error rendering template");
                (StatusCode::INTERNAL_SERVER_ERROR, "Render failed").into_response()
            }
        }
    }
}
