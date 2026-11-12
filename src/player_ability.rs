use std::collections::HashMap;
use std::io::{self, Read, Seek};

use serde::Serialize;
use eso_common::*;
use crate::enums::skill_line::SkillLine;

pub const PLAYER_ABILITY_MANIFEST_HEX: u32 = 0xAA;
pub const PA_RECORDSIZE_OFFSET: u64 = 32;
pub const PA_HEADER_SIZE: u64 = 36;
const PA_FILE_MAGIC:   u32 = 0xFAFA_EBEB;
const PA_RECORD_MAGIC: u32 = 0x2323_2323;
const PA_UNKNOWN3_START:  u32 = 0x10;
const PA_UNKNOWN3_STRIDE: u32 = 12;
pub const PA_XP_NONE:     u32 = 999_999_999;
pub const PA_XP_NONE_ALT: u32 =  99_999_999;

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct PlayerAbilityEntry {
    #[serde(skip_serializing_if = "is_zero_u32")]               pub ability_id:    u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub zero1:         u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size1:         u32,
    /// Cumulative XP needed per rank. Passives and unused slots are all zero. 
    /// Both morphs of a skill share the same curve.
    #[serde(skip_serializing_if = "is_empty_vec")]              pub xp_thresholds: Vec<u32>,
}

impl PlayerAbilityEntry {
    /// Last (highest) XP threshold, 0 when the entry has none.
    pub fn max_xp(&self) -> u32 { self.xp_thresholds.last().copied().unwrap_or(0) }
}

#[allow(clippy::ptr_arg)]
fn is_empty_entries(v: &Vec<PlayerAbilityEntry>) -> bool { v.is_empty() }

#[derive(Debug, Default, Serialize, PartialEq, Clone)]
pub struct PlayerAbilityData {
    #[serde(skip_serializing_if = "always_skip_serialization")] pub start_offset:       u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub end_offset:         u64,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub magic_header:       u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub index:              u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length1:     u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length2:     u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown1:           u32,
    /// Player skill index (1..1083); equals `player_skill_index` of the abilities below.
    #[serde(skip_serializing_if = "is_zero_u32")]               pub skill_id1:          u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub unknown3:           u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub record_length3:     u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub skill_id2:          u32,
    #[serde(skip_serializing_if = "is_empty_str")]              pub name:               String, // always empty
    #[serde(skip_serializing_if = "is_zero_u8")]                pub zero1:              u8,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub zero2:              u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero3:              u32,
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1a:                u16,
    /// Observed: 182 on passives; on actives 0, except the Warden (60), Necromancer (88) and
    /// Arcanist (138) skill lines. Exact meaning unknown.
    #[serde(skip_serializing_if = "is_zero_u16")]               pub u1b:                u16,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero4:              u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub date_time:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub zero5:              u32,
    /// Localised name string id: skill_id1 + 1_616_040 for actives, + 1_616_039 for passives.
    #[serde(skip_serializing_if = "is_zero_u32")]               pub string_id:          u32,
    #[serde(skip_serializing_if = "is_zero_u32")]               pub total_xp:           u32,
    pub base:                                                                           PlayerAbilityEntry,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size_morphs:        u32,
    /// Always 2 morphs. For passives the ids are the following rank abilities (xp all zero).
    #[serde(skip_serializing_if = "is_empty_entries")]          pub morphs:             Vec<PlayerAbilityEntry>,
    /// Matches `skill_line_id` in the base_data of the base ability, see `SkillLine`.
    #[serde(skip_serializing_if = "is_zero_u32")]               pub skill_line_id:      u32,
    /// Companion ability of the skill (often named "<skill> Dummy"); shares this skill's index.
    #[serde(skip_serializing_if = "is_zero_u32")]               pub related_ability_id: u32,
    #[serde(skip_serializing_if = "always_skip_serialization")] pub size_links:         u32,
    /// 0..6 unique ids in 1..238, each used by exactly one skill in the whole file.
    /// Not ability, tag, skill line or skill ids; what they reference is still unresolved.
    #[serde(skip_serializing_if = "is_empty_vec")]              pub link_ids:           Vec<u32>,
}

impl PlayerAbilityData {
    pub fn id(&self) -> u32 { self.skill_id1 }

    pub fn is_active(&self)  -> bool { self.total_xp != 0 && self.total_xp < PA_XP_NONE_ALT }

