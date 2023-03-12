use serde::{ self, Serialize, Deserialize };

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BodyPart {
  ChestFront = 0x00,
  ChestBack = 0x01,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EffectPath {
  Haptic(BodyPart),
  Thermal(BodyPart),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EffectFrame {
  path: EffectPath,
  duration_millis: u32,
  points: Vec<EffectPoint>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EffectInterpolation {
  None,
  FadeIn,
  FadeOut,
  FadeInOut,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EffectPoint {
  pub x: u8,
  pub y: u8,
  pub intensity: u8,
}
