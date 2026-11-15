
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleArticle {
    #[default]
    #[serde(rename = "Article")]
    Article,
    #[serde(rename = "NewsArticle")]
    NewsArticle,
    #[serde(rename = "BlogPosting")]
    BlogPosting,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleBreadcrumb {
    #[default]
    #[serde(rename = "BreadcrumbList")]
    BreadcrumbList,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleCarousel {
    #[default]
    #[serde(rename = "Recipe")]
    Recipe,
    #[serde(rename = "Course")]
    Course,
    #[serde(rename = "Restaurant")]
    Restaurant,
    #[serde(rename = "Movie")]
    Movie,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleCourseList {
    #[default]
    #[serde(rename = "Course")]
    Course,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleDataset {
    #[default]
    #[serde(rename = "Dataset")]
    Dataset,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleEvent {
    #[default]
    #[serde(rename = "Event")]
    Event,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleImageMeta {
    #[default]
    #[serde(rename = "ImageObject")]
    ImageObject,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleLocalBusiness {
    #[default]
    #[serde(rename = "LocalBusiness")]
    LocalBusiness,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleMovie {
    #[default]
    #[serde(rename = "Movie")]
    Movie,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleOrganization {
    #[default]
    #[serde(rename = "Organization")]
    Organization,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleProduct {
    #[default]
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "Offer")]
    Offer,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleProfilePage {
    #[default]
    #[serde(rename = "ProfilePage")]
    ProfilePage,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleQA {
    #[default]
    #[serde(rename = "QAPage")]
    QAPage,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleRecipe {
    #[default]
    #[serde(rename = "Recipe")]
    Recipe,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleReview {
    #[default]
    #[serde(rename = "AggregateRating")]
    AggregateRating,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleSoftware {
    #[default]
    #[serde(rename = "SoftwareApplication")]
    SoftwareApplication,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleSpeakable {
    #[default]
    #[serde(rename = "SpeakableSpecification")]
    SpeakableSpecification,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleSubscription {
    #[default]
    #[serde(rename = "isAccessibleForFree")]
    IsAccessibleForFree,
    #[serde(rename = "hasPart")]
    HasPart,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleVacation {
    #[default]
    #[serde(rename = "VacationRental")]
    VacationRental,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum GoogleVideo {
    #[default]
    #[serde(rename = "VideoObject")]
    VideoObject,
}