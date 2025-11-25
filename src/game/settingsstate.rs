use serde::{Deserialize, Serialize};

use super::Difficulty;

#[derive(Clone)]
pub struct SettingsState {
    pub difficulty_options: Difficulty,
    pub audio_state: i32,
    pub reset_level: bool,
}