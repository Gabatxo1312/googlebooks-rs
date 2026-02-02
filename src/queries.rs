//! Query builders for the Google Books API.
//!
//! This module provides an ergonomic builder pattern for constructing queries
//! to the Google Books Volumes API.
//!
//! # Examples
//!
//!
//! use googlebooks_rs::queries::{VolumeQuery, Projection};
//!
//! // Simple ISBN search
//! let query = VolumeQuery::isbn("9782348054693");
//!
//! // Complex search with options
//! let query = VolumeQuery::title("Gastronomie & anarchisme")
//!     .lang_restrict("en".to_string())
//!     .max_results(10)
//!     .projection(Projection::Lite);

#[derive(Debug, Clone)]
pub enum Projection {
    /// Includes all volume metadata (default).
    Full,
    /// Includes only essential metadata and access information.
    Lite,
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Projection::Full => write!(f, "full"),
            Projection::Lite => write!(f, "lite"),
        }
    }
}

/// Print type for filtering results.
#[derive(Debug, Clone)]
pub enum PrintType {
    /// Returns all content types (default).
    All,
    /// Returns only books
    Books,
    /// Returns only magazines
    Magazines,
}

impl std::fmt::Display for PrintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintType::Books => write!(f, "books"),
            PrintType::All => write!(f, "all"),
            PrintType::Magazines => write!(f, "magazines"),
        }
    }
}

/// Query builder for searching volumes in the Google Books API.
///
/// Uses the Builder pattern to construct queries in a fluent manner.
/// All methods (except constructors) return Self to enable chaining.
///
/// # Examples
///
/// use googlebooks_rs::queries::{VolumeQuery, PrintType};
///
/// // Search by ISBN
/// let query = VolumeQuery::isbn("9782348054693");
///
/// // Search with filters
/// let query = VolumeQuery::author("Victor Hugo")
///     .lang_restrict("fr".to_string())
///     .print_type(PrintType::Books)
///     .max_results(20);
///
#[derive(Debug, Clone)]
pub struct VolumeQuery {
    /// Full-text search query string.
    pub q: String,
    /// Maximum number of results to return.
    pub max_results: Option<i32>,
    /// Starting position in results (pagination).
    pub start_index: Option<i32>,
    /// Language code to filter results (e.g., "fr", "en").
    pub lang_restrict: Option<String>,
    /// Metadata projection type.
    pub projection: Option<Projection>,
    /// Print type to filter results.
    pub print_type: Option<PrintType>,
}

impl VolumeQuery {
    /// Creates a new query with a custom search string.
    ///
    /// # Arguments
    ///
    /// * search - Search term or full query string
    ///
    /// # Examples
    ///
    /// use googlebooks_rs::queries::VolumeQuery;
    ///
    /// let query = VolumeQuery::new("the housemaid".to_string());
    ///
    pub fn new(search: impl Into<String>) -> Self {
        Self {
            q: search.into(),
            max_results: None,
            start_index: None,
            lang_restrict: None,
            projection: None,
            print_type: None,
        }
    }

    /// Creates a search query by ISBN.
    pub fn isbn(isbn: impl Into<String>) -> Self {
        Self::new(format!("isbn:{}", isbn.into()))
    }

    /// Creates a search query by title.
    pub fn title(title: impl Into<String>) -> Self {
        Self::new(format!("intitle:{}", title.into()))
    }

    /// Creates a search query by author.
    pub fn author(author: impl Into<String>) -> Self {
        Self::new(format!("inauthor:{}", author.into()))
    }

    /// Creates a search query by publisher.
    pub fn publisher(publisher: impl Into<String>) -> Self {
        Self::new(format!("inpublisher:{}", publisher.into()))
    }

    /// Creates a search query by subjext.
    pub fn subject(subject: impl Into<String>) -> Self {
        Self::new(format!("subject:{}", subject.into()))
    }

    /// Creates a search query by lccn.
    pub fn lccn(lccn: impl Into<String>) -> Self {
        Self::new(format!("lccn:{}", lccn.into()))
    }

    /// Creates a search query by oclc.
    pub fn oclc(oclc: impl Into<String>) -> Self {
        Self::new(format!("oclc:{}", oclc.into()))
    }

    pub fn and_isbn(mut self, isbn: impl Into<String>) -> Self {
        self.q.push_str(&format!(" isbn:{}", isbn.into()));
        self
    }

    pub fn and_title(mut self, title: impl Into<String>) -> Self {
        self.q.push_str(&format!(" intitle:{}", title.into()));
        self
    }

    pub fn and_author(mut self, author: impl Into<String>) -> Self {
        self.q.push_str(&format!(" inauthor:{}", author.into()));
        self
    }

