use std::time::Duration;

use centaurus::error::Result;
use centaurus::eyre::ContextCompat;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use url::Url;

use crate::api::ApiClient;

pub struct CodeServer {
  receiver: oneshot::Receiver<Option<String>>,
  task: JoinHandle<()>,
}

impl CodeServer {
  pub async fn new(app_url: Url) -> Result<Self> {
    let listener = TcpListener::bind(("127.0.0.1", 16406)).await?;
    let (sender, receiver) = oneshot::channel();

    let task = spawn(async {
      sender
        .send(
          code_server(listener, app_url)
            .await
            .map_err(|err| {
              eprintln!("Error in code server: {:?}", err);
              err
            })
            .ok(),
        )
        .unwrap_or_else(|_| eprintln!("Failed to send code result"));
    });

    Ok(Self { receiver, task })
  }

  pub fn cleanup(&self) {
    self.task.abort();
  }

  pub fn get_code(&mut self) -> Option<String> {
    self.receiver.try_recv().ok().flatten()
  }
}

async fn code_server(listener: TcpListener, app_url: Url) -> Result<String> {
  let (mut stream, _) = listener.accept().await?;
  let (reader, mut writer) = stream.split();
  let mut reader = BufReader::new(reader);

  let mut request_line = String::new();
  reader.read_line(&mut request_line).await?;

  let token_result = get_token(request_line, app_url).await;

  let body = if token_result.is_ok() { "OK" } else { "Error" };
  let res_code = if token_result.is_ok() {
    "200 OK"
  } else {
    "400 Bad Request"
  };

  let response = format!(
    "HTTP/1.1 {}\r\n\
Content-Type: text/html; charset=utf-8\r\n\
Content-Length: {}\r\n\
Connection: close\r\n\
Access-Control-Allow-Origin: *\r\n\
\r\n\
{}",
    res_code,
    body.len(),
    body
  );

  writer.write_all(response.as_bytes()).await?;
  writer.flush().await?;

  let _ = stream.shutdown().await;

  sleep(Duration::from_millis(250)).await;

  token_result
}

async fn get_token(request_line: String, app_url: Url) -> Result<String> {
  // Extract the path (e.g., "/callback?code=xyz&user=abc")
  let parts: Vec<&str> = request_line.split_whitespace().collect();
  let path = parts.get(1).context("Invalid request")?;

  let url = Url::parse(&format!("http://localhost{}", path))?;
  let code = url
    .query_pairs()
    .find(|(k, _)| k == "code")
    .map(|(_, v)| v.into_owned())
    .context("Code not found in query parameters")?;
  let user = url
    .query_pairs()
    .find(|(k, _)| k == "user")
    .map(|(_, v)| v.into_owned())
    .context("User not found in query parameters")?;

  let mut api = ApiClient::new(app_url, None).await?;
  let token = api.request_token(&code, &user).await?;

  Ok(token)
}
