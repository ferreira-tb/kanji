use anyhow::{Result, bail};
use reqwest::Client;
use std::sync::LazyLock;
use std::time::Duration;
use url::Url;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub static HTTP: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .use_rustls_tls()
    .user_agent(USER_AGENT)
    .timeout(Duration::from_secs(30))
    .https_only(true)
    .build()
    .expect("Failed to create http client")
});

pub async fn get(url: Url) -> Result<String> {
  let response = HTTP.get(url).send().await?;
  if !response.status().is_success() {
    bail!("Failed to fetch: {}", response.url());
  }

  Ok(response.text().await?)
}
