use std::collections::HashMap;
use std::io::{self, Read, Seek};

use serde::Serialize;
use eso_common::*;
use crate::enums::ability_tag::AbilityTag;
use crate::enums::ability_type::AbilityType;
use crate::enums::coefficient_type::CoefficientType;
use crate::enums::damage_type::DamageType;
use crate::enums::flags::{FLAG_COST_PER_TICK, FLAG_TOGGLED};
use crate::enums::major_minor::MajorMinorBuff;
use crate::enums::mechanic::Mechanic;
use crate::enums::skill_line::SkillLine;
use crate::enums::tooltip_type::TooltipType;

pub mod enums;

pub const FLAGSIZE:  usize = 177;
pub const U2ASIZE:   usize = 5;
pub const U4SIZE:    usize = 6;
pub const U6SIZE:    usize = 33;
pub const U8SIZE:    usize = 24;
pub const U10SIZE:   usize = 14;
pub const U11SIZE:   usize = 12;
pub const U12SIZE:   usize = 27;
pub const U13SIZE:   usize = 16;
pub const U15SIZE:   usize = 15;
pub const U18SIZE:   usize = 13;
pub const U22SIZE:   usize = 5;
pub const U24SIZE:   usize = 33;

const SKILLDATA_RECORDSIZE_OFFSET: u64 = 32;
const SKILLDATA_HEADER_SIZE: u64 = 36; // magic, index, 2x length, unknown1, id, unknown3, length3, id2

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct SkillBaseData {
    #[serde(skip_serializing_if = "is_zero_u32")] pub date_time:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub caused_by:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub scribing_index:     u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub player_skill_index: u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub skill_line_id:      u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub maybe_idk:          u32,
    #[serde(skip_serializing_if = "is_zero_u64")] pub uid:                u64,
    #[serde(skip_serializing_if = "is_zero_u32")] pub cast_time:          u32,
    #[serde(skip_serializing_if = "is_zero_i32")] pub value0:             i32,
    #[serde(skip_serializing_if = "is_zero_i32")] pub value1:             i32,
    #[serde(skip_serializing_if = "is_zero_i32")] pub value2:             i32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub range:              u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub ability_type:       u8,
    #[serde(skip_serializing_if = "is_zero_u16")] pub target_type:        u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub related_ability_id: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub cc_immunity_type:   u8,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub crime_event:        u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub duration:           u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub z7:                 u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub tick:               u32,
    #[serde(skip_serializing_if = "is_zero_u16")] pub start_tick:         u16,
    #[serde(skip_serializing_if = "is_zero_u16")] pub cost:               u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub radius:             u32,
    #[serde(skip_serializing_if = "is_zero_f32")] pub angle:              f32,
    #[serde(skip_serializing_if = "is_zero_u16")] pub u4:                 u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u5:                 u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub sprint_block_rule:  u8, // 1 = prevents sprinting & blocking, 3 = prevents sprinting.
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct PreSkillCoef {
    #[serde(skip_serializing_if = "is_zero_u16")] pub u0: u16,
    #[serde(skip_serializing_if = "is_zero_f32")] pub u1: f32,
    #[serde(skip_serializing_if = "is_zero_f32")] pub u2: f32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u3: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub u4: Vec<u8>,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u5: u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u6: u32,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct PostSkillCoef {
    #[serde(skip_serializing_if = "is_zero_u16")]  pub u1: u16,
    #[serde(skip_serializing_if = "is_zero_u8")]   pub u2: u8,
    #[serde(skip_serializing_if = "is_empty_vec")] pub u3: Vec<u8>,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub u4: u32,
    #[serde(skip_serializing_if = "is_zero_u8")]   pub u5: u8,
    #[serde(skip_serializing_if = "is_zero_u8")]   pub u6: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub u7: u32,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct SkillCoef {
    #[serde(skip_serializing_if = "is_zero_u8")]  pub type1: u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub coef1: f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub type2: u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub coef2: f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub type3: u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub coef3: f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub type4: u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub coef4: f32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u20:   u32,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List26Data {
    #[serde(skip_serializing_if = "is_empty_vec")] pub u2: Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size3: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list3: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct U5Data {
    #[serde(skip_serializing_if = "is_zero_u64")] pub uid: u64,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub s5a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub d5a: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub s5b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub d5b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct TooltipData {
    #[serde(skip_serializing_if = "is_zero_u64")] pub uid: u64,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub num_tooltip_types: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub tooltip_types:     Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub num_tooltip_ids:   u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub tooltip_ids:       Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List9Data {
    #[serde(skip_serializing_if = "is_zero_u64")] pub uid: u64,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size9a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list9a: Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size9b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list9b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List11Data {
    #[serde(skip_serializing_if = "is_zero_u64")] pub uid: u64,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size11a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list11a: Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size11b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list11b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct ExecuteData {
    #[serde(skip_serializing_if = "is_zero_u8")]  pub u1:                          u8,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub threshold_below_health_pct:  u8,
    #[serde(skip_serializing_if = "is_zero_u16")] pub bonus_up_to_pct:             u16,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List25Entry {
    #[serde(skip_serializing_if = "is_zero_u32")] pub base_ability_id: u32,
    #[serde(skip_serializing_if = "is_zero_u8")] pub u1: u8,
    #[serde(skip_serializing_if = "is_zero_u8")] pub rank_index: u8,
    #[serde(skip_serializing_if = "is_zero_u8")] pub u2: u8,
}

fn is_default<T: Default + PartialEq>(v: &T) -> bool { *v == T::default() }

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct ActionSlotData {
    #[serde(skip_serializing_if = "is_zero_f32")] pub f2e0:             f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub action_slot_type: u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u2e8:             u32,
    #[serde(skip_serializing_if = "is_zero_u16")] pub u9:               u16,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub u11:              u8,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct U13Data {
    #[serde(skip_serializing_if = "is_zero_u8")]  pub u0:                    u8,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub default_always_never:  u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u334:                  u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub sheathe_weapon_option: u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u33c_ability_id:       u32,
    #[serde(skip_serializing_if = "is_zero_f32")] pub f340:                  f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub additional_telegraph_type: u8,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct U15Data {
    #[serde(skip_serializing_if = "is_zero_u32")] pub u34c_ability_id:      u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub ground_target_type:   u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub f354:                 f32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub bonus_rules_type:     u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u35c:                 u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub u14:                  u8,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct U18Data {
    #[serde(skip_serializing_if = "is_zero_u8")]  pub weapon_config_type:   u8,
    #[serde(skip_serializing_if = "is_zero_f32")] pub f36c:                 f32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub weapon_skill_line:    u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u374_ability_id:      u32,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct U22Data {
    #[serde(skip_serializing_if = "is_two_u8")]   pub cc_immunity_visuals:  u8,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u39c_ability_id:      u32,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct SkillData34 {
    #[serde(skip_serializing_if = "always_skip_serialization")] pub start_offset:   u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub end_offset:     u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub magic_header:   u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub index:          u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length1: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length2: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown1:       u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub ability_id1:    u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown3:       u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length3: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub ability_id2:    u32,
    #[serde(skip_serializing_if = "is_empty_str")]              pub name:           String,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub zero1:          u8,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub zero2:          u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero3:          u32,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1a:            u16,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1b:            u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero4:          u32,
    pub base_data:                                              SkillBaseData,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub flags: [u8; FLAGSIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size1:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub link_types:     Vec<u8>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size2:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub causes_ids:     Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub u2a:            u8,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u2b:            u32,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u4: [u16; U4SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size5:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list5:          Vec<U5Data>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub num_tooltips:   u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub tooltip_data:   Vec<TooltipData>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u6: [u8; U6SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size6a:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list6a:         Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size_tags:      u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub ability_tags:   Vec<u16>,
    pub pre:                                                    PreSkillCoef,
    pub post:                                                   PostSkillCoef,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size7:          u16,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list7:          Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_f32")]               pub f8a:            f32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub icon_file_index:u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u8c:            u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size8:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list8:          Vec<u32>,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u9a:            Vec<u8>,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u9b:            Vec<u8>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size9:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list9:          Vec<List9Data>,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub half_width:     u16,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u10: [u8; U10SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size11:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list11:         Vec<List11Data>,
    #[serde(skip_serializing_if = "is_default")]                pub action_slot:    ActionSlotData,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u12: [u8; U12SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size13:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list13:         Vec<u8>,
    #[serde(skip_serializing_if = "is_default")]                pub u13:            U13Data,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub major_minor_id: u8,
    #[serde(skip_serializing_if = "is_default")]                pub u15:            U15Data,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub u16:            u8,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u17:            u32,
    #[serde(skip_serializing_if = "is_default")]                pub u18:            U18Data,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size19:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list19:         Vec<ExecuteData>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u20:            u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size21:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list21:         Vec<u8>,
    #[serde(skip_serializing_if = "is_default")]                pub u22:            U22Data,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub mechanic:       u8,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub u23:            u8,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u24: [u8; U24SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size25:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list25:         Vec<List25Entry>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size26:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list26:         Vec<List26Data>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub size_coef:      u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub coef:           Vec<SkillCoef>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u27:            u32,
}

impl Default for SkillData34 {
    fn default() -> Self {
        Self {
            start_offset:   0,
            end_offset:     0,
            magic_header:   0,
            index:          0,
            record_length1: 0,
            record_length2: 0,
            unknown1:       0,
            ability_id1:    0,
            unknown3:       0,
            record_length3: 0,
            ability_id2:    0,
            name:           String::new(),
            zero1:          0,
            zero2:          0,
            zero3:          0,
            u1a:            0,
            u1b:            0,
            zero4:          0,
            base_data:      SkillBaseData::default(),
            flags:          [0u8; FLAGSIZE],
            size1:          0,
            link_types:     Vec::new(),
            size2:          0,
            causes_ids:     Vec::new(),
            u2a:            0,
            u2b:            0,
            u4:             [0u16; U4SIZE],
            size5:          0,
            list5:          Vec::new(),
            num_tooltips:   0,
            tooltip_data:   Vec::new(),
            u6:             [0u8; U6SIZE],
            size6a:         0,
            list6a:         Vec::new(),
            size_tags:      0,
            ability_tags:   Vec::new(),
            pre:            PreSkillCoef::default(),
            post:           PostSkillCoef::default(),
            size7:          0,
            list7:          Vec::new(),
            f8a:            0.0,
            icon_file_index:0,
            u8c:            0,
            size8:          0,
            list8:          Vec::new(),
            u9a:            Vec::new(),
            u9b:            Vec::new(),
            size9:          0,
            list9:          Vec::new(),
            half_width:     0,
            u10:            [0u8; U10SIZE],
            size11:         0,
            list11:         Vec::new(),
            action_slot:    ActionSlotData::default(),
            u12:            [0u8; U12SIZE],
            size13:         0,
            list13:         Vec::new(),
            u13:            U13Data::default(),
            major_minor_id: 0,
            u15:            U15Data::default(),
            u16:            0,
            u17:            0,
            u18:            U18Data::default(),
            size19:         0,
            list19:         Vec::new(),
            u20:            0,
            size21:         0,
            list21:         Vec::new(),
            u22:            U22Data::default(),
            mechanic:       0,
            u23:            0,
            u24:            [0u8; U24SIZE],
            size25:         0,
            list25:         Vec::new(),
            size26:         0,
            list26:         Vec::new(),
            size_coef:      0,
            coef:           Vec::new(),
            u27:            0,
        }
    }
}

impl SkillData34 {
    pub fn damage_type_id(&self) -> u16 { self.u4[3] }
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut r = ByteReader::new(std::io::Cursor::new(bytes));
        read_skill_record34_inner(&mut r, 0)
    }
}


pub mod display {
    pub fn format_duration(ms: &u32) -> String {
        let hours = ms / 3_600_000;
        let mins  = (ms % 3_600_000) / 60_000;
        let secs  = (ms % 60_000) / 1_000;
        let millis = ms % 1_000;

        match (hours, mins, secs, millis) {
            (h, 0, 0, 0) if h > 0 => format!("{}h", h),
            (0, m, 0, 0) if m > 1 => format!("{}mins", m),
            (0, m, 0, 0) if m > 0 => format!("{}min", m),
            (0, 0, s, 0) if s > 0 => format!("{}s", s),
            (h, m, 0, 0) if h > 0 => format!("{}h {}m", h, m),
            (0, m, s, 0) if m > 0 => format!("{}m {}s", m, s),
            _ => format!("{}ms", ms),
        }
    }

    pub fn format_distance(cm: &u32) -> String {
        format!("{}m", (*cm as f32 / 100.0))
    }

    pub fn format_angle(angle: &f32) -> String {
        format!("{}°", angle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeLink {
    ActionList(u32),
    GrantsSynergy(u32),
    CreatesItem(u32),
    AreaTeleport(u32),
    SetsCooldown(u32),
    RemovesBuff(u32),
}

impl SkillData34 {
    pub fn id(&self) -> u32 { self.ability_id1 }

    pub fn last_edited_timestamp(&self) -> i64 { self.base_data.date_time as i64 }

    pub fn caused_by_id(&self) -> Option<u32> {
        let c = self.base_data.caused_by;
        (c != 0 && c != self.ability_id1).then_some(c)
    }

    pub fn replaces_id(&self) -> Option<u32> {
        (self.u8c != 0 && self.u8c != self.ability_id1).then_some(self.u8c)
    }

    pub fn base_ability_id(&self) -> Option<u32> {
        let e = self.list25.first()?;
        (e.base_ability_id > 0
            && e.base_ability_id != self.ability_id1
            && self.base_data.scribing_index == 0)
            .then_some(e.base_ability_id)
    }

    pub fn mechanic(&self) -> Option<Mechanic> { Mechanic::from_id(&self.mechanic) }
    pub fn skill_line(&self) -> Option<SkillLine> { SkillLine::from_id(&self.base_data.skill_line_id) }
    pub fn ability_type(&self) -> Option<AbilityType> {
        if self.base_data.ability_type == 0 { None } else { AbilityType::from_id(&self.base_data.ability_type) }
    }
    pub fn damage_type(&self) -> Option<DamageType> { DamageType::from_id(&self.u4[3]) }
    pub fn major_minor_buff(&self) -> Option<MajorMinorBuff> {
        MajorMinorBuff::from_id(&(self.major_minor_id as u32))
    }

    pub fn target_type_label(&self) -> &'static str {
        match self.base_data.target_type {
            0 => "Enemy",
            1 => if self.base_data.angle != 0.0 { "Cone" } else { "Area" },
            2 => "Area",
            8 => "Ground",
            9 => "One ally", // 256490
            14 => "Corpse",
            32 => "You or an ally",
            33 => "Entire group near player",
            34 => "One ally", // guard
            _ => "???",
        }
    }

    pub fn cc_immunity_label(&self) -> Option<&'static str> {
        (self.base_data.cc_immunity_type != 0).then(|| match self.base_data.cc_immunity_type {
            3 => "All",
            2 => "Immobilisation",
            1 => "Interrupt",
            _ => "???",
        })
    }

    pub fn crime_label(&self) -> Option<&'static str> {
        (self.base_data.crime_event != 0).then(|| match self.base_data.crime_event {
            18 => "Necromancer",
            19 => "Vampire",
            20 => "Werewolf",
            _ => "???",
        })
    }

    pub fn scribing_ability_name(&self) -> Option<&'static str> {
        match self.base_data.scribing_index {
            0 => None,
            1 => Some("Vault"),
            2 => Some("Wield Soul"),
            3 => Some("Shield Throw"),
            4 => Some("Smash"),
            5 => Some("Elemental Explosion"),
            6 => Some("Mender's Bond"),
            7 => Some("Travelling Knife"),
            8 => Some("Soul Burst"),
            9 => Some("Ulfsild's Contingency"),
            10 => Some("Torchbearer"),
            11 => Some("Trample"),
            12 => Some("Banner Bearer"),
            _ => Some("Unknown Scribing Ability"),
        }
    }

    pub fn list26_trigger_label(&self) -> Option<&'static str> {
        let i = *self.list26.first()?.u2.first()?;
        match i {
            1  => Some("Has effect when cast"),
            4  => Some("Affects magicka/stamina resource pool"),
            7  => Some("Affects duration of eaten food"),
            8  => Some("Affects ultimate pool"),
            9  => Some("Has effect when you heal yourself or a group member/ally"),
            14 => Some("Has effect when your attack is blocked"),
            15 => Some("Has effect when you successfully roll dodge (mixed category, not guaranteed)"),
            18 => Some("Has effect when you block"),
            39 => Some("Has effect when you cast a specific kind of ability"),
            49 => Some("Has effect when you deal critical damage"),
            50 => Some("Has effect when you take critical damage"),
            76 => Some("Tel Var District Bonus"),
            78 => Some("Has effect when you bash (or blade of woe?) an enemy"),
            86 => Some("Has effect every second in combat"),
            88 => Some("Increases your ability range"),
            92 => Some("Reduces chance for enchantment proc to reduce charge"),
            93 => Some("Reduces wayshrine recall cost"),
            96 => Some("Reduces the time it takes for a fish to bite"),
            98 => Some("Reduces the chance to consume a potion"),
            _  => None,
        }
    }

    pub fn link_type_label(t: u8) -> Option<&'static str> {
        match t {
            0 => Some("Intrinsically linked"),
            1 => Some("Affects you"),
            2 => Some("Buff given to you"),
            3 => Some("Trigger on buff end"),
            4 => Some("Affects everyone in an area"),
            5 => Some("Given to player"),
            6 => Some("[De]buff given to whomever"),
            8 => Some("Debuff given to target"),
            22 => Some("Gain [de]buff on initial hit"),
            _ => None,
        }
    }

    pub fn links(&self) -> Vec<(u32, u8, Option<&'static str>)> {
        self.causes_ids
            .iter()
            .zip(self.link_types.iter())
            .map(|(id, t)| (*id, *t, Self::link_type_label(*t)))
            .collect()
    }

    pub fn tag_label(id: u16) -> String {
        match AbilityTag::from_id(&id) {
            Some(tag) => format!("{} ({})", tag.as_str(), id),
            None => format!("{}", id),
        }
    }

    pub fn is_toggled(&self) -> bool { self.flags[FLAG_TOGGLED] == 1 }
    pub fn is_cost_per_tick(&self) -> bool { self.flags[FLAG_COST_PER_TICK] == 1 }

    pub fn execute(&self) -> Option<&ExecuteData> { self.list19.first() }
    pub fn threshold_below_health_pct(&self) -> u8 { self.execute().map_or(0, |e| e.threshold_below_health_pct) }
    pub fn bonus_up_to_pct(&self) -> u16 { self.execute().map_or(0, |e| e.bonus_up_to_pct) }
    pub fn target_cap(&self) -> u16 { self.pre.u0 }

    pub fn list26_u2_value(&self, index: usize) -> Option<u32> {
        self.list26.first().and_then(|l| l.u2.get(index)).copied().filter(|v| *v != 0)
    }

    pub fn type_link(&self) -> Option<TypeLink> {
        match self.ability_type()? {
            AbilityType::ActionList => self.list26_u2_value(0).map(TypeLink::ActionList),
            AbilityType::GrantAbility => self.list26_u2_value(1).map(TypeLink::GrantsSynergy),
            AbilityType::CreateInventoryItem => self.list26_u2_value(1).map(TypeLink::CreatesItem),
            AbilityType::AreaTeleport => self.list26_u2_value(1).map(TypeLink::AreaTeleport),
            AbilityType::SetCooldown | AbilityType::ModifyCooldown => self.list26_u2_value(1).map(TypeLink::SetsCooldown),
            AbilityType::RemoveType => self.list26_u2_value(1).map(TypeLink::RemovesBuff),
            _ => None,
        }
    }

    fn is_weapon_spell(t: u8) -> bool { matches!(t, 25 | 35) }
    fn is_resource(t: u8) -> bool { matches!(t, 4 | 29) }
    fn is_resist(t: u8) -> bool { matches!(t, 13 | 22) }

    fn is_pair(a: u8, b: u8) -> bool {
        a != b
            && ((Self::is_weapon_spell(a) && Self::is_weapon_spell(b))
                || (Self::is_resource(a) && Self::is_resource(b))
                || (Self::is_resist(a) && Self::is_resist(b)))
    }

    fn paired_term(t1: u8, t2: u8, coef: f32) -> String {
        match (t1, t2) {
            (25, 35) | (35, 25) => format!("{coef}×MaxPower"),
            (4, 29) | (29, 4) => format!("{coef}×MaxResource"),
            (22, 13) | (13, 22) => format!("{coef}×MaxResist"),
            _ => format!(
                "{coef}×max({}, {})",
                CoefficientType::from_id(&t1).map_or("unknown", |t| t.as_str()),
                CoefficientType::from_id(&t2).map_or("unknown", |t| t.as_str()),
            ),
        }
    }

    fn render_coef(c: &SkillCoef) -> Option<String> {
        let terms: Vec<(u8, f32)> = [
            (c.type1, c.coef1),
            (c.type2, c.coef2),
            (c.type3, c.coef3),
            (c.type4, c.coef4),
        ]
        .into_iter()
        .filter(|(ty, coef)| *ty > 0 && *coef != 0.0)
        .collect();

        if terms.is_empty() {
            return None;
        }

        let coef_str = |id: u8, coef: f32| {
            CoefficientType::from_id(&id)
                .map(|t| format!("{coef}×{}", t.as_str()))
                .unwrap_or_else(|| format!("{coef}×unknown({id})"))
        };

        let mut used = vec![false; terms.len()];
        let mut parts = Vec::with_capacity(terms.len());

        for i in 0..terms.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            let (t1, c1) = terms[i];

            let partner = (i + 1..terms.len())
                .find(|&j| !used[j] && terms[j].1 == c1 && Self::is_pair(t1, terms[j].0));

            match partner {
                Some(j) => {
                    used[j] = true;
                    parts.push(Self::paired_term(t1, terms[j].0, c1));
                }
                None => parts.push(coef_str(t1, c1)),
            }
        }

        Some(parts.join(" + "))
    }

    pub fn equation(&self) -> Option<String> {
        let parts: Vec<String> = self.coef.iter().filter_map(Self::render_coef).collect();
        if parts.is_empty() { None } else { Some(parts.join(" AND ")) }
    }

    pub fn is_value_tooltip(t: TooltipType) -> bool {
        matches!(
            t,
            TooltipType::Percentage
                | TooltipType::StatPercentage
                | TooltipType::ReduceHeatPercent
                | TooltipType::Duration
                | TooltipType::DelayedStrike
                | TooltipType::DeprecatedZeroDuration
                | TooltipType::MinimumCooldown
                | TooltipType::IncreaseDurationOf
                | TooltipType::Knockback
                | TooltipType::SelfHeal
                | TooltipType::ReduceCostIncreaseRecovery
                | TooltipType::TickRate
                | TooltipType::MagicalDamage
                | TooltipType::MartialDamage
                | TooltipType::SingleTargetDoT
                | TooltipType::AreaHoT
                | TooltipType::SingleTargetHeal
                | TooltipType::NoblesConquest
                | TooltipType::DeprecatedMultiHit
                | TooltipType::ResourceGain
                | TooltipType::BonusUpToPercent
                | TooltipType::ThresholdBelowHealthPercent
                | TooltipType::TargetCap
                | TooltipType::IncreasedDurationVsMonsters
        )
    }

    pub fn tooltip_value(&self, t: TooltipType) -> Option<String> {
        use display::{format_distance, format_duration};
        let b = &self.base_data;
        match t {
            TooltipType::Percentage => {
                if let Some(eq) = self.equation() {
                    Some(eq)
                } else if b.value1 != 0 {
                    Some(format!("{}%", b.value1))
                } else {
                    self.major_minor_buff().map(|m| format!("{}%", m.tooltip_value()))
                }
            }
            TooltipType::StatPercentage => (b.value1 != 0).then(|| format!("{}%", b.value1)),
            TooltipType::ReduceHeatPercent => (b.value1 != 0).then(|| format!("{}%", b.value1 / 10)),
            TooltipType::Duration | TooltipType::DelayedStrike | TooltipType::DeprecatedZeroDuration => {
                (b.duration != 0).then(|| format_duration(&b.duration))
            }
            TooltipType::MinimumCooldown => (b.value0 != 0).then(|| format_duration(&(b.value0 as u32))),
            TooltipType::IncreaseDurationOf => (b.value1 != 0).then(|| format_duration(&(b.value1 as u32))),
            TooltipType::Knockback => (b.value1 != 0).then(|| format_distance(&(b.value1 as u32))),
            TooltipType::SelfHeal => if let Some(eq) = self.equation() {
                    Some(eq)
                } else if b.value1 != 0 {
                    Some(format!("{}", b.value1))
                } else {
                    None
                }
            TooltipType::ReduceCostIncreaseRecovery => (b.value1 != 0).then(|| b.value1.to_string()),
            TooltipType::TickRate => (b.tick != 0).then(|| format_duration(&b.tick)),
            TooltipType::MagicalDamage
            | TooltipType::MartialDamage
            | TooltipType::SingleTargetDoT
            | TooltipType::AreaHoT
            | TooltipType::SingleTargetHeal
            | TooltipType::NoblesConquest
            | TooltipType::DeprecatedMultiHit => {
                if let Some(eq) = self.equation() {
                    Some(eq)
                } else {
                    (b.value1 != 0).then(|| b.value1.to_string())
                }
            }
            TooltipType::ResourceGain => {
                if let Some(m) = self.major_minor_buff() {
                    Some(format!("{}", m.tooltip_value()))
                } else if let Some(eq) = self.equation() {
                    Some(eq)
                } else {
                    Some(b.value1.to_string())
                }
            }
            TooltipType::BonusUpToPercent => {
                let v = self.bonus_up_to_pct();
                (v != 0).then(|| format!("{}%", v))
            }
            TooltipType::ThresholdBelowHealthPercent => {
                let v = self.threshold_below_health_pct();
                (v != 0).then(|| format!("{}%", v))
            }
            TooltipType::TargetCap => {
                let v = self.target_cap();
                (v != 0).then(|| format!("{}", v))
            }
            TooltipType::IncreasedDurationVsMonsters => {
                let v = self.post.u4;
                (v != 0).then(|| format_duration(&v))
            }
            _ => None,
        }
    }

    pub fn has_tooltip_value(&self, t: TooltipType) -> bool {
        let b = &self.base_data;
        match t {
            TooltipType::BuffGain => true,
            TooltipType::Percentage => {
                b.value1 != 0 || self.major_minor_buff().is_some() || self.equation().is_some()
            }
            TooltipType::StatPercentage | TooltipType::ReduceHeatPercent => b.value1 != 0,
            TooltipType::Duration | TooltipType::DelayedStrike | TooltipType::DeprecatedZeroDuration => b.duration != 0,
            TooltipType::MinimumCooldown => b.value0 != 0,
            TooltipType::IncreaseDurationOf
            | TooltipType::Knockback
            | TooltipType::SelfHeal
            | TooltipType::ReduceCostIncreaseRecovery => b.value1 != 0,
            TooltipType::TickRate => b.tick != 0,
            TooltipType::MagicalDamage
            | TooltipType::MartialDamage
            | TooltipType::SingleTargetDoT
            | TooltipType::AreaHoT
            | TooltipType::SingleTargetHeal
            | TooltipType::NoblesConquest
            | TooltipType::DeprecatedMultiHit => self.equation().is_some() || b.value1 != 0,
            TooltipType::ResourceGain => {
                self.major_minor_buff().is_some() || b.value1 != 0 || self.equation().is_some()
            }
            TooltipType::BonusUpToPercent => self.bonus_up_to_pct() != 0,
            TooltipType::TargetCap => self.target_cap() != 0,
            TooltipType::ThresholdBelowHealthPercent => self.threshold_below_health_pct() != 0,
            TooltipType::IncreasedDurationVsMonsters => self.post.u4 != 0,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ParserState {
    pub size_of_skills_file: u64,
    pub magic_header:        u32,
    pub unknown1_header:     u32,
    pub num_records_header:  u32,
    pub unknown2_header:     u32,
    pub skill_index:         u32,
    pub valid_skill_ids:     HashMap<u32, u32>,
    pub skills:              Vec<SkillData34>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkillIndexEntry {
    pub ability_id:   u32,
    pub start_offset: u64,
    pub end_offset:   u64,
}

impl SkillIndexEntry {
    pub fn to_bytes(self) -> [u8; 20] {
        let mut buf = [0u8; 20];
        buf[0..4].copy_from_slice(&self.ability_id.to_be_bytes());
        buf[4..12].copy_from_slice(&self.start_offset.to_be_bytes());
        buf[12..20].copy_from_slice(&self.end_offset.to_be_bytes());
        buf
    }

    pub fn from_bytes(b: &[u8; 20]) -> Self {
        Self {
            ability_id:   u32::from_be_bytes(b[0..4].try_into().unwrap()),
            start_offset: u64::from_be_bytes(b[4..12].try_into().unwrap()),
            end_offset:   u64::from_be_bytes(b[12..20].try_into().unwrap()),
        }
    }
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            size_of_skills_file: 0,
            magic_header:        0,
            unknown1_header:     0,
            num_records_header:  0,
            unknown2_header:     0,
            skill_index:         0,
            valid_skill_ids:     HashMap::new(),
            skills:              Vec::new(),
        }
    }

    pub fn build_skill_index(state: &Self) -> Vec<SkillIndexEntry> {
        let mut entries: Vec<SkillIndexEntry> = state.skills.iter().map(|s| SkillIndexEntry {
            ability_id:   s.ability_id1,
            start_offset: s.start_offset,
            end_offset:   s.end_offset,
        }).collect();

        entries.sort_unstable_by_key(|e| e.ability_id);
        entries
    }
}

impl Default for ParserState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_skill_data(filename: &str) -> io::Result<ParserState> {
    let mut state = ParserState::new();
    eprintln!("Loading and parsing skill data file '{filename}'...");

    let mut file = ByteReader::open(filename)
        .map_err(|_| report_error(&format!("Error: Failed to open file '{filename}'!")))?;

    state.size_of_skills_file = file.file_size()?;
    state.magic_header        = file.read_dword_be()?;
    state.unknown1_header     = file.read_dword_be()?;
    state.num_records_header  = file.read_dword_be()?;
    state.unknown2_header     = file.read_dword_be()?;

    if state.magic_header != 0xFAFA_EBEB {
        return Err(report_error(&format!(
            "Error: Magic header 0x{:08X} not expected value!", state.magic_header
        )));
    }

    read_skill_records(&mut file, &mut state)?;
    Ok(state)
}

fn read_skill_records<R: Read + Seek>(
    file: &mut ByteReader<R>,
    state: &mut ParserState,
) -> io::Result<()> {
    loop {
        let remaining = state.size_of_skills_file.saturating_sub(file.tell()?);
        if remaining < SKILLDATA_HEADER_SIZE {
            if remaining > 0 {
                eprintln!("Ignoring {remaining} trailing byte(s) at end of file (partial record header).");
            }
            break;
        }
        match file.read_byte() {
            Err(_) => break,
            Ok(b) => {
                let pos = file.tell()?;
                file.seek(pos - 1)?;
                if b != b'#' { break; }
            }
        }
        let skill = read_skill_record34_inner(file, state.skill_index + 1)?;
        state.skill_index = skill.index;
        state.valid_skill_ids.insert(skill.ability_id1, state.skill_index);
        state.skills.push(skill);
    }
    Ok(())
}

fn read_skill_record34_inner<R: Read + Seek>(r: &mut ByteReader<R>, expected_index: u32) -> io::Result<SkillData34> {
    let mut skill = SkillData34::default();

    skill.start_offset   = r.tell()?;
    skill.magic_header   = r.read_dword_be()?;
    skill.index          = r.read_dword_be()?;
    skill.record_length1 = r.read_dword_be()?;
    skill.record_length2 = r.read_dword_be()?;
    skill.unknown1       = r.read_dword_be()?;

    skill.ability_id1    = r.read_dword_be()?;
    skill.unknown3       = r.read_dword_be()?;
    skill.record_length3 = r.read_dword_be()?;
    skill.ability_id2    = r.read_dword_be()?;
    debug_assert!(skill.ability_id1 == skill.ability_id2, "ability_id1 and ability_id2 are not equal");

    if skill.magic_header != 0x2323_2323 {
        return Err(report_error(&format!(
            "Error: Skill data header 0x{:08X} not expected value!", skill.magic_header
        )));
    }
    if expected_index != 0 && skill.index != expected_index {
        eprintln!("Skill Index Mismatch: {} != {}", skill.index, expected_index);
    }
    if skill.record_length1 != skill.record_length2 {
        eprintln!("Record Length 1+2 Mismatch: 0x{:08X} 0x{:08X}", skill.record_length1, skill.record_length2);
    }
    if skill.record_length1 != skill.record_length3 {
        eprintln!("Record Length 1+3 Mismatch: 0x{:08X} 0x{:08X}", skill.record_length1, skill.record_length3);
    }
    if skill.record_length2 != skill.record_length3 {
        eprintln!("Record Length 2+3 Mismatch: 0x{:08X} 0x{:08X}", skill.record_length2, skill.record_length3);
    }

    skill.end_offset = skill.start_offset
        + skill.record_length1 as u64
        + SKILLDATA_RECORDSIZE_OFFSET;

    let string_size = r.read_word_be()? as usize;
    let raw         = r.read_bytes(string_size)?;
    skill.name = String::from_utf8_lossy(&raw).replace("\u{2026}", "...");

    skill.zero1 = r.read_byte()?;
    skill.zero2 = r.read_word_be()?;
    skill.zero3 = r.read_dword_be()?;
    skill.u1a   = r.read_word_be()?;
    skill.u1b   = r.read_word_be()?;
    skill.zero4 = r.read_dword_be()?;

    {
        let bd = &mut skill.base_data;
        bd.date_time          = r.read_dword_be()?;
        bd.caused_by          = r.read_dword_be()?;
        bd.scribing_index     = r.read_dword_be()?;
        bd.player_skill_index = r.read_dword_be()?;
        bd.skill_line_id      = r.read_dword_be()?;
        debug_assert!(SkillLine::from_id(&bd.skill_line_id).is_some() || bd.skill_line_id == 0, "skill_line_id not known: {} for ability id {}", bd.skill_line_id, skill.ability_id1);
        bd.maybe_idk          = r.read_dword_be()?;
        bd.uid                = r.read_dword64_be()?;
        bd.cast_time          = r.read_dword_be()?;
        bd.value0             = r.read_idword_be()?;
        bd.value1             = r.read_idword_be()?;
        bd.value2             = r.read_idword_be()?;
        bd.range              = r.read_dword_be()?;
        bd.ability_type       = r.read_byte()?;
        debug_assert!(AbilityType::from_id(&bd.ability_type).is_some(), "ability_type not known: {} for ability id {}", bd.ability_type, skill.ability_id1);
        bd.target_type        = r.read_word_be()?;
        bd.related_ability_id = r.read_dword_be()?;
        bd.cc_immunity_type   = r.read_byte()?;
        bd.crime_event        = r.read_byte()?;
        bd.duration           = r.read_dword_be()?;
        bd.z7                 = r.read_dword_be()?;
        bd.tick               = r.read_dword_be()?;
        bd.start_tick         = r.read_word_be()?;
        bd.cost               = r.read_word_be()?;
        bd.radius             = r.read_dword_be()?;
        bd.angle              = r.read_float_le()?;
        bd.u4                 = r.read_word_be()?;
        bd.u5                 = r.read_dword_be()?;
        bd.sprint_block_rule  = r.read_byte()?;
    }

    let flags_bytes = r.read_bytes(FLAGSIZE)?;
    skill.flags.copy_from_slice(&flags_bytes);

    skill.size1 = r.read_byte()? as u32;
    skill.link_types = (0..skill.size1)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;

    skill.size2      = r.read_byte()? as u32;
    skill.causes_ids = (0..skill.size2)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.u2a = r.read_byte()?;
    skill.u2b = r.read_dword_be()?;

    for slot in skill.u4.iter_mut() { *slot = r.read_word_be()?; }

    skill.size5 = r.read_byte()? as u32;
    skill.list5.reserve(skill.size5 as usize);
    for _ in 0..skill.size5 {
        let mut entry = U5Data::default();
        entry.uid    = r.read_dword64_be()?;
        entry.s5a = r.read_byte()? as u32;
        entry.d5a = (0..entry.s5a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.s5b = r.read_byte()? as u32;
        entry.d5b = (0..entry.s5b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list5.push(entry);
    }

    skill.num_tooltips = r.read_byte()? as u32;
    skill.tooltip_data.reserve(skill.num_tooltips as usize);
    for _ in 0..skill.num_tooltips {
        let mut td = TooltipData::default();
        td.uid               = r.read_dword64_be()?;
        td.num_tooltip_types = r.read_byte()? as u32;
        td.tooltip_types     = (0..td.num_tooltip_types)
            .map(|_| r.read_dword_be())
            .collect::<io::Result<_>>()?;
        debug_assert!(td.tooltip_types.iter().all(|f| TooltipType::from_id(f).is_some()), "Tooltip type in {:?} didn't match for id {}", td.tooltip_types, skill.ability_id1);
        td.num_tooltip_ids   = r.read_byte()? as u32;
        td.tooltip_ids       = (0..td.num_tooltip_ids)
            .map(|_| r.read_dword_be())
            .collect::<io::Result<_>>()?;
        skill.tooltip_data.push(td);
    }

    for slot in skill.u6.iter_mut() { *slot = r.read_byte()?; }

    skill.size6a = r.read_byte()? as u32;
    skill.list6a = (0..skill.size6a)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.size_tags = r.read_byte()? as u32;
    skill.ability_tags = (0..skill.size_tags)
        .map(|_| r.read_word_be())
        .collect::<io::Result<_>>()?;


    {
        let p = &mut skill.pre;
        p.u0 = r.read_word_be()?;
        p.u1 = r.read_float_le()?;
        p.u2 = r.read_float_le()?;
        p.u3 = r.read_dword_be()?;
        p.u4 = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
        p.u5 = r.read_dword_be()?;
        p.u6 = r.read_dword_be()?;
    }


    {
        let p = &mut skill.post;
        p.u1 = r.read_word_be()?;
        p.u2 = r.read_byte()?;
        p.u3 = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
        p.u4 = r.read_dword_be()?;
        p.u5 = r.read_byte()?;
        p.u6 = r.read_byte()?;
        p.u7 = r.read_dword_be()?;
    }

    skill.size7 = r.read_byte()? as u16;
    skill.list7 = (0..skill.size7 as u32)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.f8a = r.read_float_le()?;
    skill.icon_file_index = r.read_dword_be()?;
    skill.u8c = r.read_dword_be()?;

    skill.size8 = r.read_byte()? as u32;
    skill.list8 = (0..skill.size8)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.u9a = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
    skill.u9b = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;

    skill.size9 = r.read_byte()? as u32;
    skill.list9.reserve(skill.size9 as usize);
    for _ in 0..skill.size9 {
        let mut entry = List9Data::default();
        entry.uid    = r.read_dword64_be()?;
        entry.size9a = r.read_byte()? as u32;
        entry.list9a = (0..entry.size9a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.size9b = r.read_byte()? as u32;
        entry.list9b = (0..entry.size9b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list9.push(entry);
    }


    skill.half_width = r.read_word_be()?;
    for slot in skill.u10.iter_mut() { *slot = r.read_byte()?; }

    skill.size11 = r.read_byte()? as u32;
    skill.list11.reserve(skill.size11 as usize);
    for _ in 0..skill.size11 {
        let mut entry = List11Data::default();
        entry.uid     = r.read_dword64_be()?;
        entry.size11a = r.read_byte()? as u32;
        entry.list11a = (0..entry.size11a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.size11b = r.read_byte()? as u32;
        entry.list11b = (0..entry.size11b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list11.push(entry);
    }

    skill.action_slot = ActionSlotData {
        f2e0:             r.read_float_le()?,
        action_slot_type: r.read_byte()?,
        u2e8:             r.read_dword_be()?,
        u9:               r.read_word_be()?,
        u11:              r.read_byte()?,
    };
    for slot in skill.u12.iter_mut() { *slot = r.read_byte()?; }

    skill.size13 = r.read_byte()? as u32;
    skill.list13 = r.read_bytes(skill.size13 as usize * 6)?;

    skill.u13 = U13Data {
        u0:                       r.read_byte()?,
        default_always_never:     r.read_byte()?,
        u334:                     r.read_dword_be()?,
        sheathe_weapon_option:    r.read_byte()?,
        u33c_ability_id:          r.read_dword_be()?,
        f340:                     r.read_float_le()?,
        additional_telegraph_type: r.read_byte()?,
    };
    skill.major_minor_id  = r.read_byte()?;
    skill.u15 = U15Data {
        u34c_ability_id:    r.read_dword_be()?,
        ground_target_type: r.read_byte()?,
        f354:               r.read_float_le()?,
        bonus_rules_type:   r.read_byte()?,
        u35c:               r.read_dword_be()?,
        u14:                r.read_byte()?,
    };
    skill.u16 = r.read_byte()?;
    skill.u17  = r.read_dword_be()?;

    skill.u18 = U18Data {
        weapon_config_type: r.read_byte()?,
        f36c:               r.read_float_le()?,
        weapon_skill_line:  r.read_dword_be()?,
        u374_ability_id:    r.read_dword_be()?,
    };

    skill.size19 = r.read_byte()? as u32;
    skill.list19.reserve(skill.size19 as usize);
    debug_assert!(skill.size19 < 2, "size19 > 1");
    for _ in 0..skill.size19 {
        skill.list19.push(ExecuteData {
            u1:                         r.read_byte()?,
            threshold_below_health_pct: r.read_byte()?,
            bonus_up_to_pct:            r.read_word_be()?,
        });
    }
    skill.u20 = r.read_dword_be()?;

    skill.size21 = r.read_byte()? as u32;
    skill.list21 = r.read_bytes(skill.size21 as usize * 5)?;

    skill.u22 = U22Data {
        cc_immunity_visuals: r.read_byte()?,
        u39c_ability_id:     r.read_dword_be()?,
    };
    skill.mechanic = r.read_byte()?;
    debug_assert!(Mechanic::from_id(&skill.mechanic).is_some(), "mechanic not known: {} for ability id {}", skill.mechanic, skill.ability_id1);
    skill.u23      = r.read_byte()?;
    for slot in skill.u24.iter_mut() { *slot = r.read_byte()?; }
    skill.size25 = r.read_byte()? as u32;
    skill.list25.reserve((skill.size25 * 7) as usize);
    for _ in 0..skill.size25 {
        let mut e = List25Entry::default();
        e.base_ability_id = r.read_dword_be()?;
        e.u1 = r.read_byte()?;
        e.rank_index = r.read_byte()?;
        e.u2 = r.read_byte()?;
        skill.list25.push(e);
    }
    skill.size26 = r.read_byte()? as u32;
    skill.list26.reserve(skill.size26 as usize);
    for _ in 0..skill.size26 {
        let mut e = List26Data::default();
        e.u2    = (0..4).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        e.size3 = r.read_byte()? as u32;
        e.list3 = (0..e.size3).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list26.push(e);
    }

    skill.size_coef = r.read_byte()? as u32;
    skill.coef.reserve(skill.size_coef as usize);
    for _ in 0..skill.size_coef {
        let mut c = SkillCoef::default();
        c.type1 = r.read_byte()?;
        c.coef1 = r.read_float_le()?;
        c.type2 = r.read_byte()?;
        c.coef2 = r.read_float_le()?;
        c.type3 = r.read_byte()?;
        c.coef3 = r.read_float_le()?;
        c.type4 = r.read_byte()?;
        c.coef4 = r.read_float_le()?;
        c.u20   = r.read_dword_be()?;
        skill.coef.push(c);
    }

    skill.u27 = r.read_dword_be()?;

    let cur_pos = r.tell()?;
    if cur_pos > skill.end_offset {
        eprintln!(
            "\t{:08X}: #{:06}: Over read skill by {} bytes!",
            skill.start_offset, skill.index, cur_pos - skill.end_offset
        );
    } else if cur_pos < skill.end_offset {
        let remaining = r.read_bytes((skill.end_offset - cur_pos) as usize)?;
        eprintln!(
            "\t{:08X}: #{:06}: Under read skill by {} bytes: {:02X?}",
            skill.start_offset, skill.index, remaining.len(), remaining
        );
    }

    r.seek(skill.end_offset)?;
    Ok(skill)
}