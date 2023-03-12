use tracing_subscriber;
use tracing::Level;

use xrconnect::{
  bhaptics_studio::server::BHapticsStudioServer
};

pub struct XRConnectCLIArgs {
  version: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
  tracing_subscriber::fmt::init();

  let server = BHapticsStudioServer::default();

  tokio::select! {
    _ = server.run() => {
      println!("Server shutting down");
    }
    _ = tokio::signal::ctrl_c() => {
      println!("Ctrl-C received, shutting down");
    }
  }

  Ok(())
}
