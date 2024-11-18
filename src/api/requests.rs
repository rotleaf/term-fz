pub mod api_requests {
    use crate::api::structs::{DetailResponse, DownloadResponse, SearchResponse};
    use reqwest::Client;
    use std::io::Result;

    pub async fn search(keyword: &str) -> Result<Vec<SearchResponse>> {
        let base_url: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
        let url: String = format!("{base_url}/search");
        let client: Client = Client::new();
        let payload: serde_json::Value = serde_json::json!({"keyword": keyword});

        let response: reqwest::Response = client.post(url).json(&payload).send().await.unwrap();
        let text: String = response.text().await.unwrap();

        Ok(serde_json::from_str::<Vec<SearchResponse>>(&text)?)
    }

    pub async fn details(path: &str) -> Result<DetailResponse> {
        let base_url: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
        let url: String = format!("{base_url}/details");
        let client: Client = Client::new();
        let payload: serde_json::Value = serde_json::json!({"path": path});

        let response: reqwest::Response = client.post(url).json(&payload).send().await.unwrap();
        let text: String = response.text().await.unwrap();

        Ok(serde_json::from_str::<DetailResponse>(&text)?)
    }

    pub async fn download(download_key: &str) -> Result<DownloadResponse> {
        let base_url: String = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
        let url: String = format!("{base_url}/download");
        let client: Client = Client::new();
        let payload: serde_json::Value = serde_json::json!({"key": download_key});

        let response: reqwest::Response = client.post(url).json(&payload).send().await.unwrap();
        let text: String = response.text().await.unwrap();

        Ok(serde_json::from_str::<DownloadResponse>(&text)?)
    }
}
