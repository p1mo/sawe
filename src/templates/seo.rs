use std::sync::Arc;

use parking_lot::RwLock;

use crate::templates::json_ld::GraphEntity;

use super::json_ld::JsonLD;


#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct BaseEntity {
    /// Description: Title of your Website
    /// Example: My Awesome Website
    /// Generates: <title>YOUR_VALUE</title>
    title: Option<String>,
    /// Description: Write a clear description of your website, company, services, products and what makes you different.
    /// Example: Some clear description about my Website
    /// Generates: <meta name="description" content="YOUR_VALUE">
    description: Option<String>,
    /// Description: Page title
    /// Example: "index, follow"
    /// Generates: <meta name="author" content="YOUR_VALUE">
    author: Option<String>,
    /// Description: Page title
    /// Example: "index, follow, max-image-preview:large"
    /// Generates: <meta name="robots" content="YOUR_VALUE">
    robots: Option<String>,
    /// Description: Page title
    /// Example: "index, follow"
    /// Generates: <meta name="googlebot" content="YOUR_VALUE">
    googlebot: Option<String>,
    /// Description: Page title
    /// Example: Your Website Name
    /// Generates: <meta name="application-name" content="YOUR_VALUE">
    application_name: Option<String>,
    /// Description: Page title
    /// Example: #FFFFFF
    /// Generates: <meta name="theme-color" content="YOUR_VALUE">
    theme_color: Option<String>,
    /// Description: Page title
    /// Example: https://www.example.com/
    /// Generates: <link rel="canonical" href="YOUR_VALUE">
    canonical: Option<String>,
    /// Description: Page title
    /// Example: ("image/png", "/favicon.png")
    /// Generates: <link rel="icon" type="YOUR_MIME" href="YOUR_VALUE">
    icon: Option<(String, String)>,
    /// Description: Page title
    /// Example: /apple-touch-icon.png
    /// Generates: <link rel="apple-touch-icon" href="YOUR_VALUE">
    apple_touch_icon: Option<String>,
    /// Description: Page title
    /// Example: /site.webmanifest
    /// Generates: <link rel="manifest" href="YOUR_VALUE">
    manifest: Option<String>,
}

impl BaseEntity {

