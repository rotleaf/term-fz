#[derive(Debug, serde::Deserialize)]
pub struct SearchResponse {
    pub title: String,
    pub image_src: String,
    pub path: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DetailResponse {
    pub info: DetailInfo,
    pub download_items: Vec<DownloadItems>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DownloadItems {
    pub file_name: String,
    pub counter: String,
    pub download_key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct DetailInfo {
    pub runtime: String,
    pub downloads: String,
    pub plot: String,
    pub genres: Vec<String>,
    pub cast: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DownloadResponse {
    pub files: Vec<FileItems>,
}

#[derive(Debug, serde::Deserialize)]
pub struct FileItems {
    pub name: String,
    pub file_path: String,
    pub connections: String,
}
