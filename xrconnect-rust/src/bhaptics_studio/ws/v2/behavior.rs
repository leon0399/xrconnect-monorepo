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

async fn ws_handler(ws: warp::ws::Ws, app_info: BHapticsAppInfo) -> Result<impl Reply, Rejection> {
    println!("Client connected to /v2/feedbacks: {:?}", app_info);

    Ok(ws.on_upgrade(|socket| async move {
        tokio::task::spawn(async move {
            let (mut _tx, mut rx) = socket.split();

            while let Some(result) = rx.next().await {
                let msg = result.unwrap();

                if msg.is_close() {
                    println!("Client disconnected from /v2/feedbacks: {:?}", app_info);
                    break;
                }

                let decoded: Option<PlayerRequest> = serde_json::from_slice(&msg.as_bytes()).unwrap_or(None);

                match decoded {
                    Some(message) => handle_haptic_request(message.clone(), app_info.clone()).await,
                    _ => println!("Invalid message from the client: {:?}", msg),
                }
            }
        });
    }))
}

async fn handle_haptic_request(message: PlayerRequest, app_info: BHapticsAppInfo) {
    println!("Client sent message to /v2/feedbacks: {:?}", message);
}