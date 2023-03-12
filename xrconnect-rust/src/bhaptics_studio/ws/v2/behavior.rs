use crate::bhaptics_studio::{
    server::{
        BHapticsStudioServer,
        BHapticsAppInfo,
    },
    tact::PlayerRequest,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use serde_json::Value::Null;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use tracing::{ instrument, debug, error, info };

use warp::{
    self,
    ws::{ Message, WebSocket },
    Filter, Reply, Rejection,
};

pub struct BHapticsWebsocketV2Behavior {

}

impl BHapticsWebsocketV2Behavior {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("v2" / "feedbacks")
            .and(warp::ws())
            .and(warp::query::<BHapticsAppInfo>())
            .and_then(ws_handler)
    }
}

#[instrument]
async fn ws_handler(ws: warp::ws::Ws, app_info: BHapticsAppInfo) -> Result<impl Reply, Rejection> {
    info!("Client connected to bHaptics Studio /v2/feedbacks");

    Ok(ws.on_upgrade(|socket| async move {
        tokio::task::spawn(async move {
            let (mut _tx, mut rx) = socket.split();

            while let Some(result) = rx.next().await {
                match result {
                    Ok(msg) => {
                        if msg.is_close() {
                            info!("Client disconnected from bHaptics Studio /v2/feedbacks");
                            break;
                        }

                        match serde_json::from_slice::<PlayerRequest>(&msg.as_bytes()) {
                            Err(why) => error!("Invalid message from the client: {:?}", why),
                            Ok(message) => handle_haptic_request(message.clone(), app_info.clone()).await,
                        }
                    },
                    Err(why) => error!("Error receiving message: {:?}", why),
                }
            }
        });
    }))
}

#[instrument]
async fn handle_haptic_request(message: PlayerRequest, app_info: BHapticsAppInfo) {
    //
}