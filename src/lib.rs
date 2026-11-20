pub extern crate salvo;

pub mod server;
pub use server::*;



#[cfg(feature = "db")]
pub mod database;
#[cfg(feature = "db")]
pub use database::*;



#[cfg(feature = "fsread")]
pub mod fsread;
#[cfg(feature = "fsread")]
pub use fsread::*;



#[cfg(feature = "md")]
pub mod markdown;
#[cfg(feature = "md")]
pub use markdown::*;



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
    ServerError(#[from] ServerError),
    
    #[cfg(feature = "db")]
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[cfg(feature = "md")]
    #[error(transparent)]
    MarkdownError(#[from] MarkdownError),
    
    #[cfg(feature = "seo")]
    #[error(transparent)]
    SeoError(#[from] SeoError),
    
    #[cfg(feature = "docs")]
    #[error(transparent)]
    DocsError(#[from] DocsError),
    
    #[cfg(feature = "templates")]
    #[error(transparent)]
    TemplatesError(#[from] TemplatesError),
}