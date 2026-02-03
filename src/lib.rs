use crate::{
    errors::{AppError, DeserializeJsonSnafu, HttpSnafu},
    models::{GoogleApiError, VolumeResponse},
    queries::VolumeQuery,
};
use snafu::prelude::*;

pub mod errors;
pub mod models;
pub mod queries;

/// Base URL for Google Books API
const GOOGLE_BOOKS_BASE_URL: &str = "https://www.googleapis.com";

/// Main client for interacting with Google Books API
#[derive(Clone)]
pub struct GoogleBooks {
    pub client: reqwest::Client,
    pub api_key: Option<String>,
}

impl Default for GoogleBooks {
    fn default() -> Self {
        Self::new(None)
    }
}

impl GoogleBooks {
    /// Creates a new GoogleBooks client instance
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Searches for books using a query builder
    ///
    /// # Example
    /// ```no_run
    /// use googlebooks_rs::{GoogleBooks, queries::VolumeQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = GoogleBooks::new(Some("api_key".to_string()));
    /// let query = VolumeQuery::new("Rust programming");
    /// let response = client.search(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search(&self, query: VolumeQuery) -> Result<VolumeResponse, AppError> {
        println!(
            "{:?}",
            query
                .build_url(GOOGLE_BOOKS_BASE_URL, self.api_key.clone())
                .as_str()
        );
        let response = reqwest::get(query.build_url(GOOGLE_BOOKS_BASE_URL, self.api_key.clone()))
            .await
            .context(HttpSnafu)?;

        if !response.status().is_success() {
            let error_body: GoogleApiError = response.json().await.context(DeserializeJsonSnafu)?;

            if error_body.error.code == 429 {
                return Err(AppError::RateLimitExceeded {
                    message: error_body.error.message,
                });
            }

            return Err(AppError::GoogleApi {
                code: error_body.error.code,
                message: error_body.error.message,
                reason: error_body
                    .error
                    .errors
                    .and_then(|e| e.first().map(|i| i.reason.clone())),
            });
        }

        let result = response
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
        let response = reqwest::get(&format!(
            "{}/books/v1/volumes/{}",
            GOOGLE_BOOKS_BASE_URL,
            id.into()
        ))
        .await
        .context(HttpSnafu)?;

        if !response.status().is_success() {
            let error_body: GoogleApiError = response.json().await.context(DeserializeJsonSnafu)?;

            if error_body.error.code == 429 {
                return Err(AppError::RateLimitExceeded {
                    message: error_body.error.message,
                });
            }

            return Err(AppError::GoogleApi {
                code: error_body.error.code,
                message: error_body.error.message,
                reason: error_body
                    .error
                    .errors
                    .and_then(|e| e.first().map(|i| i.reason.clone())),
            });
        }

        let result = response
            .json::<VolumeResponse>()
            .await
            .context(DeserializeJsonSnafu)?;

        Ok(result)
    }
}
