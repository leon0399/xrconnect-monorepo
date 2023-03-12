use serde::{self, Serialize, Deserialize, Deserializer, de};
use serde_json::Value;

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
pub struct BHapticsWebsocketV2ServerMessage {
    #[serde(rename = "RegisteredKeys")]
    registered_keys: Vec<String>,

    #[serde(rename = "ActiveKeys")]
    active_keys: Vec<String>,

    #[serde(rename = "ConnectedDeviceCount")]
    connected_device_count: u32,

    #[serde(rename = "ConnectedPositions")]
    connected_positions: Vec<String>,
}

/// Message sent from the client to the server.
///
/// # Example Message
///
/// ## Turn Off
/// ```json
/// {
///     "Submit":[
///         {"Type":"turnOff","Key":""}
///     ]
/// }
/// ```
///
/// ### Dot Mode
/// ```json
/// {
///    "Submit":[
///       {
///          "Type":"frame",
///          "Key":"key",
///          "Frame":{
///             "Position":"VestFront",
///             "PathPoints":[
///
///             ],
///             "DotPoints":[
///                {
///                   "index":0,
///                   "intensity":100
///                }
///             ],
///             "DurationMillis":1000
///          }
///       }
///    ]
/// }
/// ```
/// ### Path Mode
/// ```json
/// {
///    "Submit":[
///       {
///          "Type":"frame",
///          "Key":"key",
///          "Frame":{
///             "Position":"VestFront",
///             "PathPoints":[
///                {
///                   "x":0.5,
///                   "y":0.5,
///                   "intensity":100
///                }
///             ],
///             "DotPoints":[
///
///             ],
///             "DurationMillis":1000
///          }
///       }
///    ]
/// }
/// ```
///
/// ### Register File
/// ```json
/// {
///    "Register":[
///       {
///          "Key":0,
///          "project":{
///             "createdAt":1583739337216,
///             "description":"",
///             "layout":{
///                "layouts":{
///                   "ForearmL":[
///                      {
///                         "index":0,
///                         "x":0,
///                         "y":0
///                      },
///                      {
///                         "index":1,
///                         "x":0.5,
///                         "y":0
///                      },
///                      {
///                         "index":2,
///                         "x":1,
///                         "y":0
///                      },
///                      {
///                         "index":3,
///                         "x":0,
///                         "y":1
///                      },
///                      {
///                         "index":4,
///                         "x":0.5,
///                         "y":1
///                      },
///                      {
///                         "index":5,
///                         "x":1,
///                         "y":1
///                      }
///                   ],
///                   "ForearmR":[
///                      {
///                         "index":0,
///                         "x":0,
///                         "y":0
///                      },
///                      {
///                         "index":1,
///                         "x":0.5,
///                         "y":0
///                      },
///                      {
///                         "index":2,
///                         "x":1,
///                         "y":0
///                      },
///                      {
///                         "index":3,
///                         "x":0,
///                         "y":1
///                      },
///                      {
///                         "index":4,
///                         "x":0.5,
///                         "y":1
///                      },
///                      {
///                         "index":5,
///                         "x":1,
///                         "y":1
///                      }
///                   ]
///                },
///                "name":"Tactosy2",
///                "type":"Tactosy2"
///             },
///             "mediaFileDuration":1,
///             "name":"Block_L",
///             "tracks":[
///                {
///                   "effects":[
///                      {
///                         "modes":{
///                            "ForearmL":{
///                               "dotMode":{
///                                  "dotConnected":false,
///                                  "feedback":[
///                                     {
///                                        "endTime":117,
///                                        "playbackType":"FADE_OUT",
///                                        "pointList":[
///                                           {
///                                              "index":0,
///                                              "intensity":0.5
///                                           },
///                                           {
///                                              "index":1,
///                                              "intensity":0.5
///                                           },
///                                           {
///                                              "index":2,
///                                              "intensity":0.5
///                                           }
///                                        ],
///                                        "startTime":0
///                                     }
///                                  ]
///                               },
///                               "mode":"DOT_MODE",
///                               "pathMode":{
///                                  "feedback":[
///                                     {
///                                        "movingPattern":"CONST_SPEED",
///                                        "playbackType":"NONE",
///                                        "visible":true,
///                                        "pointList":[
///
///                                        ]
///                                     }
///                                  ]
///                               }
///                            },
///                            "ForearmR":{
///                               "dotMode":{
///                                  "dotConnected":false,
///                                  "feedback":[
///                                     {
///                                        "endTime":117,
///                                        "playbackType":"NONE",
///                                        "startTime":0,
///                                        "pointList":[
///
///                                        ]
///                                     }
///                                  ]
///                               },
///                               "mode":"PATH_MODE",
///                               "pathMode":{
///                                  "feedback":[
///                                     {
///                                        "movingPattern":"CONST_SPEED",
///                                        "playbackType":"NONE",
///                                        "visible":true,
///                                        "pointList":[
///
///                                        ]
///                                     }
///                                  ]
///                               }
///                            }
///                         },
///                         "name":"Effect 1",
///                         "offsetTime":117,
///                         "startTime":0
///                      }
///                   ],
///                   "enable":true
///                },
///                {
///                   "enable":true,
///                   "effects":[
///
///                   ]
///                }
///             ],
///             "updatedAt":1583739371784,
///             "id":"-M1yD7LJCGRHZBDz9k7Z"
///          }
///       }
///    ]
/// }
/// ```
///
/// ### Submit File
/// ```json
/// {
///    "Submit":[
///       {
///          "Type":"key",
///          "Key":"0"
///       }
///    ]
/// }
/// ```
#[derive(Deserialize, Debug)]
pub enum BHapticsWebsocketV2ClientMessage {
    Register(Vec<BHapticsWebsocketV2Register>),
    Submit(Vec<BHapticsWebsocketV2Submit>),
}

