# googlebooks-rs

A Rust client library for the Google Books API.

## Features

- Simple and ergonomic query builder
- Search by ISBN, title, author, publisher, and more
- Type-safe response models

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
googlebooks-rs = "0.2.1"
```

## Usage

```rust
use googlebook_rs::{GoogleBooks, queries::VolumeQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GoogleBooks::new();
    // or with an API key
    // let client = GoogleBooks::new(Some("API_KEY".to_string()));
    
    // Search by ISBN
    let query = VolumeQuery::isbn("9782348054693");
    let response = client.search(query).await?;
    
    // Search by title and author
    let query = VolumeQuery::title("Rust Programming")
        .and_author("Steve Klabnik")
        .max_results(10);
    let response = client.search(query).await?;
    
    // Get a specific book by volume ID
    let response = GoogleBooks::search_by_id("zyTCAlFPjgYC").await?;
    
    Ok(())
}
```

## Query Builder

The library provides a fluent API for building queries:

```rust
use googlebook_rs::queries::{VolumeQuery, PrintType, Projection};

let query = VolumeQuery::author("Victor Hugo")
    .lang_restrict("fr".to_string())
    .print_type(PrintType::Books)
    .projection(Projection::Lite)
    .max_results(20);
```

### Available Query Methods

- `new(search)` - Generic search
- `isbn(isbn)` - Search by ISBN
- `title(title)` - Search by title
- `author(author)` - Search by author
- `publisher(publisher)` - Search by publisher
- `subject(subject)` - Search by subject
- `lccn(lccn)` - Search by Library of Congress Control Number
- `oclc(oclc)` - Search by OCLC number

You can chain queries with `and_*` methods:

- `and_isbn(isbn)`
- `and_title(title)`
- `and_author(author)`
- `and_publisher(publisher)`
- `and_subject(subject)`
- `and_lccn(lccn)`
- `and_oclc(oclc)`

### Query Options

- `max_results(n)` - Limit the number of results
- `start_index(n)` - Pagination offset
- `lang_restrict(lang)` - Filter by language (e.g., "en", "fr")
- `projection(Projection)` - Metadata detail level (Full or Lite)
- `print_type(PrintType)` - Filter by content type (All, Books, or Magazines)

## License

This project is licensed under the AGPL V3 License.
