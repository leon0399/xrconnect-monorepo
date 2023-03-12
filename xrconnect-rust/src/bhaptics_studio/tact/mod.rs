use std::collections::HashMap;

use serde::{self, Serialize, Deserialize, Deserializer, de};
use serde_json::Value;

use super::{
    tact::project::{ Project }
};

pub mod project;

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
#[derive(Deserialize, Clone, Debug)]
pub enum PlayerRequest {
    Register(Vec<PlayerRegisterRequest>),
    Submit(Vec<PlayerSubmitRequest>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct PlayerRegisterRequest {
    #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
    key: String,
    #[serde(alias = "Project")]
    project: Project,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "Type")]
pub enum PlayerSubmitRequest {
    #[serde(alias = "turnOffAll")]
    TurnOffAll,

    #[serde(alias = "turnOff")]
    TurnOff {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,
    },

    #[serde(alias = "frame")]
    SubmitFrame {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,

        #[serde(alias = "Frame")]
        frame: SubmitFrame,
    },

    #[serde(alias = "key")]
    SubmitRegistered {
        #[serde(alias = "Key", deserialize_with = "de_bhaptics_project_id")]
        key: String,
        #[serde(alias = "Parameters")]
        parameters: RegisteredParameters
    },
}

#[derive(Deserialize, Clone, Debug)]
pub struct RegisteredParameters {
    #[serde(rename = "altKey", deserialize_with = "de_bhaptics_alt_id")]
    alt_key: Option<String>,

    #[serde(rename = "startTimeMillis")]
    start_time_millis: Option<u32>,

    #[serde(rename = "scaleOption")]
    scale_option: Option<RegisteredParametersScaleOption>,

    #[serde(rename = "rotationOption")]
    rotation_option: Option<RegisteredParametersRotationOption>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RegisteredParametersScaleOption {
    intensity: f32,
    duration: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RegisteredParametersRotationOption {
    #[serde(rename = "offsetAngleX")]
    offset_angle_x: f32,

    #[serde(alias = "offsetY")]
    offset_y: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SubmitFrame {
    #[serde(alias = "Position")]
    position: String,

    #[serde(alias = "PathPoints")]
    path_points: Vec<PathPoint>,

    #[serde(alias = "DotPoints")]
    dot_points: Vec<DotPoint>,

    #[serde(alias = "DurationMillis")]
    duration_millis: u32,
}

/// Can have fields in both camelCase and PascalCase
///
/// Inspired by [this](https://github.com/bhaptics/haptic-library/blob/master/include/shared/model.h#L53)
#[derive(Serialize, Deserialize, Clone, Debug)]
struct PathPoint {
    #[serde(alias = "X")]
    x: f32,

    #[serde(alias = "Y")]
    y: f32,

    #[serde(alias = "Intensity")]
    intensity: f64,
}

/// Can have fields in both camelCase and PascalCase
///
/// Inspired by [this](https://github.com/bhaptics/haptic-library/blob/master/include/shared/model.h#L32)
#[derive(Serialize, Deserialize, Clone, Debug)]
struct DotPoint {
    #[serde(alias = "Index")]
    index: u32,

    #[serde(alias = "Intensity")]
    intensity: f64,

    #[serde(alias = "MotorCount")]
    motor_count: Option<u8>,
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
