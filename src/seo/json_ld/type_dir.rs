
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

    #[cfg(feature = "google-type-dir")]
    GoogleArticle(GoogleArticle),
    #[cfg(feature = "google-type-dir")]
    GoogleBreadcrumb(GoogleBreadcrumb),
    #[cfg(feature = "google-type-dir")]
    GoogleCarousel(GoogleCarousel),
    #[cfg(feature = "google-type-dir")]
    GoogleCourseList(GoogleCourseList),
    #[cfg(feature = "google-type-dir")]
    GoogleDataset(GoogleDataset),
    #[cfg(feature = "google-type-dir")]
    GoogleEvent(GoogleEvent),
    #[cfg(feature = "google-type-dir")]
    GoogleImageMeta(GoogleImageMeta),
    #[cfg(feature = "google-type-dir")]
    GoogleLocalBusiness(GoogleLocalBusiness),
    #[cfg(feature = "google-type-dir")]
    GoogleMovie(GoogleMovie),
    #[cfg(feature = "google-type-dir")]
    GoogleOrganization(GoogleOrganization),
    #[cfg(feature = "google-type-dir")]
    GoogleProduct(GoogleProduct),
    #[cfg(feature = "google-type-dir")]
    GoogleProfilePage(GoogleProfilePage),
    #[cfg(feature = "google-type-dir")]
    GoogleQA(GoogleQA),
    #[cfg(feature = "google-type-dir")]
    GoogleRecipe(GoogleRecipe),
    #[cfg(feature = "google-type-dir")]
    GoogleReview(GoogleReview),
    #[cfg(feature = "google-type-dir")]
    GoogleSoftware(GoogleSoftware),
    #[cfg(feature = "google-type-dir")]
    GoogleSpeakable(GoogleSpeakable),
    #[cfg(feature = "google-type-dir")]
    GoogleSubscription(GoogleSubscription),
    #[cfg(feature = "google-type-dir")]
    GoogleVacation(GoogleVacation),
    #[cfg(feature = "google-type-dir")]
    GoogleVideo(GoogleVideo),
}

impl Default for TypeDirectory {
    fn default() -> Self {
        Self::General(General::Thing)
    }
}