    pub fn to_html(&self) -> String {
        let mut html = String::new();

        if let Some(ref value) = self.title {
            html.push_str(format!(r#"<title>{}</title>"#, value).as_str());
        }

        if let Some(ref value) = self.description {
            html.push_str(format!(r#"<meta name="description" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.author {
            html.push_str(format!(r#"<meta name="author" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.robots {
            html.push_str(format!(r#"<meta name="robots" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.googlebot {
            html.push_str(format!(r#"<meta name="googlebot" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.application_name {
            html.push_str(format!(r#"<meta name="application-name" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.theme_color {
            html.push_str(format!(r#"<meta name="theme-color" content="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.canonical {
            html.push_str(format!(r#"<link rel="canonical" href="{}">"#, value).as_str());
        }

        if let Some((ref mime, ref value)) = self.icon {
            html.push_str(format!(r#"<link rel="icon" type="{}" href="{}">"#, mime, value).as_str());
        }

        if let Some(ref value) = self.apple_touch_icon {
            html.push_str(format!(r#"<link rel="apple-touch-icon" href="{}">"#, value).as_str());
        }

        if let Some(ref value) = self.manifest {
            html.push_str(format!(r#"<link rel="manifest" href="{}">"#, value).as_str());
        }

        html
    }
    
}



#[derive(Default)]
struct BaseMeta {
    base: Option<BaseEntity>,
    open_graph: Option<OpenGraphEntity>,
    twitter: Option<TwitterEntity>
}

impl BaseMeta {
    
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        if let Some(ref entity) = self.base {
            html.push_str(&entity.to_html());
        }

        if let Some(ref entity) = self.open_graph {
            html.push_str(&entity.to_html());
        }

        if let Some(ref entity) = self.twitter {
            html.push_str(&entity.to_html());
        }

        html
    }

}


pub struct Seo {
    bing_token: Option<String>,
    google_token: Option<String>,

    json: Arc<RwLock<JsonLD>>,
    meta: Arc<RwLock<BaseMeta>>,
}

impl Seo {
    
    pub fn new(bing_token: Option<String>, google_token: Option<String>) -> Self {
        Self {
            bing_token,
            google_token,

            json: Arc::new(RwLock::new(JsonLD::default())),
            meta: Arc::new(RwLock::new(BaseMeta::default())),
        }
    }

    pub fn bing_verification(&self) -> Option<String> {
        if let Some(ref token) = self.bing_token {
            return Some(format!(r#"<meta name="msvalidate.01" content="{}">\n"#, token));
        }
        None
    }

    pub fn google_verification(&self) -> Option<String> {
        if let Some(ref token) = self.google_token {
            return Some(format!(r#"<meta name="google-site-verification" content="{}">\n"#, token));
        }
        None
    }

    pub fn set_base_entity(&self, base: BaseEntity) {
        let mut meta = self.meta.write();
        meta.base = Some(base);
        drop(meta);
    }

    pub fn set_og_entity(&self, base: OpenGraphEntity) {
        let mut meta = self.meta.write();
        meta.open_graph = Some(base);
        drop(meta);
    }

    pub fn set_twitter_entity(&self, base: TwitterEntity) {
        let mut meta = self.meta.write();
        meta.twitter = Some(base);
        drop(meta);
    }

    pub fn set_json_grapth(&self, base: Vec<GraphEntity>) {
        let mut meta = self.json.write();
        meta.set_grapth(base);
        drop(meta);
    }

    pub fn push_json_grapth(&self, base: GraphEntity) {
        let mut meta = self.json.write();
        meta.push_grapth(base);
        drop(meta);
    }

    pub fn meta_to_html(&self) -> String {
        let mut html = String::new();

        let meta = self.meta.write();
        html.push_str(&meta.to_html());
        drop(meta);

        if let Some(ref value) = self.bing_verification() {
            html.push_str(&value);
        }

        if let Some(ref value) = self.google_verification() {
            html.push_str(&value);
        }

        html
    }

    pub fn meta_to_html_with_jsonld(&self) -> String {
        let mut html = String::new();

        let meta = self.meta.write();
        html.push_str(&meta.to_html());
        drop(meta);

        if let Some(ref value) = self.bing_verification() {
            html.push_str(&value);
        }

        if let Some(ref value) = self.google_verification() {
            html.push_str(&value);
        }

        html
    }

}



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenGraphEntity {
    #[serde(rename = "og:type")]
    kind : String,
    #[serde(rename = "og:site_name")]
    site_name : String,
    #[serde(rename = "og:title")]
    title : String,
    #[serde(rename = "og:description")]
    description : String,
    #[serde(rename = "og:url")]
    url : String,
    #[serde(rename = "og:locale")]
    locale : String,
    #[serde(rename = "og:image")]
    image : Option<String>,
    #[serde(rename = "og:image:alt")]
    image_alt :  Option<String>,
}

impl Default for OpenGraphEntity {
    fn default() -> Self {
        Self {
            kind: "website".to_string(),
            site_name: "example".to_string(),
            title: "example | Website".to_string(),
            description: "The Website of example com.".to_string(),
            url: "https://example.com/".to_string(),
            locale: "en_US".to_string(),
            image: None,
            image_alt: None,
        }
    }
}

impl OpenGraphEntity {
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str(format!(r#"<meta property="og:type" content="{}">\n"#, self.kind).as_str());
        html.push_str(format!(r#"<meta property="og:site_name" content="{}">\n"#, self.site_name).as_str());
        html.push_str(format!(r#"<meta property="og:title" content="{}">\n"#, self.title).as_str());
        html.push_str(format!(r#"<meta property="og:description" content="{}">\n"#, self.description).as_str());
        html.push_str(format!(r#"<meta property="og:url" content="{}">\n"#, self.url).as_str());
        html.push_str(format!(r#"<meta property="og:locale" content="{}">\n"#, self.locale).as_str());

        if let Some(ref value) = self.image {
            html.push_str(format!(r#"<meta property="og:image" content="{}">\n"#, value).as_str());
        }
        if let Some(ref value) = self.image_alt {
            html.push_str(format!(r#"<meta property="og:image:alt" content="{}">\n"#, value).as_str());
        }

        html
    }
}



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TwitterEntity {
    #[serde(rename = "twitter:card")]
    card : String,
    #[serde(rename = "twitter:title")]
    title : String,
    #[serde(rename = "twitter:description")]
    description : String,
    #[serde(rename = "twitter:image")]
    image :  Option<String>,
}

impl Default for TwitterEntity {
    fn default() -> Self {
        Self {
            card: "website".to_string(),
            title: "example | Website".to_string(),
            description: "The Website of example com.".to_string(),
            image: None
        }
    }
}

impl TwitterEntity {
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str(format!(r#"<meta property="twitter:card" content="{}">\n"#, self.card).as_str());
        html.push_str(format!(r#"<meta property="twitter:title" content="{}">\n"#, self.title).as_str());
        html.push_str(format!(r#"<meta property="twitter:description" content="{}">\n"#, self.description).as_str());

        if let Some(ref value) = self.image {
            html.push_str(format!(r#"<meta property="twitter:image" content="{}">\n"#, value).as_str());
        }

        html
    }
}