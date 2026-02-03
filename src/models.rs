use serde::Deserialize;

/// Main response from Google Books API
#[derive(Deserialize, Debug)]
pub struct VolumeResponse {
    pub kind: String,
    #[serde(rename(deserialize = "totalItems"))]
    pub total_items: i32,
    pub items: Option<Vec<Book>>,
}

/// Represents a book with its basic metadata
#[derive(Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub etag: String,
    pub kind: Option<String>,
    #[serde(rename(deserialize = "selfLink"))]
    pub self_link: String,
    #[serde(rename(deserialize = "volumeInfo"))]
    pub volume_info: VolumeInfo,
}

/// Detailed information about a book
#[derive(Deserialize, Debug)]
pub struct VolumeInfo {
    pub title: String,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<String>>,
    pub publisher: Option<String>,
    #[serde(rename(deserialize = "publishedDate"))]
    pub published_date: String,
    pub description: Option<String>,
    #[serde(rename(deserialize = "industryIdentifiers"))]
    pub industry_identifiers: Option<Vec<IndustryIdentifiers>>,
    #[serde(rename(deserialize = "pageCount"))]
    pub page_count: u16,
    #[serde(rename(deserialize = "printType"))]
    pub print_type: String,
    pub categories: Option<Vec<String>>,
    #[serde(rename(deserialize = "imageLinks"))]
    pub image_links: Option<ImageLink>,
}

/// Links to cover images
#[derive(Deserialize, Debug)]
pub struct ImageLink {
    #[serde(rename(deserialize = "smallThumbnail"))]
    pub small_thumbnail: String, // Mettre un type URL
    pub thumbnail: String, // mettre un type URL
}

/// Book standard identifiers (ISBN-10, ISBN-13, etc.)
#[derive(Deserialize, Debug)]
pub struct IndustryIdentifiers {
    pub identifier: String,
    #[serde(rename(deserialize = "type"))]
    pub identifier_type: String, // voir les types possible
}

#[derive(Debug, Deserialize)]
pub struct GoogleApiError {
    pub error: GoogleApiErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct GoogleApiErrorDetail {
    pub code: u16,
    pub message: String,
    pub status: Option<String>,
    pub errors: Option<Vec<GoogleApiErrorItem>>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleApiErrorItem {
    pub message: String,
    pub domain: String,
    pub reason: String,
}
