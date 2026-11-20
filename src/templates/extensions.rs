use super::{Templates, TemplatesError};

pub use handlebars::Context;

use std::sync::{Arc, RwLock};

use salvo::prelude::*;



pub trait StringExt {
    fn to_html(self) -> Text<String>;
}

impl StringExt for String {
    fn to_html(self) -> Text<String> {
        Text::Html(self)
    }
}





pub trait TemplatesDepotExt<T>
where 
    T: PartialEq + Send + Sync
{
    
    fn icons_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError>;
    
    fn icons_par_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError>;
    
    fn tpl_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError>;
    
    fn tpl_render<S>(&self, entry_key: T, search: &str, data: &S) -> Result<String, TemplatesError>
    where
        S: serde::Serialize;
    
    fn tpl_render_to_write<S, W>(&self, entry_key: T, search: &str, data: &S, writer: W) -> Result<(), TemplatesError>
    where
        S: serde::Serialize,
        W: std::io::Write;
    
    fn tpl_render_with_context(&self, entry_key: T, search: &str, ctx: &Context) -> Result<String, TemplatesError>;
    
    fn tpl_render_with_context_to_write<W>(&self, entry_key: T, search: &str, ctx: &Context, writer: W) -> Result<(), TemplatesError>
    where
        W: std::io::Write;
    
}

impl<T: PartialEq + Send + Sync + 'static> TemplatesDepotExt<T> for Depot {
    
    fn icons_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError> {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        let content = guard.icon_find(entry_key, search)?;
        drop(guard);
        Ok(content)
    }
    
    fn icons_par_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError> {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        let content = guard.icon_par_find(entry_key, search)?;
        drop(guard);
        Ok(content)
    }
    
    fn tpl_find(&self, entry_key: T, search: &str) -> Result<Vec<u8>, TemplatesError> {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        let content = guard.find(entry_key, search)?.ok_or(super::TemplatesError::GlobNotFound())?;
        drop(guard);
        Ok(content.to_vec())
    }

    fn tpl_render<S>(&self, entry_key: T, search: &str, data: &S) -> Result<String, TemplatesError>
    where
        S: serde::Serialize
    {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        let string = guard.render(entry_key, search, data)?;
        drop(guard);
        Ok(string)
    }
    
    fn tpl_render_to_write<S, W>(&self, entry_key: T, search: &str, data: &S, writer: W) -> Result<(), TemplatesError>
    where
        S: serde::Serialize,
        W: std::io::Write
    {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        guard.render_to_write(entry_key, search, data, writer)?;
        drop(guard);
        Ok(())
    }
    
    fn tpl_render_with_context(&self, entry_key: T, search: &str, ctx: &Context) -> Result<String, TemplatesError> {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        let string = guard.render_with_context(entry_key, search, ctx)?;
        drop(guard);
        Ok(string)
    }
    
    fn tpl_render_with_context_to_write<W>(&self, entry_key: T, search: &str, ctx: &Context, writer: W) -> Result<(), TemplatesError>
    where
        W: std::io::Write
    {
        let guard = self.get_typed::<Arc<RwLock<Templates<T>>>>().unwrap().read().unwrap();
        guard.render_with_context_to_write(entry_key, search, ctx, writer)?;
        drop(guard);
        Ok(())
    }

}