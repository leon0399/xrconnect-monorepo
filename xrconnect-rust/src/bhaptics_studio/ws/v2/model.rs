use std::collections::HashMap;
use serde::{self, Serialize, Deserialize, Deserializer, de};

/// Reference: [GitHub][reference]
///
/// [reference]: https://github.com/bhaptics/haptic-library/blob/master/include/shared/model.h#LL13C5-L25C7
pub enum PositionType {
    All = 0,
    Left = 1, Right = 2, // deprecated
    Vest = 3,
    Head = 4,
    Racket = 5, // deprecated
    HandL = 6, HandR = 7,
    FootL = 8, FootR = 9,
    ForearmL = 10, ForearmR = 11,
    VestFront = 201, VestBack = 202,
    GloveL = 203, GloveR = 204,
    Custom1 = 251, Custom2 = 252, Custom3 = 253, Custom4 = 254,
}

/// Message sent from the server to the client.
///
/// # Example Message
/// ```json
/// {
///     "RegisteredKeys":["0","11"],
///     "ActiveKeys":["key", "0"],
///     "ConnectedDeviceCount":0,
///     "ConnectedPositions":["VestBack"],
///     "Status":{
///         "VestBack":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "VestFront":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "Head":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "FootL":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "FootR":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "HandL":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "HandR":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "ForearmL":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "ForearmR":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "GloveL":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
///         "GloveR":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
///    }
/// }
/// ```
#[derive(Serialize)]
pub struct PlayerResponse {
    #[serde(rename = "RegisteredKeys")]
    registered_keys: Vec<String>,

    #[serde(rename = "ActiveKeys")]
    active_keys: Vec<String>,

    #[serde(rename = "ConnectedDeviceCount")]
    connected_device_count: u32,

    #[serde(rename = "ConnectedPositions")]
    connected_positions: Vec<String>,

    #[serde(rename = "Status")]
    status: HashMap<String, i64>,
}
