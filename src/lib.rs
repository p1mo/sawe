pub extern crate salvo;

pub mod server;
pub use server::*;

#[cfg(any(feature = "db", feature = "db-pg", feature = "db-sqlite"))]
pub mod database;
#[cfg(any(feature = "db", feature = "db-pg", feature = "db-sqlite"))]
pub use database::*;

#[cfg(feature = "seo")]
pub mod seo;
#[cfg(feature = "seo")]
pub use seo::*;

#[cfg(feature = "templates")]
pub mod templates;
#[cfg(feature = "templates")]
pub use templates::*;
#[cfg(feature = "templates")]
pub use handlebars;

#[cfg(feature = "templates-build")]
pub use uiicons::build as build_uiicons;



//pub use salvo;
//pub use salvo::*;

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
    
    #[error(transparent)]
    ServerError(#[from] server::ServerError),

    #[cfg(feature = "templates")]
    #[error(transparent)]
    TemplatesError(#[from] templates::TemplatesError),
}