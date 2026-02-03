use googlebooks_rs::{queries::VolumeQuery, GoogleBooks};

#[tokio::main]
async fn main() {
    let client = GoogleBooks::new(None);

    let query = VolumeQuery::new("la femme de menage");

    match client.search(query).await {
        Ok(results) => {
            println!("{:#?}", results);
        }
        Err(e) => eprintln!("Erreur: {:?}", e),
    }
}
