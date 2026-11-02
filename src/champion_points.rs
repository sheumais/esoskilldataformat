use std::collections::HashMap;
use std::io::{self, Read, Seek};

use serde::Serialize;
use eso_common::*;

pub const CP_RECORDSIZE_OFFSET: u64 = 32;
pub const CP_HEADER_SIZE: u64 = 36;

pub const CP_MANIFEST_HEX: u32 = 0x1AB;

const CP_FILE_MAGIC:   u32 = 0xFAFA_EBEB;
const CP_RECORD_MAGIC: u32 = 0x2323_2323;

pub const CP_PERK_PASSIVE:     u8 = 0;
pub const CP_PERK_ACTIVE:      u8 = 1;
pub const CP_PERK_BASE_ACTIVE: u8 = 2;

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct ChampionExtraData {
    #[serde(skip_serializing_if = "is_zero_u64")]               pub uid:      u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub u1:       u8,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub value:    u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub u2:       u8,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub skill_id: u32,
}

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct ChampionPointData {
    #[serde(skip_serializing_if = "always_skip_serialization")] pub start_offset:   u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub end_offset:     u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub magic_header:   u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub index:          u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length1: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length2: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown1:       u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub skill_id1:      u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown3:       u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length3: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub skill_id2:      u32,
    #[serde(skip_serializing_if = "is_empty_str")]              pub name:           String,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub zero1:          u8,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub zero2:          u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero3:          u32,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1a:            u16,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1b:            u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero4:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub date_time:      u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub ability_id:     u32,
    #[serde(skip_serializing_if = "is_zero_f32")]               pub pos_x:          f32,
    #[serde(skip_serializing_if = "is_zero_f32")]               pub pos_y:          f32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size1:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub link_ids:       Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u8")]                pub u2a:            u8, 
    #[serde(skip_serializing_if = "is_zero_u8")]                pub perk_type:      u8, 
    #[serde(skip_serializing_if = "is_zero_u64")]               pub ref_uid:        u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size2:          u32,
    #[serde(skip_serializing_if = "is_empty_vec")]              pub link_ids2:      Vec<u32>,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub max_stages:     u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub cost_per_stage: u32,
    #[serde(skip_serializing_if = "is_zero_f32")]               pub value_per_stage:f32,
    #[serde(skip_serializing_if = "is_zero_u64")]               pub uid:            u64,
    #[serde(skip_serializing_if = "Option::is_none")]           pub extra:          Option<ChampionExtraData>,
    #[serde(skip_serializing_if = "is_zero_u64")]               pub u4:             u64,
}

impl ChampionPointData {
    pub fn id(&self) -> u32 { self.skill_id1 }

    pub fn is_passive(&self)     -> bool { self.perk_type == CP_PERK_PASSIVE }
    pub fn is_active(&self)      -> bool { self.perk_type == CP_PERK_ACTIVE }
    pub fn is_base_active(&self) -> bool { self.perk_type == CP_PERK_BASE_ACTIVE }

   
    pub fn max_points(&self) -> u32 { self.max_stages * self.cost_per_stage }

   
    pub fn max_value(&self) -> f32 { self.max_stages as f32 * self.value_per_stage }

    pub fn last_edited_timestamp(&self) -> i64 { self.date_time as i64 }

   
    pub fn all_link_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.link_ids.iter().chain(self.link_ids2.iter()).copied()
    }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut r = ByteReader::new(std::io::Cursor::new(bytes));
        read_champion_point_record_inner(&mut r, 0)
    }
}

#[derive(Debug, Serialize)]
pub struct ChampionParserState {
    pub size_of_file:       u64,
    pub magic_header:       u32,
    pub unknown1_header:    u32,
    pub num_records_header: u32,
    pub unknown2_header:    u32,
    pub record_index:       u32,
    pub valid_skill_ids:    HashMap<u32, u32>,
    pub champion_points:    Vec<ChampionPointData>,
}

impl ChampionParserState {
    pub fn new() -> Self {
        Self {
            size_of_file:       0,
            magic_header:       0,
            unknown1_header:    0,
            num_records_header: 0,
            unknown2_header:    0,
            record_index:       0,
            valid_skill_ids:    HashMap::new(),
            champion_points:    Vec::new(),
        }
    }

    pub fn by_skill_id(&self, skill_id: u32) -> Option<&ChampionPointData> {
        self.champion_points.iter().find(|c| c.skill_id1 == skill_id)
    }

   
    pub fn by_ability_id(&self, ability_id: u32) -> Option<&ChampionPointData> {
        self.champion_points.iter().find(|c| c.ability_id == ability_id)
    }
}

impl Default for ChampionParserState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_champion_point_data(filename: &str) -> io::Result<ChampionParserState> {
    let mut state = ChampionParserState::new();
    eprintln!("Loading and parsing champion point data file '{filename}'...");

    let mut file = ByteReader::open(filename)
        .map_err(|_| report_error(&format!("Error: Failed to open file '{filename}'!")))?;

    state.size_of_file       = file.file_size()?;
    state.magic_header       = file.read_dword_be()?;
    state.unknown1_header    = file.read_dword_be()?;
    state.num_records_header = file.read_dword_be()?;
    state.unknown2_header    = file.read_dword_be()?;

