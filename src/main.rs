use anyhow::{Context, Result};

use std::time::Duration;
use snarkvm::prelude::{Deserialize, MainnetV0};
use snarkvm::prelude::Block;

#[derive(Clone)]
pub struct AleoRpcClient {
    base_url: String,
    inner: reqwest::Client,
}

impl AleoRpcClient {
    pub fn new(base_url: &str) -> Self {
        // ex: https://vm.aleo.org/api/testnet3
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            inner: reqwest::Client::builder().timeout(Duration::from_secs(5)).build().unwrap(),
        }
    }

    pub async fn get_resource<R: for<'a> Deserialize<'a>>(&self, url: &str) -> Result<R> {
        let resp = self.inner.get(url).send().await?;
        let status = resp.status();
        let data = resp.text().await.context("get resource to text")?;
        let resource = match status.is_success() {
            true => serde_json::from_str::<R>(&data).with_context(move || format!("serialize data to resource: {}", data))?,
            false => return Err(anyhow::anyhow!("request {} failed, status: {}, body: {}", &url, status, data)),
        };
        Ok(resource)
    }
}

impl AleoRpcClient {
    pub async fn get_block(&self, block_height: u32) -> Result<Block<MainnetV0>> {
        let url = format!("{}/block/{}", self.base_url, block_height);
        let block = self.get_resource(&url).await?;
        Ok(block)
    }
}

#[tokio::main]
async fn main() {
    let client = AleoRpcClient::new("https://api.explorer.provable.com/v1/mainnet/");
    let block: Block<MainnetV0> = client.get_block(1203484).await.unwrap();
}
