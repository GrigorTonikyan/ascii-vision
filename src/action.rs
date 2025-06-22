use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    // Camera actions
    StartCamera,
    StopCamera,
    ToggleCamera,
    CameraFrame(Vec<u8>, u32, u32), // Raw frame data with dimensions
    CameraError(String),
    // Camera controls
    NextCamera,
    PreviousCamera,
    SetCamera(u32),
    // ASCII controls
    NextCharacterSet,
    PreviousCharacterSet,
    ToggleColor,
    IncreaseScale,
    DecreaseScale,
    // Resolution controls
    IncreaseResolution,
    DecreaseResolution,
    SetResolution(u32, u32),
}
