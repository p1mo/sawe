use crate::Directory;

use std::sync::{Arc, RwLock};



pub use comrak::Options;
pub use comrak::options::Plugins;

use comrak::markdown_to_html;
use comrak::markdown_to_html_with_plugins;

pub use handlebars::Handlebars;

use globset::Glob;
use salvo::prelude::*;


pub struct Markdown<T> {
    options: Options<'static>,
    plugins: Plugins<'static>,

    engine: Handlebars<'static>,
    regist: Vec<(T, Vec<(String, String)>)>,
}



impl<T> Markdown<T> {
    
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            regist: Vec::new(),
            engine: Handlebars::new(),
            options: Options::default(),
            plugins: Plugins::default(),
        }))
    }

    pub fn len(&self) -> usize {
        self.regist.len()
    }

    pub fn is_empty(&self) -> bool {
        self.regist.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.regist.iter().map(|(key, _)| key)
    }

    pub fn set_options(&mut self, options: Options<'static>) {
        self.options = options;
    }

    pub fn set_plugins(&mut self, plugins: Plugins<'static>) {
        self.plugins = plugins;
    }

}



impl<T: PartialEq + Send + Sync + 'static> Markdown<T> {

    pub fn contains(&self, key: &T) -> bool {
        self.regist.iter().find(|(k, _)| *k == *key).is_some()
    }

    pub fn index_of(&self, key: T) -> Option<usize> {
        self.regist.iter().enumerate().find(|(_, (k, _))| *k == key).map(|(i, _)| i)
    }
    
    pub fn add(&mut self, key: T, bind: Directory) -> Result<(), MarkdownError> {
        if self.contains(&key) {
            return Err(MarkdownError::KeyAlreadyRegistred);
        }
        let mut reg = Vec::new();
        for entry in bind.iter() {
            let data = entry.bytes()?.unwrap();
            let path = entry.path.display().to_string().replace("\\", "/");
            tracing::debug!("parsing md document [{}] size: {} bytes", &path, &data.len());
            reg.push((path, markdown_to_html(&String::from_utf8(data.to_vec())?, &self.options)));
        }
        self.regist.push((key, reg));
        Ok(())
    }
    
    pub fn add_with_plugins(&mut self, key: T, bind: Directory) -> Result<(), MarkdownError> {
        if self.contains(&key) {
            return Err(MarkdownError::KeyAlreadyRegistred);
        }
        let mut reg = Vec::new();
        for entry in bind.iter() {
            let data = entry.bytes()?.unwrap();
            let path = entry.path.display().to_string().replace("\\", "/");
            tracing::debug!("parsing md document [{}] size: {} bytes", &path, &data.len());
            reg.push((path, markdown_to_html_with_plugins(&String::from_utf8(data.to_vec())?, &self.options, &self.plugins)));
        }
        self.regist.push((key, reg));
        Ok(())
    }
    
    fn get(&self, key: T, search: &str) -> Result<&String, MarkdownError> {        
        tracing::debug!("looking for [{}]", search);
        let matcher = Glob::new(search)?.compile_matcher();
        Ok(
            self
                .regist
                .iter()
                .find(|(k, _)| *k == key)
                .map(|(_, d)| d)
                .map(|items| {
                    items
                        .iter()
                        .find(|(p, _)| matcher.is_match(p))
                        .map(|(_, n)| n)
                        .ok_or(MarkdownError::FileNotFound)
                })
                .ok_or(MarkdownError::KeyNotFound)??
        )
    }

}



pub trait MarkdownDepot<T> {
    fn md_find(&self, entry_key: T, search: &str) -> Result<String, MarkdownError>;
    fn md_render<S>(&self, entry_key: T, search: &str, data: &S) -> Result<String, MarkdownError>
    where 
        S: serde::Serialize;
}

impl<T: PartialEq + Send + Sync + 'static> MarkdownDepot<T> for Depot {

    fn md_find(&self, entry_key: T, search: &str) -> Result<String, MarkdownError> {
        let guard = self.get_typed::<Arc<RwLock<Markdown<T>>>>().unwrap().read().unwrap();
        let content = guard.get(entry_key, search)?.to_string();
        drop(guard);
        Ok(content)
    }

    fn md_render<S>(&self, entry_key: T, search: &str, data: &S) -> Result<String, MarkdownError>
    where 
        S: serde::Serialize,
    {
        let guard = self.get_typed::<Arc<RwLock<Markdown<T>>>>().unwrap().read().unwrap();
        let content = guard.engine.render_template(&guard.get(entry_key, search)?.to_string(), data)?;
        drop(guard);
        Ok(content)
    }

}





#[derive(Debug, thiserror::Error)]
pub enum MarkdownError {
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    GlobsetError(#[from] globset::Error),

    #[error(transparent)]
    RenderError(#[from] handlebars::RenderError),

    #[error("failed to get typed depot.")]
    FailedToGetDepot,
    #[error("key is already registred.")]
    KeyAlreadyRegistred,
    #[error("key not found.")]
    KeyNotFound,
    #[error("file not found.")]
    FileNotFound,
}