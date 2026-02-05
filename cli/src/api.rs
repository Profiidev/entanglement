use centaurus::error::Result;
use reqwest::{Client, Response};
use url::Url;

pub struct ApiClient {
  client: Client,
  api_url: Url,
  token: Option<String>,
}

impl ApiClient {
  pub async fn new(api_url: Url, token: Option<String>) -> Result<Self> {
    let client = Client::new();

    let mut api = Self {
      client,
      api_url,
      token,
    };

    if api.is_authenticated() && !api.test_token().await? {
      api.token = None;
    }

    Ok(api)
  }

  pub fn is_authenticated(&self) -> bool {
    self.token.is_some()
  }

  pub async fn test_token(&self) -> Result<bool> {
    if self.token.is_none() {
      return Ok(false);
    }

    let response = self.send_request("/api/auth/test_token").await?;
    if !response.status().is_success() {
      return Ok(false);
    }

    let valid: bool = response.json().await?;
    Ok(valid)
  }

  async fn send_request(&self, path: &str) -> Result<Response> {
    let url = self.api_url.join(path)?;
    let mut request = self.client.get(url);
    if let Some(token) = &self.token {
      request = request.bearer_auth(token);
    }

    let request = request.build()?;
    let response = self.client.execute(request).await?;

    Ok(response.error_for_status()?)
  }
}
