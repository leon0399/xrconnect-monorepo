use std::collections::HashMap;
use serde::{self, Serialize, Deserialize};

use haptic_lib::EffectInterpolation;

use super::{ DotPoint, PathPoint };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    id: String,
    description: String,
    name: String,

    #[serde(alias = "mediaFileDuration")]
    media_file_duration: u64,
    layout: ProjectLayout,
    tracks: Vec<ProjectTrack>,

    #[serde(alias = "createdAt")]
    created_at: u64,

    #[serde(alias = "updatedAt")]
    updated_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectLayout {
    #[serde(rename = "type", alias = "Type")]
    _type: String,

    #[serde(alias = "name")]
    name: String,

    layouts: HashMap<String, Vec<ProjectLayoutObject>>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectLayoutObject {
    index: u8,
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectTrack {
    #[serde(alias = "Enable")]
    enable: bool,

    #[serde(alias = "Effects")]
    effects: Vec<HapticEffect>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HapticEffect {
    name: String,

    modes: HashMap<String, HapticEffectMode>,

    #[serde(alias = "startTime")]
    start_time: u64,

    #[serde(alias = "offsetTime")]
    offset_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HapticEffectMode {
    mode: HapticFeedbackMode,

    #[serde(rename = "dotMode")]
    dot_mode: DotMode,

    #[serde(rename = "pathMode")]
    path_mode: PathMode,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HapticFeedbackMode {
    DotMode,
    PathMode,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DotMode {
    #[serde(alias = "dotConnected")]
    dot_connected: bool,

    feedback: Vec<DotModeFeedbackCollection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DotModeFeedbackCollection {
    #[serde(alias = "startTime")]
    start_time: u64,

    #[serde(alias = "endTime")]
    end_time: u64,

    #[serde(alias = "playbackType")]
    playback_type: String,

    #[serde(alias = "pointList")]
    point_list: Vec<DotPoint>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathMode {
    feedback: Vec<PathModeFeedbackCollection>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathModeFeedbackCollection {
    #[serde(alias = "movingPattern")]
    moving_pattern: PathMovingPattern,

    #[serde(alias = "playbackType")]
    playback_type: EffectInterpolation,

    #[serde(alias = "visible")]
    visible: bool,

    #[serde(alias = "pointList")]
    point_list: Vec<PathPoint>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PathMovingPattern {
    ConstSpeed,
    #[serde(alias = "CONST_TDM")]
    ConstTDM,
}
