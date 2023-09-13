use reqwest::Client;
use reqwest::header::{HeaderMap, USER_AGENT};

fn main() {
    let proxy_url = "https://proxy.example.com:8080";
    
    let client = Client::builder()
        .proxy(reqwest::Proxy::http(proxy_url).unwrap())
        .proxy(reqwest::Proxy::https(proxy_url).unwrap())
        .build()
        .unwrap();
    
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
    (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".parse().unwrap());
    
    let response = client
        .get("https://example.com/protected_page")
        .headers(headers)
        .send()
        .unwrap();
}
//[dependencies]
//reqwest = "0.11"