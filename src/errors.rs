use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AppError {
    #[snafu(display("HTTP error"))]
    Http { source: reqwest::Error },
    #[snafu(display("There are an error while Json deserialization: {source}"))]
    DeserializeJson { source: reqwest::Error },
    #[snafu(display("Rate limit exceeded: {message}"))]
    RateLimitExceeded { message: String },
    #[snafu(display("Google API error {code}: {message}"))]
    GoogleApi {
        code: u16,
        message: String,
        reason: Option<String>,
    },
}
