mod req;
use req::request;
use req::CustomRequestOptions;
use std::collections::HashMap;

async fn make_request(url: &str, options: HashMap<String, String>) {
    let options = CustomRequestOptions { headers: options, method: None };

    match request(url, options).await {
        Ok(response_data) => println!("Response data: {}", response_data),
        Err(e) => eprintln!("Error: {}", e),
    }
}
 
#[tokio::main]
async fn main() {
    let url = "https://example.com/";
    let mut options = HashMap::new();
    options.insert("Header-Key".to_string(), "Header-Value".to_string());

    make_request(url, options).await;
}