    pub fn is_passive(&self) -> bool { self.total_xp >= PA_XP_NONE_ALT }

    pub fn base_ability_id(&self) -> u32 { self.base.ability_id }

    pub fn morph_ability_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.morphs.iter().map(|m| m.ability_id)
    }

    pub fn all_ability_ids(&self) -> impl Iterator<Item = u32> + '_ {
        std::iter::once(self.base.ability_id)
            .chain(self.morphs.iter().map(|m| m.ability_id))
            .chain(std::iter::once(self.related_ability_id))
            .filter(|&id| id != 0)
    }

    pub fn skill_line(&self) -> Option<SkillLine> { SkillLine::from_id(&self.skill_line_id) }

    pub fn last_edited_timestamp(&self) -> i64 { self.date_time as i64 }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut r = ByteReader::new(std::io::Cursor::new(bytes));
        read_player_ability_record_inner(&mut r, 0)
    }
}

#[derive(Debug, Serialize)]
pub struct PlayerAbilityParserState {
    pub size_of_file:       u64,
    pub magic_header:       u32,
    pub unknown1_header:    u32,
    pub num_records_header: u32,
    pub unknown2_header:    u32,
    pub record_index:       u32,
    pub next_unknown3:      u32,
    pub valid_skill_ids:    HashMap<u32, u32>,
    pub player_abilities:   Vec<PlayerAbilityData>,
}

impl PlayerAbilityParserState {
    pub fn new() -> Self {
        Self {
            size_of_file:       0,
            magic_header:       0,
            unknown1_header:    0,
            num_records_header: 0,
            unknown2_header:    0,
            record_index:       0,
            next_unknown3:      PA_UNKNOWN3_START,
            valid_skill_ids:    HashMap::new(),
            player_abilities:   Vec::new(),
        }
    }

    pub fn by_skill_id(&self, skill_id: u32) -> Option<&PlayerAbilityData> {
        self.player_abilities.iter().find(|p| p.skill_id1 == skill_id)
    }

    pub fn by_ability_id(&self, ability_id: u32) -> Option<&PlayerAbilityData> {
        if ability_id == 0 { return None; }
        self.player_abilities.iter().find(|p| p.all_ability_ids().any(|id| id == ability_id))
    }

    pub fn by_skill_line(&self, skill_line_id: u32) -> impl Iterator<Item = &PlayerAbilityData> + '_ {
        self.player_abilities.iter().filter(move |p| p.skill_line_id == skill_line_id)
    }
}

impl Default for PlayerAbilityParserState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_player_ability_data(filename: &str) -> io::Result<PlayerAbilityParserState> {
    let mut state = PlayerAbilityParserState::new();
    eprintln!("Loading and parsing player ability data file '{filename}'...");

    let mut file = ByteReader::open(filename)
        .map_err(|_| report_error(&format!("Error: Failed to open file '{filename}'!")))?;

    state.size_of_file       = file.file_size()?;
    state.magic_header       = file.read_dword_be()?;
    state.unknown1_header    = file.read_dword_be()?;
    state.num_records_header = file.read_dword_be()?;
    state.unknown2_header    = file.read_dword_be()?;

    if state.magic_header != PA_FILE_MAGIC {
        return Err(report_error(&format!(
            "Error: Magic header 0x{:08X} not expected value!", state.magic_header
        )));
    }

    read_player_ability_records(&mut file, &mut state)?;

    if state.player_abilities.len() != state.num_records_header as usize {
        eprintln!(
            "Record count mismatch: header says {}, read {}",
            state.num_records_header, state.player_abilities.len()
        );
    }
    Ok(state)
}

