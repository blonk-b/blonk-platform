use crate::collections::BlinkMetadata;
use reqwest::{Client, Error};

pub async fn get_blink_metadata(url: &String) -> Result<BlinkMetadata, Error> {
    let client = Client::new();

    let blink_response = client.get(url).send().await;

    match blink_response {
        Ok(res) => {
            let response = res.json::<BlinkMetadata>().await;

            match response {
                Ok(metadata) => Ok(metadata),
                Err(e) => {
                    println!("Metadata failed: {}", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("Request failed: {}", e);
            Err(e)
        }
    }
}