    if state.magic_header != CP_FILE_MAGIC {
        return Err(report_error(&format!(
            "Error: Magic header 0x{:08X} not expected value!", state.magic_header
        )));
    }

    read_champion_point_records(&mut file, &mut state)?;

    if state.champion_points.len() != state.num_records_header as usize {
        eprintln!(
            "Record count mismatch: header says {}, read {}",
            state.num_records_header, state.champion_points.len()
        );
    }
    Ok(state)
}

fn read_champion_point_records<R: Read + Seek>(
    file: &mut ByteReader<R>,
    state: &mut ChampionParserState,
) -> io::Result<()> {
    loop {
        let remaining = state.size_of_file.saturating_sub(file.tell()?);
        if remaining < CP_HEADER_SIZE {
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
        let cp = read_champion_point_record_inner(file, state.record_index + 1)?;
        state.record_index = cp.index;
        state.valid_skill_ids.insert(cp.skill_id1, state.record_index);
        state.champion_points.push(cp);
    }
    Ok(())
}

fn read_champion_point_record_inner<R: Read + Seek>(r: &mut ByteReader<R>, expected_index: u32) -> io::Result<ChampionPointData> {
    let mut cp = ChampionPointData::default();

    cp.start_offset   = r.tell()?;
    cp.magic_header   = r.read_dword_be()?;
    cp.index          = r.read_dword_be()?;
    cp.record_length1 = r.read_dword_be()?;
    cp.record_length2 = r.read_dword_be()?;
    cp.unknown1       = r.read_dword_be()?;

    cp.skill_id1      = r.read_dword_be()?;
    cp.unknown3       = r.read_dword_be()?;
    cp.record_length3 = r.read_dword_be()?;
    cp.skill_id2      = r.read_dword_be()?;
    debug_assert!(cp.skill_id1 == cp.skill_id2, "skill_id1 and skill_id2 are not equal");

    if cp.magic_header != CP_RECORD_MAGIC {
        return Err(report_error(&format!(
            "Error: Champion point header 0x{:08X} not expected value!", cp.magic_header
        )));
    }
    if expected_index != 0 && cp.index != expected_index {
        eprintln!("Champion Point Index Mismatch: {} != {}", cp.index, expected_index);
    }
    if cp.record_length1 != cp.record_length2 || cp.record_length1 != cp.record_length3 {
        eprintln!(
            "Record Length Mismatch: 0x{:08X} 0x{:08X} 0x{:08X}",
            cp.record_length1, cp.record_length2, cp.record_length3
        );
    }

    cp.end_offset = cp.start_offset
        + cp.record_length1 as u64
        + CP_RECORDSIZE_OFFSET;

    let string_size = r.read_word_be()? as usize;
    let raw         = r.read_bytes(string_size)?;
    cp.name = String::from_utf8_lossy(&raw).replace("\u{2026}", "...");

    cp.zero1 = r.read_byte()?;
    cp.zero2 = r.read_word_be()?;
    cp.zero3 = r.read_dword_be()?;
    cp.u1a   = r.read_word_be()?;
    cp.u1b   = r.read_word_be()?;
    cp.zero4 = r.read_dword_be()?;

    cp.date_time  = r.read_dword_be()?;
    cp.ability_id = r.read_dword_be()?;
    cp.pos_x      = r.read_float_le()?;
    cp.pos_y      = r.read_float_le()?;

    cp.size1 = r.read_byte()? as u32;
    cp.link_ids = (0..cp.size1)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    cp.u2a       = r.read_byte()?;
    cp.perk_type = r.read_byte()?;
    cp.ref_uid   = r.read_dword64_be()?;

    cp.size2 = r.read_byte()? as u32;
    cp.link_ids2 = (0..cp.size2)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    cp.max_stages      = r.read_dword_be()?;
    cp.cost_per_stage  = r.read_dword_be()?;
    cp.value_per_stage = r.read_float_le()?;
    cp.uid = r.read_dword64_be()?;

    let has_extra = r.read_byte()?;
    debug_assert!(has_extra < 2, "has_extra > 1 for skill id {}", cp.skill_id1);
    if has_extra != 0 {
        cp.extra = Some(ChampionExtraData {
            uid:      r.read_dword64_be()?,
            u1:       r.read_byte()?,
            value:    r.read_dword_be()?,
            u2:       r.read_byte()?,
            skill_id: r.read_dword_be()?,
        });
    }

    cp.u4 = r.read_dword64_be()?;

    let cur_pos = r.tell()?;
    if cur_pos > cp.end_offset {
        eprintln!(
            "\t{:08X}: #{:06}: Over read champion point by {} bytes!",
            cp.start_offset, cp.index, cur_pos - cp.end_offset
        );
    } else if cur_pos < cp.end_offset {
        let remaining = r.read_bytes((cp.end_offset - cur_pos) as usize)?;
        eprintln!(
            "\t{:08X}: #{:06}: Under read champion point by {} bytes: {:02X?}",
            cp.start_offset, cp.index, remaining.len(), remaining
        );
    }

    r.seek(cp.end_offset)?;
    Ok(cp)
}