fn read_player_ability_records<R: Read + Seek>(
    file: &mut ByteReader<R>,
    state: &mut PlayerAbilityParserState,
) -> io::Result<()> {
    loop {
        let remaining = state.size_of_file.saturating_sub(file.tell()?);
        if remaining < PA_HEADER_SIZE {
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
        let pa = read_player_ability_record_inner(file, state.record_index + 1)?;
        if pa.unknown3 != state.next_unknown3 {
            eprintln!(
                "\t{:08X}: #{:06}: Unknown3 offset mismatch: 0x{:08X} != 0x{:08X}",
                pa.start_offset, pa.index, pa.unknown3, state.next_unknown3
            );
        }
        state.next_unknown3 = pa.unknown3
            .wrapping_add(pa.unknown1)
            .wrapping_add(PA_UNKNOWN3_STRIDE);
        state.record_index = pa.index;
        state.valid_skill_ids.insert(pa.skill_id1, state.record_index);
        state.player_abilities.push(pa);
    }
    Ok(())
}

fn read_player_ability_entry<R: Read + Seek>(r: &mut ByteReader<R>) -> io::Result<PlayerAbilityEntry> {
    let mut e = PlayerAbilityEntry::default();

    e.ability_id = r.read_dword_be()?;
    e.zero1      = r.read_dword_be()?;
    e.size1      = r.read_byte()? as u32;
    e.xp_thresholds = (0..e.size1)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    Ok(e)
}

fn read_player_ability_record_inner<R: Read + Seek>(r: &mut ByteReader<R>, expected_index: u32) -> io::Result<PlayerAbilityData> {
    let mut pa = PlayerAbilityData::default();

    pa.start_offset   = r.tell()?;
    pa.magic_header   = r.read_dword_be()?;
    pa.index          = r.read_dword_be()?;
    pa.record_length1 = r.read_dword_be()?;
    pa.record_length2 = r.read_dword_be()?;
    pa.unknown1       = r.read_dword_be()?;

    pa.skill_id1      = r.read_dword_be()?;
    pa.unknown3       = r.read_dword_be()?;
    pa.record_length3 = r.read_dword_be()?;
    pa.skill_id2      = r.read_dword_be()?;
    debug_assert!(pa.skill_id1 == pa.skill_id2, "skill_id1 and skill_id2 are not equal");

    if pa.magic_header != PA_RECORD_MAGIC {
        return Err(report_error(&format!(
            "Error: Player ability header 0x{:08X} not expected value!", pa.magic_header
        )));
    }
    if expected_index != 0 && pa.index != expected_index {
        eprintln!("Player Ability Index Mismatch: {} != {}", pa.index, expected_index);
    }
    if pa.record_length1 != pa.record_length2 || pa.record_length1 != pa.record_length3 {
        eprintln!(
            "Record Length Mismatch: 0x{:08X} 0x{:08X} 0x{:08X}",
            pa.record_length1, pa.record_length2, pa.record_length3
        );
    }

    pa.end_offset = pa.start_offset
        + pa.record_length1 as u64
        + PA_RECORDSIZE_OFFSET;

    let string_size = r.read_word_be()? as usize;
    let raw         = r.read_bytes(string_size)?;
    pa.name = String::from_utf8_lossy(&raw).replace("\u{2026}", "...");

    pa.zero1 = r.read_byte()?;
    pa.zero2 = r.read_word_be()?;
    pa.zero3 = r.read_dword_be()?;
    pa.u1a   = r.read_word_be()?;
    pa.u1b   = r.read_word_be()?;
    pa.zero4 = r.read_dword_be()?;

    pa.date_time = r.read_dword_be()?;
    pa.zero5     = r.read_dword_be()?;
    pa.string_id = r.read_dword_be()?;
    pa.total_xp  = r.read_dword_be()?;

    pa.base = read_player_ability_entry(r)?;

    pa.size_morphs = r.read_byte()? as u32;
    pa.morphs = (0..pa.size_morphs)
        .map(|_| read_player_ability_entry(&mut *r))
        .collect::<io::Result<_>>()?;

    pa.skill_line_id      = r.read_dword_be()?;
    pa.related_ability_id = r.read_dword_be()?;

    pa.size_links = r.read_byte()? as u32;
    pa.link_ids = (0..pa.size_links)
        .map(|_| r.read_dword_be())
        .collect::<io::Result<_>>()?;

    let cur_pos = r.tell()?;
    if cur_pos > pa.end_offset {
        eprintln!(
            "\t{:08X}: #{:06}: Over read player ability by {} bytes!",
            pa.start_offset, pa.index, cur_pos - pa.end_offset
        );
    } else if cur_pos < pa.end_offset {
        let remaining = r.read_bytes((pa.end_offset - cur_pos) as usize)?;
        eprintln!(
            "\t{:08X}: #{:06}: Under read player ability by {} bytes: {:02X?}",
            pa.start_offset, pa.index, remaining.len(), remaining
        );
    }

    r.seek(pa.end_offset)?;
    Ok(pa)
}