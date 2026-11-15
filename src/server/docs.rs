use std::{path::Path, sync::Arc};

use handlebars::{handlebars_helper, Handlebars};
use salvo::prelude::*;
use parking_lot::RwLock;


handlebars_helper!(is_http_method: |method: str| matches!(method, "get" | "post" | "put" | "patch" | "delete" | "options" | "head" | "trace"));
handlebars_helper!(json: |value: Json| serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_string()));


pub type ArcDocsStore = Arc<RwLock<DocsStore>>;

pub struct DocsStore(Handlebars<'static>, ufs::UnifiedFS, Vec<(String, OpenApi)>);

pub static DEFAULT_DOCS_FILE: &str = "documentation.hbs";

impl DocsStore {

    pub fn new() -> Self {
        let mut hbs = Handlebars::new();
        hbs.register_helper("is_http_method", Box::new(is_http_method));
        hbs.register_helper("json", Box::new(json));
        Self(hbs, ufs::bind_dir!("layouts"), Vec::new())
    }
    
    pub fn add(&mut self, path: &str, docs: OpenApi) {
        self.2.push((path.to_string(), docs));
    }

    pub fn get(&self, path: &str) -> Option<&OpenApi> {
        self.2.iter().find(|(k, _)| k.as_str() == path).map(|(_, docs)| docs)
    }

    fn find_docs(&self, path: impl AsRef<Path>) -> Option<&'static [u8]> {
        self.1.iter().find(|entry| entry.path.display().to_string().ends_with(&path.as_ref().display().to_string())).map(|item| item.content().unwrap())
    }

    pub fn docs_render<S>(&self, path: impl AsRef<Path>, data: &S) -> Result<String, DocsError>
    where
        S: serde::Serialize,
    {
        match self.find_docs(path) {
            Some(res) => {
                Ok(self.0.render_template(&String::from_utf8(res.to_vec())?, data)?)
            },
            None => Err(DocsError::OApiNotFound),
        }
    }

}



pub trait DocsDepotExt {
    fn docs_find(&self, docs: &str) -> Result<OpenApi, DocsError>;
    fn docs_render(&self, docs: &str, template: &str) -> Result<String, DocsError>;
}

impl DocsDepotExt for Depot {

    fn docs_find(&self, docs: &str) -> Result<OpenApi, DocsError> {
        let guard = self.get_typed::<super::ArcDocsStore>().unwrap().read();
        let docs = guard.get(docs).ok_or(DocsError::OApiNotFound)?.clone();
        drop(guard);
        Ok(docs)
    }

    fn docs_render(&self, docs: &str, template: &str) -> Result<String, DocsError> {
        let guard = self.get_typed::<super::ArcDocsStore>().unwrap().read();
        let docs = guard.get(docs).ok_or(DocsError::OApiNotFound)?;
        let string = guard.docs_render(template, docs)?;
        drop(guard);
        Ok(string)
    }

}


#[derive(Debug, thiserror::Error)]
pub enum DocsError {
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error("oapi endpoint not found.")]
    OApiNotFound,

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),
}