#[inline]
fn de_bhaptics_project_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(n) => n.to_string(),
        _ => return Err(de::Error::custom("expected string or number")),
    })
}

#[inline]
fn de_bhaptics_alt_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => Some(s),
        Value::Number(n) => Some(n.to_string()),
        Value::Null => None,
        _ => return Err(de::Error::custom("expected string, number or null")),
    })
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2Register {
    #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
    key: String,
    #[serde(alias = "Project")]
    project: BHapticsWebsocketV2RegisterProject,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProject {
    id: String,
    description: String,
    name: String,

    #[serde(alias = "mediaFileDuration")]
    media_file_duration: u64,
    layout: BHapticsWebsocketV2RegisterProjectLayout,
    tracks: Vec<BHapticsWebsocketV2RegisterProjectTrack>,

    #[serde(alias = "createdAt")]
    created_at: u64,

    #[serde(alias = "updatedAt")]
    updated_at: u64,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectLayout {
    #[serde(rename = "type", alias = "Type")]
    _type: String,

    #[serde(alias = "name")]
    name: String,

    layouts: std::collections::HashMap<String, Vec<BHapticsWebsocketV2RegisterProjectLayoutObject>>
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectLayoutObject {
    index: u8,
    x: f32,
    y: f32,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrack {
    #[serde(alias = "Enable")]
    enable: bool,

    #[serde(alias = "Effects")]
    effects: Vec<BHapticsWebsocketV2RegisterProjectTrackEffect>,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffect {
    name: String,

    modes: std::collections::HashMap<String, BHapticsWebsocketV2RegisterProjectTrackEffectMode>,

    #[serde(alias = "startTime")]
    start_time: u64,

    #[serde(alias = "offsetTime")]
    offset_time: u64,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffectMode {
    mode: BHapticsWebsocketV2RegisterProjectTrackEffectModeType,

    #[serde(rename = "dotMode")]
    dot_mode: BHapticsWebsocketV2RegisterProjectTrackEffectDotMode,

    #[serde(rename = "pathMode")]
    path_mode: BHapticsWebsocketV2RegisterProjectTrackEffectPathMode,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BHapticsWebsocketV2RegisterProjectTrackEffectModeType {
    DotMode,
    PathMode,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffectDotMode {
    #[serde(alias = "dotConnected")]
    dot_connected: bool,

    feedback: Vec<BHapticsWebsocketV2RegisterProjectTrackEffectModeDotModeFeedbackCollection>,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffectPathMode {
    feedback: Vec<BHapticsWebsocketV2RegisterProjectTrackEffectModePathModeFeedbackCollection>,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffectModeDotModeFeedbackCollection {
    #[serde(alias = "startTime")]
    start_time: u64,

    #[serde(alias = "endTime")]
    end_time: u64,

    #[serde(alias = "playbackType")]
    playback_type: String,

    #[serde(alias = "pointList")]
    point_list: Vec<BHapticsWebsocketV2SubmitFrameDotPoint>,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2RegisterProjectTrackEffectModePathModeFeedbackCollection {
    #[serde(alias = "movingPattern")]
    moving_pattern: BHapticsPathMovingPattern,

    #[serde(alias = "playbackType")]
    playback_type: BHapticsPlaybackType,

    #[serde(alias = "visible")]
    visible: bool,

    #[serde(alias = "pointList")]
    point_list: Vec<BHapticsWebsocketV2SubmitFramePathPoint>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BHapticsPlaybackType {
    None,
    FadeIn,
    FadeOut,
    FadeInOut,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BHapticsPathMovingPattern {
    ConstSpeed,
    #[serde(alias = "CONST_TDM")]
    ConstTDM,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "Type")]
pub enum BHapticsWebsocketV2Submit {
    #[serde(alias = "turnOffAll")]
    TurnOffAll,

    #[serde(alias = "turnOff")]
    TurnOff {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,
    },

    #[serde(alias = "key")]
    Key {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,
        #[serde(alias = "Parameters")]
        parameters: BHapticsWebsocketV2SubmitParameters
    },

    #[serde(alias = "frame")]
    Frame {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,

        #[serde(alias = "Frame")]
        frame: BHapticsWebsocketV2SubmitFrame,
    },
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2SubmitParameters {
    #[serde(rename = "altKey", deserialize_with = "de_bhaptics_alt_id")]
    alt_key: Option<String>,

    #[serde(rename = "startTimeMillis")]
    start_time_millis: Option<u32>,

    #[serde(rename = "scaleOption")]
    scale_option: Option<BHapticsWebsocketV2SubmitScaleOption>,

    #[serde(rename = "rotationOption")]
    rotation_option: Option<BHapticsWebsocketV2SubmitRotationOption>,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2SubmitScaleOption {
    intensity: f32,
    duration: f32,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2SubmitRotationOption {
    #[serde(rename = "offsetAngleX")]
    offset_angle_x: f32,

    #[serde(alias = "offsetY")]
    offset_y: f32,
}

#[derive(Deserialize, Debug)]
pub struct BHapticsWebsocketV2SubmitFrame {
    #[serde(alias = "Position")]
    position: String,

    #[serde(alias = "PathPoints")]
    path_points: Vec<BHapticsWebsocketV2SubmitFramePathPoint>,

    #[serde(alias = "DotPoints")]
    dot_points: Vec<BHapticsWebsocketV2SubmitFrameDotPoint>,

    #[serde(alias = "DurationMillis")]
    duration_millis: u32,
}

/// Can have fields in both camelCase and PascalCase
#[derive(Deserialize, Debug)]
struct BHapticsWebsocketV2SubmitFramePathPoint {
    #[serde(alias = "X")]
    x: f32,

    #[serde(alias = "Y")]
    y: f32,

    #[serde(alias = "Intensity")]
    intensity: f64,
}

/// Can have fields in both camelCase and PascalCase
#[derive(Deserialize, Debug)]
struct BHapticsWebsocketV2SubmitFrameDotPoint {
    #[serde(alias = "Index")]
    index: u32,

    #[serde(alias = "Intensity")]
    intensity: f64,

    #[serde(alias = "MotorCount")]
    motor_count: Option<u8>,
}