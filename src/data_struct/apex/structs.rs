use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::data_struct::pos2::*;
use crate::data_struct::pos3::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntityList {
    pub data: HashMap<u64, Option<u64>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NamePtr {
    pub data: HashMap<u64, Option<u64>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameList {
    pub data: HashMap<u64, Option<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Player {
    pub index: u64,
    pub pointer: u64,
    pub bone_pointer: u64,
    pub hitbox: Hitbox,
    pub status: Status,
    pub position: Pos3,
    pub position_2d: Pos2,
    pub distance: f32,
    pub distance_2d: f32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalPlayer {
    pub render_pointer: u64,
    pub matrix_pointer: u64,
    pub view_matrix: [[f32; 4]; 4],
    pub camera_position: Pos3,
    pub pitch: f32,
    pub yaw: f32,
    pub player: Player,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Hitbox {
    pub head: Bone,
    pub neck: Bone,
    pub upper_chest: Bone,
    pub lower_chest: Bone,
    pub stomach: Bone,
    pub hip: Bone,

    pub left_shoulder: Bone,
    pub left_elbow: Bone,
    pub left_hand: Bone,

    pub right_shoulder: Bone,
    pub right_elbow: Bone,
    pub right_hand: Bone,

    pub left_thigh: Bone,
    pub left_knee: Bone,
    pub left_foot: Bone,

    pub right_thigh: Bone,
    pub right_knee: Bone,
    pub right_foot: Bone,
}

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Bone {
    pub index: u64,
    pub position: Pos3,
    pub position_2d: Pos2,
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
    pub name: String,
    pub platform_id: u64,
}