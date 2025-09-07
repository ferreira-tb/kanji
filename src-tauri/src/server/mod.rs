mod model;
mod response;
mod router;

use anyhow::Result;
use local_ip_address::local_ip;
use std::net::{IpAddr, SocketAddr, SocketAddrV4};
use tauri::AppHandle;
use tauri::async_runtime::{block_on, spawn};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

#[must_use]
#[derive(Clone, Copy, Debug)]
pub struct Server(SocketAddrV4);

impl Server {
  pub fn serve(app: &AppHandle) -> Result<Self> {
    let app = app.clone();
    let (tx, rx) = oneshot::channel();

    spawn(async move {
      let router = router::create().with_state(app);
      match bind().await {
        Ok((listener, addr)) => {
          let _ = tx.send(Ok(addr));
          axum::serve(listener, router).await.unwrap();
        }
        Err(err) => {
          let _ = tx.send(Err(err));
        }
      }
    });

    let mut addr = block_on(rx).unwrap()?;
    if let IpAddr::V4(ip) = local_ip()? {
      addr.set_ip(ip);
    }

    Ok(Self(addr))
  }

  pub fn addr(self) -> SocketAddrV4 {
    self.0
  }
}

async fn bind() -> Result<(TcpListener, SocketAddrV4)> {
  let listener = TcpListener::bind("0.0.0.0:63500").await?;
  let SocketAddr::V4(addr) = listener.local_addr()? else {
    unreachable!();
  };

  Ok((listener, addr))
}
