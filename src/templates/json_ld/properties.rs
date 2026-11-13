/// SEO-focused Schema.org structured-data properties.
///
/// Schema.org vocabulary: Version 30.1
///
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEntity {
    /// Description: A unique ID (usually a URL plus #fragment) so other nodes can reference this entity
    /// Example: https://www.example.com/#organization
    #[serde(rename = "@id")]
    pub id : Option<String>,
    /// Description: The type of thing (Article, Product, ...). Can be an array: ["Product","Book"]
    /// Example: TypeDirectory::General(General::Thing)
    #[serde(rename = "@type")]
    pub typedir : super::TypeDirectory,

    /// Description: Subject matter of an entity.
    /// Applicable types: CreativeWork, Event, DefinedTerm.
    /// Example: {"@type":"Thing","name":"Search Engine Optimization"}
    pub about: Option<serde_json::Value>,

    /// Description: Accepted answer to a question.
    /// Applicable types: Question.
    /// Example: {"@type":"Answer","text":"SEO improves search visibility."}
    pub accepted_answer: Option<serde_json::Value>,

    /// Description: Page explaining how to obtain a content license.
    /// Applicable types: CreativeWork.
    /// Example: "https://example.com/license"
    pub acquire_license_page: Option<String>,

    /// Description: Additional classification.
    /// Applicable types: Thing.
    /// Example: "https://example.com/types/ProfessionalService"
    pub additional_type: Option<String>,

    /// Description: Physical postal address.
    /// Applicable types: Organization, Person, Place.
    /// Example: {"@type":"PostalAddress","streetAddress":"Main Street 10","addressLocality":"Hannover","postalCode":"30159","addressCountry":"DE"}
    pub address: Option<serde_json::Value>,

    /// Description: Country of an address.
    /// Applicable types: PostalAddress.
    /// Example: "DE"
    pub address_country: Option<String>,

    /// Description: City or locality.
    /// Applicable types: PostalAddress.
    /// Example: "Hannover"
    pub address_locality: Option<String>,

    /// Description: State or administrative region.
    /// Applicable types: PostalAddress.
    /// Example: "Lower Saxony"
    pub address_region: Option<String>,

    /// Description: Aggregate rating from multiple reviews.
    /// Applicable types: Product, Organization, LocalBusiness, Service.
    /// Example: {"@type":"AggregateRating","ratingValue":"4.8","reviewCount":"120","bestRating":"5"}
    pub aggregate_rating: Option<serde_json::Value>,

    /// Description: Alternative name of an entity.
    /// Applicable types: Thing.
    /// Example: "Example Digital Agency"
    pub alternate_name: Option<String>,

    /// Description: Amenities or features of accommodation.
    /// Applicable types: Accommodation, Place.
    /// Example: {"@type":"LocationFeatureSpecification","name":"Free Wi-Fi","value":true}
    pub amenity_feature: Option<serde_json::Value>,

    /// Description: Number of answers.
    /// Applicable types: Question.
    /// Example: "12"
    pub answer_count: Option<String>,

    /// Description: Location requirements for job applicants.
    /// Applicable types: JobPosting.
    /// Example: {"@type":"Country","name":"Germany"}
    pub applicant_location_requirements: Option<serde_json::Value>,

    /// Description: Software application category.
    /// Applicable types: SoftwareApplication.
    /// Example: "SEO Tool"
    pub application_category: Option<String>,

    /// Description: Geographic area served.
    /// Applicable types: Organization, Service, ContactPoint.
    /// Example: {"@type":"Country","name":"Germany"}
    pub area_served: Option<serde_json::Value>,

    /// Description: Main article body.
    /// Applicable types: Article.
    /// Example: "This article explains technical SEO..."
    pub article_body: Option<String>,

    /// Description: Editorial article section.
    /// Applicable types: Article.
    /// Example: "Technical SEO"
    pub article_section: Option<String>,

    /// Description: Person or organization responsible for content.
    /// Applicable types: CreativeWork, Rating.
    /// Example: {"@type":"Person","name":"Jane Smith","url":"https://example.com/authors/jane/"}
    pub author: Option<serde_json::Value>,

    /// Description: Offer availability status.
    /// Applicable types: Offer, Demand.
    /// Example: "https://schema.org/InStock"
    pub availability: Option<String>,

    /// Description: Date when availability ends.
    /// Applicable types: Offer, Demand.
    /// Example: "2026-12-31"
    pub availability_ends: Option<String>,

    /// Description: Date when availability starts.
    /// Applicable types: Offer, Demand.
    /// Example: "2026-11-01"
    pub availability_starts: Option<String>,

    /// Description: Base salary for a job.
    /// Applicable types: JobPosting, EmployeeRole.
    /// Example: {"@type":"MonetaryAmount","currency":"EUR","value":{"@type":"QuantitativeValue","value":55000,"unitText":"YEAR"}}
    pub base_salary: Option<serde_json::Value>,

    /// Description: Highest rating value.
    /// Applicable types: Rating.
    /// Example: "5"
    pub best_rating: Option<String>,

    /// Description: Brand associated with an entity.
    /// Applicable types: Organization, Person, Product, Service.
    /// Example: {"@type":"Brand","name":"Example Brand"}
    pub brand: Option<serde_json::Value>,

    /// Description: Breadcrumb navigation.
    /// Applicable types: WebPage.
    /// Example: {"@type":"BreadcrumbList","itemListElement":[{"@type":"ListItem","position":1,"name":"Home","item":"https://example.com/"}]}
    pub breadcrumb: Option<serde_json::Value>,

    /// Description: Caption for media.
    /// Applicable types: ImageObject, AudioObject, VideoObject.
    /// Example: "Our office in Hannover"
    pub caption: Option<String>,

    /// Description: Category or classification.
    /// Applicable types: Multiple types, including Product and CreativeWork.
    /// Example: "Digital Marketing"
    pub category: Option<String>,

    /// Description: Citation or source reference.
    /// Applicable types: CreativeWork, Dataset.
    /// Example: "https://developers.google.com/search/docs"
    pub citation: Option<String>,

    /// Description: Closing time.
    /// Applicable types: OpeningHoursSpecification.
    /// Example: "18:00"
    pub closes: Option<String>,

    /// Description: Direct media file URL.
    /// Applicable types: MediaObject.
    /// Example: "https://example.com/videos/seo.mp4"
    pub content_url: Option<String>,

    /// Description: Contact information.
    /// Applicable types: Organization, Person.
    /// Example: {"@type":"ContactPoint","telephone":"+49-511-123456","contactType":"customer service","email":"info@example.com"}
    pub contact_point: Option<serde_json::Value>,

    /// Description: Cooking duration.
    /// Applicable types: Recipe.
    /// Example: "PT30M"
    pub cook_time: Option<String>,

    /// Description: Copyright owner.
    /// Applicable types: CreativeWork.
    /// Example: {"@type":"Organization","name":"Example GmbH"}
    pub copyright_holder: Option<serde_json::Value>,

    /// Description: Copyright notice.
    /// Applicable types: CreativeWork.
    /// Example: "© 2026 Example GmbH"
    pub copyright_notice: Option<String>,

    /// Description: Course identifier.
    /// Applicable types: Course.
    /// Example: "SEO-101"
    pub course_code: Option<String>,

    /// Description: Course delivery mode.
    /// Applicable types: CourseInstance.
    /// Example: "Online"
    pub course_mode: Option<String>,

    /// Description: Original creator.
    /// Applicable types: CreativeWork.
    /// Example: {"@type":"Person","name":"Jane Smith"}
    pub creator: Option<serde_json::Value>,

    /// Description: Attribution text.
    /// Applicable types: CreativeWork.
    /// Example: "Image courtesy of Example GmbH"
    pub credit_text: Option<String>,

    /// Description: Original creation date.
    /// Applicable types: CreativeWork, DataFeedItem.
    /// Example: "2026-01-15T09:00:00+01:00"
    pub date_created: Option<String>,

    /// Description: Last modification date.
    /// Applicable types: CreativeWork.
    /// Example: "2026-11-12T14:00:00+01:00"
    pub date_modified: Option<String>,

    /// Description: Job or listing publication date.
    /// Applicable types: JobPosting, RealEstateListing.
    /// Example: "2026-11-10"
    pub date_posted: Option<String>,

    /// Description: Original publication date.
    /// Applicable types: CreativeWork.
    /// Example: "2026-11-10T09:00:00+01:00"
    pub date_published: Option<String>,

    /// Description: Description of an entity.
    /// Applicable types: Thing.
    /// Example: "Professional SEO and digital marketing services."
    pub description: Option<String>,

    /// Description: Direct job application availability.
    /// Applicable types: JobPosting.
    /// Example: "true"
    pub direct_apply: Option<String>,

    /// Description: Distribution of a dataset.
    /// Applicable types: Dataset.
    /// Example: {"@type":"DataDownload","encodingFormat":"text/csv","contentUrl":"https://example.com/data.csv"}
    pub distribution: Option<serde_json::Value>,

    /// Description: Event admission start time.
    /// Applicable types: Event.
    /// Example: "2026-12-12T08:30:00+01:00"
    pub door_time: Option<String>,

    /// Description: Duration of media or event.
    /// Applicable types: Event, MediaObject, Movie.
    /// Example: "PT1H30M"
    pub duration: Option<String>,

    /// Description: Educational requirements.
    /// Applicable types: JobPosting, Occupation.
    /// Example: "Bachelor's degree in Marketing"
    pub education_requirements: Option<String>,

    /// Description: Geographic eligibility.
    /// Applicable types: Offer, Demand.
    /// Example: {"@type":"Country","name":"Germany"}
    pub eligible_region: Option<serde_json::Value>,

    /// Description: Email address.
    /// Applicable types: Organization, Person, ContactPoint.
    /// Example: "info@example.com"
    pub email: Option<String>,

    /// Description: Embedded media player URL.
    /// Applicable types: VideoObject, MediaObject.
    /// Example: "https://example.com/embed/seo-video"
    pub embed_url: Option<String>,

    /// Description: Type of employment.
    /// Applicable types: JobPosting.
    /// Example: "FULL_TIME"
    pub employment_type: Option<String>,

    /// Description: Ending date.
    /// Applicable types: Event, Schedule.
    /// Example: "2026-12-12T18:00:00+01:00"
    pub end_date: Option<String>,

    /// Description: Event attendance method.
    /// Applicable types: Event.
    /// Example: "https://schema.org/OfflineEventAttendanceMode"
    pub event_attendance_mode: Option<String>,

    /// Description: Event status.
    /// Applicable types: Event.
    /// Example: "https://schema.org/EventScheduled"
    pub event_status: Option<String>,

    /// Description: Experience required for a job.
    /// Applicable types: JobPosting.
    /// Example: "3 years of SEO experience"
    pub experience_requirements: Option<String>,

    /// Description: Expiration date of media.
    /// Applicable types: MediaObject.
    /// Example: "2027-12-31"
    pub expires: Option<String>,

    /// Description: Founder of an organization.
    /// Applicable types: Organization.
    /// Example: {"@type":"Person","name":"John Smith"}
    pub founder: Option<serde_json::Value>,

    /// Description: Founding date.
    /// Applicable types: Organization.
    /// Example: "2015-06-01"
    pub founding_date: Option<String>,

    /// Description: Geographical information.
    /// Applicable types: Place.
    /// Example: {"@type":"GeoCoordinates","latitude":52.3759,"longitude":9.7320}
    pub geo: Option<serde_json::Value>,

    /// Description: Global Trade Item Number.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "04012345678901"
    pub gtin: Option<String>,

    /// Description: Eight-digit GTIN.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "12345670"
    pub gtin8: Option<String>,

    /// Description: Twelve-digit GTIN.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "012345678901"
    pub gtin12: Option<String>,

    /// Description: Thirteen-digit GTIN.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "4006381333931"
    pub gtin13: Option<String>,

    /// Description: Fourteen-digit GTIN.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "10012345678901"
    pub gtin14: Option<String>,

    /// Description: Scheduled instances of a course.
    /// Applicable types: Course.
    /// Example: {"@type":"CourseInstance","courseMode":"Online"}
    pub has_course_instance: Option<serde_json::Value>,

    /// Description: Merchant return policy.
    /// Applicable types: Offer, Organization, Product.
    /// Example: {"@type":"MerchantReturnPolicy","returnPolicyCategory":"https://schema.org/MerchantReturnFiniteReturnWindow","merchantReturnDays":30}
    pub has_merchant_return_policy: Option<serde_json::Value>,

    /// Description: Part of a larger creative work.
    /// Applicable types: CreativeWork.
    /// Example: {"@type":"WebPageElement","name":"Introduction"}
    pub has_part: Option<serde_json::Value>,

    /// Description: Article headline.
    /// Applicable types: CreativeWork.
    /// Example: "Complete Guide to Technical SEO"
    pub headline: Option<String>,

    /// Description: Hiring organization.
    /// Applicable types: JobPosting.
    /// Example: {"@type":"Organization","name":"Example GmbH","sameAs":"https://example.com"}
    pub hiring_organization: Option<serde_json::Value>,

    /// Description: Identifier of an entity.
    /// Applicable types: Thing.
    /// Example: "EXAMPLE-123"
    pub identifier: Option<String>,

    /// Description: Representative image or image collection.
    /// Applicable types: Thing.
    /// Example: "https://example.com/images/company.jpg"
    pub image: Option<serde_json::Value>,

    /// Description: Language of content.
    /// Applicable types: CreativeWork, Event.
    /// Example: "en"
    pub in_language: Option<String>,

    /// Description: Application installation URL.
    /// Applicable types: SoftwareApplication.
    /// Example: "https://example.com/download"
    pub install_url: Option<String>,

    /// Description: Statistics about content interaction.
    /// Applicable types: CreativeWork, Organization, Person.
    /// Example: {"@type":"InteractionCounter","interactionType":"https://schema.org/ReadAction","userInteractionCount":1500}
    pub interaction_statistic: Option<serde_json::Value>,

    /// Description: Indicates whether content is free to access.
    /// Applicable types: CreativeWork, Event, Place.
    /// Example: "true"
    pub is_accessible_for_free: Option<String>,

    /// Description: Parent or containing creative work.
    /// Applicable types: CreativeWork.
    /// Example: {"@id":"https://example.com/#website"}
    pub is_part_of: Option<serde_json::Value>,

    /// Description: Entity represented by a list item.
    /// Applicable types: ListItem.
    /// Example: {"@type":"Thing","name":"SEO Services"}
    pub item: Option<serde_json::Value>,

    /// Description: Product condition.
    /// Applicable types: Offer, Product, Demand.
    /// Example: "https://schema.org/NewCondition"
    pub item_condition: Option<String>,

    /// Description: Elements of a list.
    /// Applicable types: ItemList.
    /// Example: {"@type":"ListItem","position":1,"name":"SEO"}
    pub item_list_element: Option<serde_json::Value>,

    /// Description: Ordering of a list.
    /// Applicable types: ItemList.
    /// Example: "https://schema.org/ItemListOrderAscending"
    pub item_list_order: Option<String>,

    /// Description: Job location.
    /// Applicable types: JobPosting.
    /// Example: {"@type":"Place","address":{"@type":"PostalAddress","addressLocality":"Hannover","addressCountry":"DE"}}
    pub job_location: Option<serde_json::Value>,

    /// Description: Job location arrangement.
    /// Applicable types: JobPosting.
    /// Example: "TELECOMMUTE"
    pub job_location_type: Option<String>,

    /// Description: Job title of a person.
    /// Applicable types: Person.
    /// Example: "SEO Specialist"
    pub job_title: Option<String>,

    /// Description: Keywords associated with content.
    /// Applicable types: CreativeWork, Event, Organization, Place, Product.
    /// Example: "SEO, Technical SEO, Digital Marketing"
    pub keywords: Option<String>,

    /// Description: Topics about which an entity has knowledge.
    /// Applicable types: Person, Organization.
    /// Example: ["Technical SEO","Structured Data","Content Marketing"]
    pub knows_about: Option<serde_json::Value>,

    /// Description: Languages known.
    /// Applicable types: Person, Organization.
    /// Example: ["German","English"]
    pub knows_language: Option<serde_json::Value>,

    /// Description: Last review date of a webpage.
    /// Applicable types: WebPage.
    /// Example: "2026-11-12"
    pub last_reviewed: Option<String>,

    /// Description: Official registered organization name.
    /// Applicable types: Organization.
    /// Example: "Example GmbH"
    pub legal_name: Option<String>,

    /// Description: License of creative work.
    /// Applicable types: CreativeWork.
    /// Example: "https://creativecommons.org/licenses/by/4.0/"
    pub license: Option<String>,

    /// Description: Location associated with an entity.
    /// Applicable types: Event, Organization, Action.
    /// Example: {"@type":"Place","name":"Hannover"}
    pub location: Option<serde_json::Value>,

    /// Description: Official logo.
    /// Applicable types: Organization, Brand, Place, Product, Service.
    /// Example: "https://example.com/images/logo.png"
    pub logo: Option<serde_json::Value>,

    /// Description: Primary entity described by a page.
    /// Applicable types: CreativeWork.
    /// Example: {"@id":"https://example.com/#organization"}
    pub main_entity: Option<serde_json::Value>,

    /// Description: Page primarily describing an entity.
    /// Applicable types: Thing.
    /// Example: {"@id":"https://example.com/about/#webpage"}
    pub main_entity_of_page: Option<serde_json::Value>,

    /// Description: Product manufacturer.
    /// Applicable types: Product.
    /// Example: {"@type":"Organization","name":"Example Manufacturer"}
    pub manufacturer: Option<serde_json::Value>,

    /// Description: Material used in a product.
    /// Applicable types: Product, CreativeWork.
    /// Example: "Stainless steel"
    pub material: Option<String>,

    /// Description: Product model.
    /// Applicable types: Product.
    /// Example: "Model X"
    pub model: Option<String>,

    /// Description: Manufacturer part number.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "EX-2026-001"
    pub mpn: Option<String>,

    /// Description: Name of an entity.
    /// Applicable types: Thing.
    /// Example: "Example GmbH"
    pub name: Option<String>,

    /// Description: Number of bedrooms.
    /// Applicable types: Accommodation.
    /// Example: "3"
    pub number_of_bedrooms: Option<String>,

    /// Description: Number of elements in a list.
    /// Applicable types: ItemList.
    /// Example: "10"
    pub number_of_items: Option<String>,

    /// Description: Number of accommodation rooms.
    /// Applicable types: Accommodation.
    /// Example: "5"
    pub number_of_rooms: Option<String>,

    /// Description: Offers associated with an entity.
    /// Applicable types: Product, Service, Event, CreativeWork.
    /// Example: {"@type":"Offer","price":"99.00","priceCurrency":"EUR"}
    pub offers: Option<serde_json::Value>,

    /// Description: General business opening hours.
    /// Applicable types: LocalBusiness, CivicStructure.
    /// Example: "Mo-Fr 09:00-18:00"
    pub opening_hours: Option<String>,

    /// Description: Detailed opening schedule.
    /// Applicable types: Place.
    /// Example: {"@type":"OpeningHoursSpecification","dayOfWeek":["Monday","Tuesday"],"opens":"09:00","closes":"18:00"}
    pub opening_hours_specification: Option<serde_json::Value>,

    /// Description: Supported operating system.
    /// Applicable types: SoftwareApplication.
    /// Example: "Windows 11, macOS, Android"
    pub operating_system: Option<String>,

    /// Description: Event organizer.
    /// Applicable types: Event.
    /// Example: {"@type":"Organization","name":"Example Events GmbH"}
    pub organizer: Option<serde_json::Value>,

    /// Description: Accommodation occupancy.
    /// Applicable types: Accommodation.
    /// Example: {"@type":"QuantitativeValue","maxValue":4}
    pub occupancy: Option<serde_json::Value>,

    /// Description: Event performer.
    /// Applicable types: Event.
    /// Example: {"@type":"Person","name":"John Smith"}
    pub performer: Option<serde_json::Value>,

    /// Description: Position within a sequence.
    /// Applicable types: ListItem.
    /// Example: "1"
    pub position: Option<String>,

    /// Description: Postal or ZIP code.
    /// Applicable types: PostalAddress.
    /// Example: "30159"
    pub postal_code: Option<String>,

    /// Description: Product or offer price.
    /// Applicable types: Offer, PriceSpecification.
    /// Example: "99.99"
    pub price: Option<String>,

    /// Description: Currency of a price.
    /// Applicable types: Offer, PriceSpecification.
    /// Example: "EUR"
    pub price_currency: Option<String>,

    /// Description: General price range of a local business.
    /// Applicable types: LocalBusiness.
    /// Example: "€€"
    pub price_range: Option<String>,

    /// Description: Last date on which an offer price is valid.
    /// Applicable types: Offer.
    /// Example: "2026-12-31"
    pub price_valid_until: Option<String>,

    /// Description: Main image of a webpage.
    /// Applicable types: WebPage.
    /// Example: {"@type":"ImageObject","url":"https://example.com/images/main.jpg"}
    pub primary_image_of_page: Option<serde_json::Value>,

    /// Description: Product identification.
    /// Applicable types: Product.
    /// Example: "PRODUCT-001"
    pub product_id: Option<String>,

    /// Description: Provider of a service.
    /// Applicable types: Service.
    /// Example: {"@type":"Organization","name":"Example GmbH"}
    pub provider: Option<serde_json::Value>,

    /// Description: Number of ratings.
    /// Applicable types: AggregateRating.
    /// Example: "120"
    pub rating_count: Option<String>,

    /// Description: Rating value.
    /// Applicable types: Rating.
    /// Example: "4.8"
    pub rating_value: Option<String>,

    /// Description: Recipe category.
    /// Applicable types: Recipe.
    /// Example: "Dessert"
    pub recipe_category: Option<String>,

    /// Description: Recipe cuisine.
    /// Applicable types: Recipe.
    /// Example: "German"
    pub recipe_cuisine: Option<String>,

    /// Description: Recipe ingredients.
    /// Applicable types: Recipe.
    /// Example: ["250 g flour","150 g sugar","2 eggs"]
    pub recipe_ingredient: Option<serde_json::Value>,

    /// Description: Recipe preparation instructions.
    /// Applicable types: Recipe.
    /// Example: [{"@type":"HowToStep","text":"Mix the ingredients."}]
    pub recipe_instructions: Option<serde_json::Value>,

    /// Description: Recipe yield.
    /// Applicable types: Recipe.
    /// Example: "8 servings"
    pub recipe_yield: Option<String>,

    /// Description: Related webpage link.
    /// Applicable types: WebPage.
    /// Example: "https://example.com/services/"
    pub related_link: Option<String>,

    /// Description: Indicates whether an image represents webpage content.
    /// Applicable types: ImageObject.
    /// Example: "true"
    pub representative_of_page: Option<String>,

    /// Description: Review associated with an entity.
    /// Applicable types: Product, Organization, Service, Place.
    /// Example: {"@type":"Review","author":{"@type":"Person","name":"Jane"},"reviewBody":"Excellent service."}
    pub review: Option<serde_json::Value>,

    /// Description: Written text of a review.
    /// Applicable types: Review.
    /// Example: "Excellent service and professional support."
    pub review_body: Option<String>,

    /// Description: Number of reviews.
    /// Applicable types: AggregateRating.
    /// Example: "85"
    pub review_count: Option<String>,

    /// Description: Rating assigned in an individual review.
    /// Applicable types: Review.
    /// Example: {"@type":"Rating","ratingValue":"5","bestRating":"5"}
    pub review_rating: Option<serde_json::Value>,

    /// Description: Reviewer of a webpage.
    /// Applicable types: WebPage.
    /// Example: {"@type":"Person","name":"Jane Smith"}
    pub reviewed_by: Option<serde_json::Value>,

    /// Description: Authoritative URL identifying the same entity.
    /// Applicable types: Thing.
    /// Example: "https://www.linkedin.com/company/example"
    pub same_as: Option<serde_json::Value>,

    /// Description: Software screenshot.
    /// Applicable types: SoftwareApplication.
    /// Example: "https://example.com/images/screenshot.png"
    pub screenshot: Option<serde_json::Value>,

    /// Description: Seller of an offer.
    /// Applicable types: Offer.
    /// Example: {"@type":"Organization","name":"Example Store"}
    pub seller: Option<serde_json::Value>,

    /// Description: Cuisine types served by a restaurant.
    /// Applicable types: FoodEstablishment.
    /// Example: ["German","Italian"]
    pub serves_cuisine: Option<serde_json::Value>,

    /// Description: Shipping configuration.
    /// Applicable types: Offer.
    /// Example: {"@type":"OfferShippingDetails","shippingRate":{"@type":"MonetaryAmount","value":"4.90","currency":"EUR"}}
    pub shipping_details: Option<serde_json::Value>,

    /// Description: Product size.
    /// Applicable types: Product.
    /// Example: "Large"
    pub size: Option<String>,

    /// Description: Merchant-specific stock keeping unit.
    /// Applicable types: Product, Offer, Demand.
    /// Example: "SKU-001"
    pub sku: Option<String>,

    /// Description: Organization or brand slogan.
    /// Applicable types: Brand, Organization, Product, Service.
    /// Example: "Your digital growth partner"
    pub slogan: Option<String>,

    /// Description: Software application version.
    /// Applicable types: SoftwareApplication.
    /// Example: "2.5.0"
    pub software_version: Option<String>,

    /// Description: Geographic coverage of content or a dataset.
    /// Applicable types: CreativeWork, Dataset.
    /// Example: {"@type":"Place","name":"Germany"}
    pub spatial_coverage: Option<serde_json::Value>,

    /// Description: Content suitable for text-to-speech.
    /// Applicable types: CreativeWork.
    /// Example: {"@type":"SpeakableSpecification","cssSelector":[".article-headline",".article-body"]}
    pub speakable: Option<serde_json::Value>,

    /// Description: Starting date of an event.
    /// Applicable types: Event.
    /// Example: "2026-12-12T09:00:00+01:00"
    pub start_date: Option<String>,

    /// Description: Street address.
    /// Applicable types: PostalAddress.
    /// Example: "Main Street 10"
    pub street_address: Option<String>,

    /// Description: Telephone number.
    /// Applicable types: Organization, Person, Place, ContactPoint.
    /// Example: "+49-511-123456"
    pub telephone: Option<String>,

    /// Description: Temporal coverage of content.
    /// Applicable types: CreativeWork, Dataset.
    /// Example: "2026-01-01/2026-12-31"
    pub temporal_coverage: Option<String>,

    /// Description: Media thumbnail URL.
    /// Applicable types: CreativeWork.
    /// Example: "https://example.com/images/video-thumbnail.jpg"
    pub thumbnail_url: Option<String>,

    /// Description: Title of a job posting.
    /// Applicable types: JobPosting.
    /// Example: "Senior SEO Manager"
    pub title: Option<String>,

    /// Description: Total time required for a recipe.
    /// Applicable types: Recipe, HowTo.
    /// Example: "PT1H30M"
    pub total_time: Option<String>,

    /// Description: Media upload date.
    /// Applicable types: MediaObject.
    /// Example: "2026-11-10T10:00:00+01:00"
    pub upload_date: Option<String>,

    /// Description: Official URL of an entity.
    /// Applicable types: Thing.
    /// Example: "https://example.com/"
    pub url: Option<String>,

    /// Description: Last date an offer or posting remains valid.
    /// Applicable types: Offer, JobPosting, Demand.
    /// Example: "2026-12-31"
    pub valid_through: Option<String>,

    /// Description: Measured variable in a dataset.
    /// Applicable types: Dataset.
    /// Example: "Annual revenue"
    pub variable_measured: Option<serde_json::Value>,

    /// Description: Video associated with a creative work.
    /// Applicable types: CreativeWork.
    /// Example: {"@type":"VideoObject","name":"SEO Tutorial","thumbnailUrl":"https://example.com/thumb.jpg"}
    pub video: Option<serde_json::Value>,

    /// Description: Lowest rating value.
    /// Applicable types: Rating.
    /// Example: "1"
    pub worst_rating: Option<String>,

    /// Description: Word count of an article.
    /// Applicable types: Article, CreativeWork.
    /// Example: "1500"
    pub word_count: Option<String>,

    /// Description: Organization employing a person.
    /// Applicable types: Person.
    /// Example: {"@type":"Organization","name":"Example GmbH"}
    pub works_for: Option<serde_json::Value>,
}