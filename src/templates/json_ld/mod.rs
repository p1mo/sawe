mod properties;
pub use properties::*;

mod type_dir;
pub use type_dir::*;

#[cfg(feature = "google-type-dir")]
mod type_gdir;
#[cfg(feature = "google-type-dir")]
pub use type_gdir::*;


use crate::Error;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct JsonLD {
    /// Description: A unique ID (usually a URL plus #fragment) so other nodes can reference this entity
    /// Example: https://schema.org
    #[serde(rename = "@context")]
    pub context : String,
    /// Description: An array of several connected entities in one block
    /// Example: vec![GraphEntity]
    #[serde(rename = "@graph")]
    pub graph : Vec<GraphEntity>,
    /// Description: A unique ID (usually a URL plus #fragment) so other nodes can reference this entity
    /// Example: https://www.example.com/#organization
    #[serde(rename = "@id")]
    pub id : String,
    /// Description: The type of thing (Article, Product, ...). Can be an array: ["Product","Book"]
    /// Example: TypeDirectory::General(General::Thing)
    #[serde(rename = "@type")]
    pub typedir : TypeDirectory,
}

impl Default for JsonLD {
    fn default() -> Self {
        Self {
            context: "https://schema.org".to_string(),
            graph: vec![],
            id: "http://example.com/#default".to_string(),
            typedir: TypeDirectory::default(),
        }
    }
}

impl JsonLD {

    pub fn new_graph(graph: Vec<GraphEntity>, id : &str, typedir : TypeDirectory) -> Self {
        Self {
            graph,
            typedir,
            id: id.to_string(),
            ..Default::default()
        }
    }

    pub fn set_grapth(&mut self, graph: Vec<GraphEntity>) {
        self.graph = graph;
    }

    pub fn push_grapth(&mut self, graph: GraphEntity) {
        self.graph.push(graph);
    }

    pub fn to_string(&self, script_tag: bool) -> Result<String, Error> {
        Ok(match script_tag {
            false => serde_json::to_string(self)?,
            true => format!(r#"<script type="application/ld+json">{}</script>"#, serde_json::to_string(self)?),
        })
    }

    pub fn to_string_pretty(&self, script_tag: bool) -> Result<String, Error> {
        Ok(match script_tag {
            false => serde_json::to_string_pretty(self)?,
            true => format!(r#"<script type="application/ld+json">{}</script>"#, serde_json::to_string_pretty(self)?),
        })
    }

}