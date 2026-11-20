
use std::sync::{Arc, RwLock};

use handlebars::Handlebars;
use handlebars::handlebars_helper;

use crate::{ Directory, embed_dir };

use globset::Glob;
use salvo::prelude::*;



handlebars_helper!(is_http_method: |method: str| matches!(method, "get" | "post" | "put" | "patch" | "delete" | "options" | "head" | "trace"));
handlebars_helper!(json: |value: Json| serde_json::to_string_pretty(value).unwrap_or_else(|_| "null".to_string()));


pub type ArcDocsStore = Arc<RwLock<DocsStore>>;

pub struct DocsStore(Handlebars<'static>, Directory, Vec<(String, OpenApi)>);

pub static DEFAULT_DOCS_FILE: &str = "**/docs_content.hbs";

impl DocsStore {

    pub fn new() -> Self {
        let mut hbs = Handlebars::new();
        hbs.register_helper("is_http_method", Box::new(is_http_method));
        hbs.register_helper("json", Box::new(json));
        Self(hbs, embed_dir!("layouts", true, crate::Compression::Fast), Vec::new())
    }
    
    pub fn add(&mut self, schema: &str, docs: OpenApi) {
        self.2.push((schema.to_string(), docs));
    }

    pub fn get(&self, schema: &str) -> Option<&OpenApi> {
        self.2.iter().find(|(k, _)| k.as_str() == schema).map(|(_, docs)| docs)
    }

    fn find_docs(&self, search: &str) -> Result<Option<Arc<[u8]>>, DocsError> {
        let matcher = Glob::new(search)?.compile_matcher();
        for entry in self.1.iter() {
            if matcher.is_match(entry.path.display().to_string().replace("\\", "/")) {
                return Ok(entry.bytes()?);
            }
        }
        Err(DocsError::DocsLayoutNotFound(search.into()))
    }

    pub fn docs_render<S>(&self, glob: &str, data: &S) -> Result<String, DocsError>
    where
        S: serde::Serialize,
    {
        let content = self.find_docs(glob)?;
        Ok(self.0.render_template(&String::from_utf8(content.unwrap().to_vec())?, data)?)
    }

}



pub trait DocsDepotExt {
    fn docs_find(&self, schema: &str) -> Result<OpenApi, DocsError>;
    fn docs_render(&self, schema: &str, template: &str) -> Result<String, DocsError>;
}

impl DocsDepotExt for Depot {

    fn docs_find(&self, schema: &str) -> Result<OpenApi, DocsError> {
        let guard = self.get_typed::<super::ArcDocsStore>().unwrap().read().unwrap();
        let docs = guard.get(schema).ok_or(DocsError::OApiNotFound)?.clone();
        drop(guard);
        Ok(docs)
    }

    fn docs_render(&self, schema: &str, template: &str) -> Result<String, DocsError> {
        let guard = self.get_typed::<super::ArcDocsStore>().unwrap().read().unwrap();
        let docs = guard.get(schema).ok_or(DocsError::OApiNotFound)?;
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
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),

    #[error(transparent)]
    GlobsetError(#[from] globset::Error),


    #[error("oapi endpoint not found.")]
    OApiNotFound,

    #[error("docs layout [{0}] not found.")]
    DocsLayoutNotFound(String),
}