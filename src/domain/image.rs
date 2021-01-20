use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait ImageRepository {
    async fn list(&self) -> Result<Vec<Image>, Box<dyn Error + Send + Sync>>;
    async fn inspect(&self, id: String) -> Result<ImageDetail, Box<dyn Error + Send + Sync>>;
    async fn history(&self, id: String) -> Result<ImageHistory, Box<dyn Error + Send + Sync>>;
}

pub struct Image {
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub created: DateTime<Utc>,
    pub size: i32,
    pub labels: HashMap<String, String>,
}

pub struct ImageDetail {
    pub image: Image,
    pub os: String,
    pub architecture: String,
    pub env: Vec<String>,
    pub entrypoint: Vec<String>,
    pub cmd: Vec<String>,
}

pub type ImageHistory = Vec<ImageRecord>;

pub struct ImageRecord {
    pub id: String,
    pub created_by: String,
    pub size: i32,
}
