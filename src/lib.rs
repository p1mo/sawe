pub mod server;

#[cfg(feature = "templates")]
pub mod templates;

#[cfg(feature = "templates")]
pub use ufs;

#[cfg(feature = "templates")]
pub use uiicons;

#[cfg(all(feature = "build", feature = "templates"))]
pub use uiicons::build as build_uiicons;


pub use salvo;
pub use tokio;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),


    #[cfg(feature = "docs")]
    #[error("oapi endpoint not found.")]
    OApiNotFound,

    #[cfg(feature = "docs")]
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "docs")]
    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),

    
    #[error(transparent)]
    ServerError(#[from] server::ServerError),

    #[cfg(feature = "templates")]
    #[error(transparent)]
    TemplatesError(#[from] templates::TemplatesError),
}