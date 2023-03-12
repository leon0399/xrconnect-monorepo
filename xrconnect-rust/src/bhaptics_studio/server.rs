use super::{
    ws::v2::behavior::BHapticsWebsocketV2Behavior,
};

use std::{
    net::SocketAddr,
};
use std::convert::Into;
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BHapticsAppInfo {
    #[serde(default, rename = "app_id")]
    id: String,

    #[serde(default, rename = "app_name")]
    name: String,
}

impl Default for BHapticsAppInfo {
    fn default() -> Self {
        Self {
            id: String::from("no-app-name"),
            name: String::from("Client"),
        }
    }
}

pub struct BHapticsStudioServer {
    /// Socket Address for WebSocket Server
    address: SocketAddr,
}

impl Default for BHapticsStudioServer {
    fn default() -> Self {
        Self {
            address: ([0, 0, 0, 0], 15881).into(),
        }
    }
}

impl BHapticsStudioServer {
    pub fn new(address: SocketAddr) -> Self {
        Self {
            address,
        }
    }

    pub async fn run(&self) {
        let routes = BHapticsWebsocketV2Behavior::new().routes();

        warp::serve(routes)
            .run(self.address)
            .await;
    }
}