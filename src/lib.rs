use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom};

use serde::Serialize;
use serde::ser::SerializeSeq;
use serde::Serializer;

use crate::enums::ability_type::AbilityType;
use crate::enums::mechanic::Mechanic;
use crate::enums::skill_line::SkillLine;
use crate::enums::tooltip_type::TooltipType;

pub mod enums;

pub fn serialize_array<S, T, const N: usize>(arr: &[T; N], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize,
{
    let mut seq = s.serialize_seq(Some(N))?;
    for b in arr.iter() {
        seq.serialize_element(b)?;
    }
    seq.end()
}

pub const FLAGSIZE:  usize = 176;
pub const U2ASIZE:   usize = 5;
pub const U4SIZE:    usize = 6;
pub const U6SIZE:    usize = 33;
pub const U8SIZE:    usize = 24;
pub const U10SIZE:   usize = 16;
pub const U11SIZE:   usize = 12;
pub const U12SIZE:   usize = 27;
pub const U13SIZE:   usize = 16;
pub const U15SIZE:   usize = 15;
pub const U18SIZE:   usize = 13;
pub const U22SIZE:   usize = 5;
pub const U24SIZE:   usize = 32;

const SKILLDATA_RECORDSIZE_OFFSET: u64 = 32;

pub fn is_zero_u8(v: &u8)       -> bool { *v == 0 }
pub fn is_zero_u16(v: &u16)     -> bool { *v == 0 }
pub fn is_zero_u32(v: &u32)     -> bool { *v == 0 }
pub fn is_zero_i32(v: &i32)     -> bool { *v == 0 }
pub fn is_zero_f32(v: &f32)     -> bool { *v == 0.0 }
pub fn is_empty_str(v: &String) -> bool { v.is_empty() }
pub fn is_empty_vec<T>(v: &Vec<T>) -> bool where T: Default + PartialEq { v.is_empty() || v.iter().all(|f| f == &T::default()) }
pub fn is_empty_array<T, const N: usize>(v: &[T; N]) -> bool where T: Default + PartialEq {v.iter().all(|f| f == &T::default())}
pub fn always_skip_serialization<T>(_v: &T) -> bool { true }

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct SkillBaseData {
    #[serde(skip_serializing_if = "is_zero_u32")] pub date_time:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub caused_by:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub scribing_index:     u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub player_skill_index: u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub skill_line_id:      u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub cast_time:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub value0:             u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub value1:             u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub value2:             u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub range:              u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub ability_type:       u8,
    #[serde(skip_serializing_if = "is_zero_u16")] pub u2:                 u16,
    #[serde(skip_serializing_if = "is_zero_u16")] pub z4:                 u16,
    #[serde(skip_serializing_if = "is_zero_u16")] pub z5:                 u16,
    #[serde(skip_serializing_if = "is_zero_u16")] pub z6:                 u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub duration:           u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub z7:                 u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub tick:               u32,
    #[serde(skip_serializing_if = "is_zero_u16")] pub start_tick:         u16,
    #[serde(skip_serializing_if = "is_zero_u16")] pub cost:               u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub radius:             u32,
    #[serde(skip_serializing_if = "is_zero_f32")] pub angle:              f32,
    #[serde(skip_serializing_if = "is_zero_u16")] pub u4:                 u16,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u5:                 u32,
    #[serde(skip_serializing_if = "is_zero_u8")]  pub u6:                 u8,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct PreSkillCoef {
    #[serde(skip_serializing_if = "is_zero_u32")] pub u1: u32,
    #[serde(skip_serializing_if = "is_zero_u32")] pub u2: u32,
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
    #[serde(skip_serializing_if = "is_zero_u8")]   pub u4: u8,
    #[serde(skip_serializing_if = "is_empty_vec")] pub u5: Vec<u8>,
    #[serde(skip_serializing_if = "is_zero_u8")]   pub u6: u8,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub u7: u32,
    #[serde(skip_serializing_if = "is_zero_u16")]  pub u8: u16,
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
    #[serde(skip_serializing_if = "is_zero_u32")]  pub s5a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub d5a: Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub s5b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub d5b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct TooltipData {
    #[serde(skip_serializing_if = "is_zero_u32")]  pub num_tooltip_types: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub tooltip_types:     Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]  pub num_tooltip_ids:   u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub tooltip_ids:       Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List9Data {
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size9a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list9a: Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size9b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list9b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List11Data {
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size11a: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list11a: Vec<u32>,
    #[serde(skip_serializing_if = "always_skip_serialization")]  pub size11b: u32,
    #[serde(skip_serializing_if = "is_empty_vec")] pub list11b: Vec<u32>,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct List19Data {
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

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct SkillData34 {
    #[serde(skip_serializing_if = "always_skip_serialization")] pub start_offset:   u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub end_offset:     u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub magic_header:   u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub index:          u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length1: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length2: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub unknown1:       u32,
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
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list1:          Vec<u8>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size2:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub causes_ids:     Vec<u32>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u2a: [u8; U2ASIZE],
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
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u8a:            Vec<u8>,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u8b:            Vec<u8>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u8c:            u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size8:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list8:          Vec<u32>,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u9a:            Vec<u8>,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub u9b:            Vec<u8>,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size9:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list9:          Vec<List9Data>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u10: [u8; U10SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size11:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list11:         Vec<List11Data>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u11: [u8; U11SIZE],
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u12: [u8; U12SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size13:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list13:         Vec<u8>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u13: [u8; U13SIZE],
    #[serde(skip_serializing_if = "is_zero_u8")]                pub major_minor_id: u8,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u15: [u8; U15SIZE],
    #[serde(skip_serializing_if = "is_zero_u8")]                pub u16:            u8,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u17:            u32,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u18: [u8; U18SIZE],
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size19:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list19:         Vec<List19Data>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub u20:            u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size21:         u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub list21:         Vec<u8>,
    #[serde(serialize_with = "serialize_array", skip_serializing_if = "is_empty_array")] pub u22: [u8; U22SIZE],
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
            list1:          Vec::new(),
            size2:          0,
            causes_ids:     Vec::new(),
            u2a:            [0u8; U2ASIZE],
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
            u8a:            Vec::new(),
            u8b:            Vec::new(),
            u8c:            0,
            size8:          0,
            list8:          Vec::new(),
            u9a:            Vec::new(),
            u9b:            Vec::new(),
            size9:          0,
            list9:          Vec::new(),
            u10:            [0u8; U10SIZE],
            size11:         0,
            list11:         Vec::new(),
            u11:            [0u8; U11SIZE],
            u12:            [0u8; U12SIZE],
            size13:         0,
            list13:         Vec::new(),
            u13:           [0u8; U13SIZE],
            major_minor_id:            0,
            u15:           [0u8; U15SIZE],
            u16:           0,
            u17:            0,
            u18:            [0u8; U18SIZE],
            size19:        0,
            list19:        Vec::new(),
            u20:           0,
            size21:         0,
            list21:         Vec::new(),
            u22:            [0u8; U22SIZE],
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
    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut r = ByteReader::new(std::io::Cursor::new(bytes));
        read_skill_record34_inner(&mut r, 0)
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

pub struct ByteReader<R: Read + Seek> {
    inner: R,
}

impl<R: Read + Seek> ByteReader<R> {
    pub fn new(inner: R) -> Self { Self { inner } }

    pub fn tell(&mut self) -> io::Result<u64> {
        self.inner.stream_position()
    }

    pub fn seek(&mut self, pos: u64) -> io::Result<()> {
        self.inner.seek(SeekFrom::Start(pos))?;
        Ok(())
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_word_be(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_word_le(&mut self) -> io::Result<u16> {
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_dword_be(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_dword_le(&mut self) -> io::Result<u32> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_float_le(&mut self) -> io::Result<f32> {
        let bits = self.read_dword_le()?;
        Ok(f32::from_bits(bits))
    }

    pub fn read_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; n];
        self.inner.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl ByteReader<std::fs::File> {
    pub fn open(filename: &str) -> io::Result<Self> {
        let f = std::fs::File::open(filename)?;
        Ok(Self::new(f))
    }

    pub fn file_size(&mut self) -> io::Result<u64> {
        let cur = self.tell()?;
        let end = self.inner.seek(SeekFrom::End(0))?;
        self.seek(cur)?;
        Ok(end)
    }
}

fn report_error(msg: &str) -> io::Error {
    eprintln!("{msg}");
    io::Error::new(io::ErrorKind::InvalidData, msg.to_string())
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
        bd.cast_time          = r.read_dword_be()?;
        bd.value0             = r.read_dword_be()?;
        bd.value1             = r.read_dword_be()?;
        bd.value2             = r.read_dword_be()?;
        bd.range              = r.read_dword_be()?;
        bd.ability_type       = r.read_byte()?;
        debug_assert!(AbilityType::from_id(&bd.ability_type).is_some(), "ability_type not known: {} for ability id {}", bd.ability_type, skill.ability_id1);
        bd.u2                 = r.read_word_be()?;
        bd.z4                 = r.read_word_be()?;
        bd.z5                 = r.read_word_be()?;
        bd.z6                 = r.read_word_be()?;
        bd.duration           = r.read_dword_be()?;
        bd.z7                 = r.read_dword_be()?;
        bd.tick               = r.read_dword_be()?;
        bd.start_tick         = r.read_word_be()?;
        bd.cost               = r.read_word_be()?;
        bd.radius             = r.read_dword_be()?;
        bd.angle              = r.read_float_le()?;
        bd.u4                 = r.read_word_be()?;
        bd.u5                 = r.read_dword_be()?;
        bd.u6                 = r.read_byte()?;
    }

    let flags_bytes = r.read_bytes(FLAGSIZE)?;
    skill.flags.copy_from_slice(&flags_bytes);

    skill.size1 = r.read_dword_be()?;
    skill.list1 = (0..skill.size1)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;

    skill.size2      = r.read_dword_be()?;
    skill.causes_ids = (0..skill.size2)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;


    let u2a = r.read_bytes(U2ASIZE)?;
    skill.u2a.copy_from_slice(&u2a);


    for slot in skill.u4.iter_mut() { *slot = r.read_word_be()?; }

    skill.size5 = r.read_dword_be()?;
    skill.list5.reserve(skill.size5 as usize);
    for _ in 0..skill.size5 {
        let mut entry = U5Data::default();
        entry.s5a = r.read_dword_be()?;
        entry.d5a = (0..entry.s5a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.s5b = r.read_dword_be()?;
        entry.d5b = (0..entry.s5b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list5.push(entry);
    }

    skill.num_tooltips = r.read_dword_be()?;
    skill.tooltip_data.reserve(skill.num_tooltips as usize);
    for _ in 0..skill.num_tooltips {
        let mut td = TooltipData::default();
        td.num_tooltip_types = r.read_dword_be()?;
        td.tooltip_types     = (0..td.num_tooltip_types)
            .map(|_| r.read_dword_be())
            .collect::<io::Result<_>>()?;
        debug_assert!(td.tooltip_types.iter().all(|f| TooltipType::from_id(f).is_some()), "Tooltip type in {:?} didn't match for id {}", td.tooltip_types, skill.ability_id1);
        td.num_tooltip_ids   = r.read_dword_be()?;
        td.tooltip_ids       = (0..td.num_tooltip_ids)
            .map(|_| r.read_dword_be())
            .collect::<io::Result<_>>()?;
        skill.tooltip_data.push(td);
    }

    for slot in skill.u6.iter_mut() { *slot = r.read_byte()?; }

    skill.size6a = r.read_dword_be()?;
    skill.list6a = (0..skill.size6a)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.size_tags = r.read_dword_be()?;
    skill.ability_tags = (0..skill.size_tags)
        .map(|_| r.read_word_be())
        .collect::<io::Result<_>>()?;


    {
        let p = &mut skill.pre;
        p.u1 = r.read_dword_be()?;
        p.u2 = r.read_dword_be()?;
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
        p.u4 = r.read_byte()?;
        p.u5 = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
        p.u6 = r.read_byte()?;
        p.u7 = r.read_dword_be()?;
        p.u8 = r.read_word_be()?;
    }

    skill.size7 = r.read_word_be()?;
    skill.list7 = (0..skill.size7 as u32)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.u8a = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
    skill.u8b = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
    skill.u8c = r.read_dword_be()?;

    skill.size8 = r.read_dword_be()?;
    skill.list8 = (0..skill.size8)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    skill.u9a = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;
    skill.u9b = (0..4)
        .map(|_| r.read_byte())
        .collect::<io::Result<_>>()?;

    skill.size9 = r.read_dword_be()?;
    skill.list9.reserve(skill.size9 as usize);
    for _ in 0..skill.size9 {
        let mut entry = List9Data::default();
        entry.size9a = r.read_dword_be()?;
        entry.list9a = (0..entry.size9a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.size9b = r.read_dword_be()?;
        entry.list9b = (0..entry.size9b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list9.push(entry);
    }


    for slot in skill.u10.iter_mut() { *slot = r.read_byte()?; }

    skill.size11 = r.read_dword_be()?;
    skill.list11.reserve(skill.size11 as usize);
    for _ in 0..skill.size11 {
        let mut entry = List11Data::default();
        entry.size11a = r.read_dword_be()?;
        entry.list11a = (0..entry.size11a).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        entry.size11b = r.read_dword_be()?;
        entry.list11b = (0..entry.size11b).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list11.push(entry);
    }

    for slot in skill.u11.iter_mut() { *slot = r.read_byte()?; }
    for slot in skill.u12.iter_mut() { *slot = r.read_byte()?; }

    skill.size13 = r.read_dword_be()?;
    skill.list13 = r.read_bytes(skill.size13 as usize * 6)?;

    for slot in skill.u13.iter_mut() { *slot = r.read_byte()?; }
    skill.major_minor_id  = r.read_byte()?;
    for slot in skill.u15.iter_mut() { *slot = r.read_byte()?; }
    skill.u16 = r.read_byte()?;
    skill.u17  = r.read_dword_be()?;

    for slot in skill.u18.iter_mut() { *slot = r.read_byte()?; }

    skill.size19 = r.read_dword_be()?;
    skill.list19.reserve(skill.size19 as usize);
    debug_assert!(skill.size19 < 2, "size19 > 1");
    for _ in 0..skill.size19 {
        skill.list19.push(List19Data {
            u1:                         r.read_byte()?,
            threshold_below_health_pct: r.read_byte()?,
            bonus_up_to_pct:            r.read_word_be()?,
        });
    }
    skill.u20 = r.read_dword_be()?;

    skill.size21 = r.read_dword_be()?;
    skill.list21 = r.read_bytes(skill.size21 as usize * 5)?;

    for slot in skill.u22.iter_mut() { *slot = r.read_byte()?; }
    skill.mechanic = r.read_byte()?;
    debug_assert!(Mechanic::from_id(&skill.mechanic).is_some(), "mechanic not known: {} for ability id {}", skill.mechanic, skill.ability_id1);
    skill.u23      = r.read_byte()?;
    for slot in skill.u24.iter_mut() { *slot = r.read_byte()?; }
    skill.size25 = r.read_dword_be()?;
    skill.list25.reserve((skill.size25 * 7) as usize);
    for _ in 0..skill.size25 {
        let mut e = List25Entry::default();
        e.base_ability_id = r.read_dword_be()?;
        e.u1 = r.read_byte()?;
        e.rank_index = r.read_byte()?;
        e.u2 = r.read_byte()?;
        skill.list25.push(e);
    }
    skill.size26 = r.read_dword_be()?;
    skill.list26.reserve(skill.size26 as usize);
    for _ in 0..skill.size26 {
        let mut e = List26Data::default();
        e.u2    = (0..4).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        e.size3 = r.read_dword_be()?;
        e.list3 = (0..e.size3).map(|_| r.read_dword_be()).collect::<io::Result<_>>()?;
        skill.list26.push(e);
    }

    skill.size_coef = r.read_dword_be()?;
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