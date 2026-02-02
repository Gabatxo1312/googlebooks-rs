use crate::{
    errors::{AppError, DeserializeJsonSnafu, HttpSnafu},
    models::VolumeResponse,
    queries::VolumeQuery,
};
use snafu::prelude::*;

pub mod errors;
pub mod models;
pub mod queries;

/// Base URL for Google Books API
const GOOGLE_BOOKS_BASE_URL: &str = "https://www.googleapis.com";

/// Main client for interacting with Google Books API
pub struct GoogleBooks {
    pub client: reqwest::Client,
}

impl Default for GoogleBooks {
    fn default() -> Self {
        Self::new()
    }
}

impl GoogleBooks {
    /// Creates a new GoogleBooks client instance
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Searches for books using a query builder
    ///
    /// # Example
    /// ```no_run
    /// use googlebooks_rs::{GoogleBooks, queries::VolumeQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GoogleBooks::new();
    /// let query = VolumeQuery::new("Rust programming");
    /// let response = client.search(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: VolumeQuery) -> Result<VolumeResponse, AppError> {
        let result = reqwest::get(query.build_url(GOOGLE_BOOKS_BASE_URL))
            .await
            .context(HttpSnafu)?
            .json::<VolumeResponse>()
            .await
            .context(DeserializeJsonSnafu)?;
        Ok(result)
    }

    /// Fetches a specific book by its volume ID
    ///
    /// # Example
    /// ```no_run
    /// use googlebooks_rs::GoogleBooks;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///   let response = GoogleBooks::search_by_id("zyTCAlFPjgYC").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_by_id(id: impl Into<String>) -> Result<VolumeResponse, AppError> {
        let result = reqwest::get(&format!(
            "{}/books/v1/volumes/{}",
            GOOGLE_BOOKS_BASE_URL,
            id.into()
        ))
        .await
        .context(HttpSnafu)?
        .json::<VolumeResponse>()
        .await
        .context(DeserializeJsonSnafu)?;

        Ok(result)
    }
}
