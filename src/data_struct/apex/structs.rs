use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data_struct::pos2::*;
use crate::data_struct::pos3::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub entity_list: EntityList,
    pub name_list: NameList,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntityList {
    pub data: HashMap<u64, Option<u64>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameList {
    pub data: HashMap<u64, Option<Name>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Name {
    pub ptr: u64,
    pub name: String
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub index: u64,
    pub ptr: u64,
    pub bone_ptr: u64,
    pub bone_index: Vec<u16>,
    pub bone_position: Vec<Bone>,
    pub status: Status,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalPlayer {
    pub matrix_ptr: u64,
    pub view_matrix: [[f32; 4]; 4],
    pub camera_position: Pos3,
    pub angle: Angle,
    pub player: Player,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Angle {
    pub pitch: f32,
    pub yaw: f32,
}
impl Angle {
    pub fn similar(&self, other: Self) -> bool {
        let pitch_diff = (self.pitch - other.pitch).abs();
        let yaw_diff = (self.yaw - other.yaw).abs();

        pitch_diff < 1.0 && yaw_diff < 1.0
    }
}


#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub enum BoneType {
    #[default]
    Head,
    Neck,
    UpperChest,
    LowerChest,
    Stomach,
    Hip,

    LeftShoulder,
    LeftElbow,
    LeftHand,

    RightShoulder,
    RightElbow,
    RightHand,

    LeftThigh,
    LeftKnee,
    LeftFoot,

    RightThigh,
    RightKnee,
    RightFoot,
}
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Bone {
    pub index: u64,
    pub position: Pos3,
    pub bone_type: Option<BoneType>
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub dead: u16,
    pub knocked: u16,

    pub health: u16,
    pub max_health: u16,
    pub shield: u16,
    pub max_shield: u16,

    pub helmet_type: u16,
    pub armor_type: u16,

    pub last_visible_time: f32,
    pub previous_last_visible_time: f32,
    pub last_crosshair_target_time: f32,
    pub previous_last_crosshair_target_time: f32,

    pub skin: u16,
    pub team: u16,
    pub team_index: u16,
    pub platform_id: u64,
}