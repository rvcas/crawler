use async_trait::async_trait;

use crate::error::Error;

pub mod cvedetails;
pub mod github;
pub mod quotes;

#[async_trait]
pub trait Spider: Send + Sync {
    type Item;

    fn name(&self) -> String;
    async fn process(&self, item: Self::Item) -> Result<(), Error>;
    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Error>;
    fn start_urls(&self) -> Vec<String>;
}
