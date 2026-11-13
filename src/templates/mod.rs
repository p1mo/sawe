mod extensions;
pub use extensions::*;

mod json_ld;
mod seo;
mod types;



use std::sync::Arc;

use globset::Glob;
use handlebars::Handlebars;
use parking_lot::RwLock;

use super::ufs::UnifiedFS;
use super::uiicons::EmbededIcons;





pub enum Kind {
    Embeded(UnifiedFS),
    Static(UnifiedFS, String),
}



pub struct Entry {
    inner: Kind,
}

impl Entry {
    
    pub fn new_embed(fs: UnifiedFS) -> Self {
        Self { inner: Kind::Embeded(fs) }
    }

    pub fn new_static(path: impl AsRef<std::path::Path>) -> Self {
        Self { inner: Kind::Static(UnifiedFS::new(), path.as_ref().display().to_string()) }
    }

    pub fn find(&self, input: &str) -> Result<&'static [u8], TemplatesError> {
        let matcher = Glob::new(input)?;
        match self.inner {
            Kind::Embeded(ref unified_fs) => {
                for entry in unified_fs.iter() {
                    if matcher.compile_matcher().is_match(entry.path.clone()) {
                        return Ok(entry.content()?);
                    }
                }
            },
            Kind::Static(ref unified_fs, ref path) => {
                for entry in unified_fs.walk(path) {
                    if matcher.compile_matcher().is_match(entry.path.clone()) {
                        return Ok(entry.content()?);
                    }
                }
            },
        }
        Err(TemplatesError::GlobNotFound())
    }

}



pub struct Templates<T>
where 
    T: PartialEq + Send + Sync
{
    engine: Handlebars<'static>,
    entrys: Vec<(T, Entry)>,
    icon_list: Vec<(T, EmbededIcons)>,
}

impl<T: PartialEq + Send + Sync> Templates<T> {
    
    pub fn new() -> Self {
        Self { engine: Handlebars::new(), entrys: Vec::new(), icon_list: vec![] }
    }
    
    pub fn new_with_icons(entry_key: T, icons: EmbededIcons) -> Self {
        Self { engine: Handlebars::new(), entrys: Vec::new(), icon_list: vec![(entry_key, icons)] }
    }

    pub fn to_rw(self) -> Arc<RwLock<Templates<T>>> {
        Arc::new(RwLock::new(self))
    }

    pub fn add(&mut self, entry_key: T, entry: Entry) -> Result<(), TemplatesError> {
        match self.entrys.iter().find(|i| i.0 == entry_key).is_some() {
            true => Err(TemplatesError::KeyExists()),
            false => {
                self.entrys.push((entry_key, entry));
                Ok(())
            },
        }
    }

    pub fn get_entry(&self, entry_key: T) -> Result<&Entry, TemplatesError> {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => Ok(entry),
            None => Err(TemplatesError::KeyNotFound()),
        }
    }

    pub fn find(&self, entry_key: T, search: &str) -> Result<&'static [u8], TemplatesError> {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => Ok(entry.find(search)?),
            None => Err(TemplatesError::GlobNotFound()),
        }
    }

    pub fn icon_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError> {
        match self.icon_list.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => Ok(entry.find(search).ok_or(TemplatesError::GlobNotFound())?.to_vec()),
            None => Err(TemplatesError::GlobNotFound()),
        }
    }

    pub fn icon_par_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError> {
        match self.icon_list.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => Ok(entry.par_find(search).ok_or(TemplatesError::GlobNotFound())?.to_vec()),
            None => Err(TemplatesError::GlobNotFound()),
        }
    }

    pub fn render<S>(&self, entry_key: T, search: &str, data: &S) -> Result<String, TemplatesError>
    where
        S: serde::Serialize,
    {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => {
                let res = entry.find(search)?;
                Ok(self.engine.render_template(&String::from_utf8(res.to_vec())?, data)?)
            },
            None => Err(TemplatesError::KeyNotFound()),
        }
    }

    pub fn render_to_write<S, W>(&self, entry_key: T, search: &str, data: &S, writer: W) -> Result<(), TemplatesError>
    where 
        S: serde::Serialize,
        W: std::io::Write,
    {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => {
                let res = entry.find(search)?;
                Ok(self.engine.render_template_to_write(&String::from_utf8(res.to_vec())?, data, writer)?)
            },
            None => Err(TemplatesError::KeyNotFound()),
        }
    }

    pub fn render_with_context(&self, entry_key: T, search: &str, ctx: &Context) -> Result<String, TemplatesError> {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => {
                let res = entry.find(search)?;
                Ok(self.engine.render_template_with_context(&String::from_utf8(res.to_vec())?, ctx)?)
            },
            None => Err(TemplatesError::KeyNotFound()),
        }
    }

    pub fn render_with_context_to_write< W>(&self, entry_key: T, search: &str, ctx: &Context, writer: W) -> Result<(), TemplatesError>
    where
        W: std::io::Write,
    {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => {
                let res = entry.find(search)?;
                Ok(self.engine.render_template_with_context_to_write(&String::from_utf8(res.to_vec())?, ctx, writer)?)
            },
            None => Err(TemplatesError::KeyNotFound()),
        }
    }

    pub fn register_template(&mut self, entry_key: T, search: &str, name: &str) -> Result<(), TemplatesError> {
        match self.entrys.iter().find(|i| i.0 == entry_key) {
            Some((_, entry)) => {
                let res = entry.find(search)?;
                Ok(self.engine.register_template_string(name, &String::from_utf8(res.to_vec())?)?)
            },
            None => Err(TemplatesError::GlobNotFound()),
        }
    }

    pub fn render_template(&self, name: &str, data: serde_json::Value) -> Result<String, TemplatesError> {
        Ok(self.engine.render(name, &data)?)
    }

}



pub type RwTemplates<T> = Arc<RwLock<Templates<T>>>;



#[derive(Debug, thiserror::Error)]
pub enum TemplatesError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    UFSError(#[from] ufs::Error),

    #[error(transparent)]
    HandlebarsRenderError(#[from] handlebars::RenderError),
    #[error(transparent)]
    HandlebarsTemplateError(#[from] handlebars::TemplateError),

    #[error(transparent)]
    GlobSetError(#[from] globset::Error),

    #[error("glob not found")]
    GlobNotFound(),

    #[error("key already exists")]
    KeyExists(),

    #[error("key not found")]
    KeyNotFound(),

    #[error("icon not found")]
    IconNotFound(),
}