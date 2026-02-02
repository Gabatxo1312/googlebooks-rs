use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum AppError {
    #[snafu(display("HTTP error"))]
    Http { source: reqwest::Error },
    #[snafu(display("Json error"))]
    DeserializeJson { source: reqwest::Error },
}
