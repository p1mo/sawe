
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum General {
    #[default]
    #[serde(rename = "Thing")]
    Thing,
    #[serde(rename = "Intangible")]
    Intangible,
    #[serde(rename = "Enumeration")]
    Enumeration,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Websites {
    #[default]
    #[serde(rename = "WebSite")]
    WebSite,
    #[serde(rename = "WebPage")]
    WebPage,
    #[serde(rename = "FAQPage")]
    FAQPage,
    #[serde(rename = "SearchResultsPage")]
    SearchResultsPage,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Organizations {
    #[default]
    #[serde(rename = "Organization")]
    Organization,
    #[serde(rename = "Corporation")]
    Corporation,
    #[serde(rename = "OnlineStore")]
    OnlineStore,
    #[serde(rename = "NGO")]
    NGO,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum LocalBusiness {
    #[default]
    #[serde(rename = "LocalBusiness")]
    LocalBusiness,
    #[serde(rename = "Restaurant")]
    Restaurant,
    #[serde(rename = "Store")]
    Store,
    #[serde(rename = "Hotel")]
    Hotel,
    #[serde(rename = "Dentist")]
    Dentist,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum People {
    #[default]
    #[serde(rename = "Person")]
    Person,
    #[serde(rename = "ProfilePage")]
    ProfilePage,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Places {
    #[default]
    #[serde(rename = "Place")]
    Place,
    #[serde(rename = "PostalAddress")]
    PostalAddress,
    #[serde(rename = "GeoCoordinates")]
    GeoCoordinates,
    #[serde(rename = "Country")]
    Country,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Products {
    #[default]
    #[serde(rename = "Product")]
    Product,
    #[serde(rename = "ProductGroup")]
    ProductGroup,
    #[serde(rename = "IndividualProduct")]
    IndividualProduct,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Offers {
    #[default]
    #[serde(rename = "Offer")]
    Offer,
    #[serde(rename = "AggregateOffer")]
    AggregateOffer,
    #[serde(rename = "Demand")]
    Demand,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Reviews {
    #[default]
    #[serde(rename = "Review")]
    Review,
    #[serde(rename = "AggregateRating")]
    AggregateRating,
    #[serde(rename = "Rating")]
    Rating,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Editorial {
    #[default]
    #[serde(rename = "Article")]
    Article,
    #[serde(rename = "NewsArticle")]
    NewsArticle,
    #[serde(rename = "BlogPosting")]
    BlogPosting,
    #[serde(rename = "TechArticle")]
    TechArticle,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Books {
    #[default]
    #[serde(rename = "Book")]
    Book,
    #[serde(rename = "BookSeries")]
    BookSeries,
    #[serde(rename = "Chapter")]
    Chapter,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Media {
    #[default]
    #[serde(rename = "ImageObject")]
    ImageObject,
    #[serde(rename = "VideoObject")]
    VideoObject,
    #[serde(rename = "AudioObject")]
    AudioObject,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Entertainment {
    #[default]
    #[serde(rename = "Movie")]
    Movie,
    #[serde(rename = "TVSeries")]
    TVSeries,
    #[serde(rename = "MusicRecording")]
    MusicRecording,
    #[serde(rename = "PodcastSeries")]
    PodcastSeries,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Food {
    #[default]
    #[serde(rename = "Recipe")]
    Recipe,
    #[serde(rename = "Menu")]
    Menu,
    #[serde(rename = "MenuItem")]
    MenuItem,
    #[serde(rename = "FoodEstablishment")]
    FoodEstablishment,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Education {
    #[default]
    #[serde(rename = "Course")]
    Course,
    #[serde(rename = "CourseInstance")]
    CourseInstance,
    #[serde(rename = "EducationalOrganization")]
    EducationalOrganization,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Events {
    #[default]
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "BusinessEvent")]
    BusinessEvent,
    #[serde(rename = "EducationEvent")]
    EducationEvent,
    #[serde(rename = "SportsEvent")]
    SportsEvent,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Employment {
    #[default]
    #[serde(rename = "JobPosting")]
    JobPosting,
    #[serde(rename = "Occupation")]
    Occupation,
    #[serde(rename = "EmployeeRole")]
    EmployeeRole,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Questions {
    #[default]
    #[serde(rename = "Question")]
    Question,
    #[serde(rename = "Answer")]
    Answer,
    #[serde(rename = "QAPage")]
    QAPage,
    #[serde(rename = "FAQPage")]
    FAQPage,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Instructions {
    #[default]
    #[serde(rename = "HowTo")]
    HowTo,
    #[serde(rename = "HowToStep")]
    HowToStep,
    #[serde(rename = "HowToSection")]
    HowToSection,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Software {
    #[default]
    #[serde(rename = "SoftwareApplication")]
    SoftwareApplication,
    #[serde(rename = "SoftwareSourceCode")]
    SoftwareSourceCode,
    #[serde(rename = "WebApplication")]
    WebApplication,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Services {
    #[default]
    #[serde(rename = "Service")]
    Service,
    #[serde(rename = "FinancialService")]
    FinancialService,
    #[serde(rename = "ProfessionalService")]
    ProfessionalService,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Transport {
    #[default]
    #[serde(rename = "Vehicle")]
    Vehicle,
    #[serde(rename = "BusTrip")]
    BusTrip,
    #[serde(rename = "Flight")]
    Flight,
    #[serde(rename = "TrainTrip")]
    TrainTrip,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum RealEstate {
    #[default]
    #[serde(rename = "Residence")]
    Residence,
    #[serde(rename = "Apartment")]
    Apartment,
    #[serde(rename = "Accommodation")]
    Accommodation,
    #[serde(rename = "VacationRental")]
    VacationRental,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Data {
    #[default]
    #[serde(rename = "Dataset")]
    Dataset,
    #[serde(rename = "DataCatalog")]
    DataCatalog,
    #[serde(rename = "DataDownload")]
    DataDownload,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum Health {
    #[default]
    #[serde(rename = "MedicalEntity")]
    MedicalEntity,
    #[serde(rename = "MedicalOrganization")]
    MedicalOrganization,
    #[serde(rename = "MedicalCondition")]
    MedicalCondition,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TypeDirectory {
    General(General),
    Websites(Websites),
    Organizations(Organizations),
    LocalBusiness(LocalBusiness),
    People(People),
    Places(Places),
    Products(Products),
    Offers(Offers),
    Reviews(Reviews),
    Editorial(Editorial),
    Books(Books),
    Media(Media),
    Entertainment(Entertainment),
    Food(Food),
    Education(Education),
    Events(Events),
    Employment(Employment),
    Questions(Questions),
    Instructions(Instructions),
    Software(Software),
    Services(Services),
    Transport(Transport),
    RealEstate(RealEstate),
    Data(Data),
    Health(Health),

    Custom(String),

    #[cfg(feature = "seo-google")]
    GoogleArticle(super::GoogleArticle),
    #[cfg(feature = "seo-google")]
    GoogleBreadcrumb(super::GoogleBreadcrumb),
    #[cfg(feature = "seo-google")]
    GoogleCarousel(super::GoogleCarousel),
    #[cfg(feature = "seo-google")]
    GoogleCourseList(super::GoogleCourseList),
    #[cfg(feature = "seo-google")]
    GoogleDataset(super::GoogleDataset),
    #[cfg(feature = "seo-google")]
    GoogleEvent(super::GoogleEvent),
    #[cfg(feature = "seo-google")]
    GoogleImageMeta(super::GoogleImageMeta),
    #[cfg(feature = "seo-google")]
    GoogleLocalBusiness(super::GoogleLocalBusiness),
    #[cfg(feature = "seo-google")]
    GoogleMovie(super::GoogleMovie),
    #[cfg(feature = "seo-google")]
    GoogleOrganization(super::GoogleOrganization),
    #[cfg(feature = "seo-google")]
    GoogleProduct(super::GoogleProduct),
    #[cfg(feature = "seo-google")]
    GoogleProfilePage(super::GoogleProfilePage),
    #[cfg(feature = "seo-google")]
    GoogleQA(super::GoogleQA),
    #[cfg(feature = "seo-google")]
    GoogleRecipe(super::GoogleRecipe),
    #[cfg(feature = "seo-google")]
    GoogleReview(super::GoogleReview),
    #[cfg(feature = "seo-google")]
    GoogleSoftware(super::GoogleSoftware),
    #[cfg(feature = "seo-google")]
    GoogleSpeakable(super::GoogleSpeakable),
    #[cfg(feature = "seo-google")]
    GoogleSubscription(super::GoogleSubscription),
    #[cfg(feature = "seo-google")]
    GoogleVacation(super::GoogleVacation),
    #[cfg(feature = "seo-google")]
    GoogleVideo(super::GoogleVideo),
}

impl Default for TypeDirectory {
    fn default() -> Self {
        Self::General(General::Thing)
    }
}