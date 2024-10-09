use reqwest::{Client, Error};
use crate::collections::Blink;

pub async fn get_blink_metadata(url: &String) -> Result<Blink, Error> {
    let client = Client::new();

    let blink_response = client
        .get(url)
        .send()
        .await;

    match blink_response {
        Ok(res) => {
            let response = res.json::<Blink>().await;

            match response {
                Ok(metadata) => {
                    Ok(metadata)
                },
                Err(e) => {
                    println!("Metadata failed: {}", e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            println!("Request failed: {}", e);
            Err(e)
        }
    }
}