    pub fn and_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.q
            .push_str(&format!(" inpublisher:{}", publisher.into()));
        self
    }

    pub fn and_subject(mut self, subject: impl Into<String>) -> Self {
        self.q.push_str(&format!(" subject:{}", subject.into()));
        self
    }

    pub fn and_lccn(mut self, lccn: impl Into<String>) -> Self {
        self.q.push_str(&format!(" lccn:{}", lccn.into()));
        self
    }

    pub fn and_oclc(mut self, oclc: impl Into<String>) -> Self {
        self.q.push_str(&format!(" oclc:{}", oclc.into()));
        self
    }

    pub fn max_results(mut self, max: i32) -> Self {
        self.max_results = Some(max);
        self
    }

    pub fn start_index(mut self, index: i32) -> Self {
        self.start_index = Some(index);
        self
    }

    pub fn lang_restrict(mut self, lang: String) -> Self {
        self.lang_restrict = Some(lang);
        self
    }

    pub fn projection(mut self, projection: Projection) -> Self {
        self.projection = Some(projection);
        self
    }

    pub fn print_type(mut self, print_type: PrintType) -> Self {
        self.print_type = Some(print_type);
        self
    }

    /// Builds the final query URL.
    ///
    /// # Arguments
    ///
    /// * base - Base API URL (e.g., "<https://www.googleapis.com>")
    ///
    /// # Panics
    ///
    /// Panics if the constructed URL is invalid (should never happen in practice).
    ///
    /// # Note
    ///
    /// This method is typically called internally by the client and
    pub fn build_url(&self, base: &str) -> reqwest::Url {
        let base_url = &format!("{}/books/v1/volumes", base);
        let mut queries: Vec<(String, String)> = Vec::with_capacity(5);

        queries.push(("q".to_string(), self.q.clone()));

        if let Some(max) = self.max_results {
            queries.push(("maxResults".to_string(), max.to_string()));
        }
        if let Some(start_index) = self.start_index {
            queries.push(("startIndex".to_string(), start_index.to_string()));
        }
        if let Some(start_index) = self.start_index {
            queries.push(("startIndex".to_string(), start_index.to_string()));
        }
        if let Some(lang) = self.lang_restrict.clone() {
            queries.push(("langRestrict".to_string(), lang));
        }
        if let Some(projection) = self.projection.clone() {
            queries.push(("projection".to_string(), projection.to_string()));
        }
        if let Some(print_type) = self.print_type.clone() {
            queries.push(("projection".to_string(), print_type.to_string()));
        }

        reqwest::Url::parse_with_params(base_url, queries).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isbn_query() {
        let query = VolumeQuery::isbn("9782348054693");
        assert_eq!(query.q, "isbn:9782348054693");
    }

    #[test]
    fn test_title_query() {
        let query = VolumeQuery::title("Test");
        assert_eq!(query.q, "intitle:Test");
    }

    #[test]
    fn test_subject_query() {
        let query = VolumeQuery::subject("Yolo");
        assert_eq!(query.q, "subject:Yolo");
    }

    #[test]
    fn test_publisher_query() {
        let query = VolumeQuery::publisher("poche");
        assert_eq!(query.q, "inpublisher:poche");
    }

    #[test]
    fn test_author_query() {
        let query = VolumeQuery::author("emma goldmann");
        assert_eq!(query.q, "inauthor:emma goldmann");
    }

    #[test]
    fn test_chained_queries() {
        let query = VolumeQuery::title("la conquete du pain")
            .and_author("Pierre Koprotkine")
            .max_results(10);

        assert!(query.q.contains("intitle:la conquete du pain"));
        assert!(query.q.contains("inauthor:Pierre Koprotkine"));
        assert_eq!(query.max_results, Some(10));
    }

    #[test]
    fn test_build_url() {
        let query = VolumeQuery::isbn("123456789")
            .max_results(5)
            .lang_restrict("fr".to_string());

        let url = query.build_url("https://www.googleapis.com");
        println!("{:?}", url.as_str());

        assert!(url.as_str().contains("q=isbn%3A123456789"));
        assert!(url.as_str().contains("maxResults=5"));
        assert!(url.as_str().contains("langRestrict=fr"));
    }

    #[test]
    fn test_lccn_query() {
        let query = VolumeQuery::lccn("Yolo");
        assert_eq!(query.q, "lccn:Yolo");
    }

    #[test]
    fn test_projection_display() {
        assert_eq!(Projection::Full.to_string(), "full");
        assert_eq!(Projection::Lite.to_string(), "lite");
    }

    #[test]
    fn test_print_type_display() {
        assert_eq!(PrintType::Books.to_string(), "books");
        assert_eq!(PrintType::All.to_string(), "all");
        assert_eq!(PrintType::Magazines.to_string(), "magazines");
    }
}
