use itertools::Itertools;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Difficulty {
    pub script_file_name: String,
    pub attribute: Option<DifficultyAttribute>,
    pub edition: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub is_ps4dlc: bool,
    pub level: Option<Level>,
    pub level_sort_index: Option<i32>,
    pub se_name: Option<String>,
    pub pvbranch_success_se_name: Option<String>,
    pub slide_name: Option<String>,
    pub chainslide_first_name: Option<String>,
    pub chainslide_sub_name: Option<String>,
    pub chainslide_success_name: Option<String>,
    pub chainslide_failure_name: Option<String>,
    pub slidertouch_name: Option<String>,
    pub motion: Option<Vec<String>>,
    #[serde(rename = "motion2P")]
    pub motion_2p: Option<Vec<String>>,
    #[serde(rename = "motion3P")]
    pub motion_3p: Option<Vec<String>>,
    #[serde(rename = "motion4P")]
    pub motion_4p: Option<Vec<String>>,
    #[serde(rename = "motion5P")]
    pub motion_5p: Option<Vec<String>>,
    #[serde(rename = "motion6P")]
    pub motion_6p: Option<Vec<String>>,
    pub npr: Option<Npr>,
    pub pv_item: Option<Vec<String>>,
    pub hand_item: Option<Vec<String>>,
    pub edit_effect: Option<Vec<String>>,
    pub edit_effect_low_field: Option<Vec<i32>>,
    pub title_image: Option<TitleImage>,
    pub songinfo: Option<SongInfo>,
    pub songinfo_en: Option<SongInfo>,
    pub movie_file_name: Option<String>,
    pub movie_surface: Option<MovieSurface>,
    pub movie_pv_type: Option<MoviePvType>,
    pub effect_se_file_name: Option<String>,
    pub version: Option<i32>,
    pub high_speed_rate: Option<f32>,
    pub hidden_timing: Option<f32>,
    pub sudden_timing: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub edit_chara_scale: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoviePvType {
    Only,
    Alternately,
    Parallel,
    Effect,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MovieSurface {
    Back,
    Front,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieList {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SongInfo {
    pub music: Option<String>,
    pub lyrics: Option<String>,
    pub arranger: Option<String>,
    pub manipulator: Option<String>,
    pub pv_editor: Option<String>,
    pub guitar_player: Option<String>,
    pub ex_info: Option<Vec<ExInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExInfo {
    pub key: String,
    pub val: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TitleImage {
    pub time: Option<f32>,
    pub end_time: Option<f32>,
    pub aet_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Npr {
    pub chara_lightness: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(non_camel_case_types)]
pub enum Level {
    PV_LV_00_0,
    PV_LV_00_5,
    PV_LV_01_0,
    PV_LV_01_5,
    PV_LV_02_0,
    PV_LV_02_5,
    PV_LV_03_0,
    PV_LV_03_5,
    PV_LV_04_0,
    PV_LV_04_5,
    PV_LV_05_0,
    PV_LV_05_5,
    PV_LV_06_0,
    PV_LV_06_5,
    PV_LV_07_0,
    PV_LV_07_5,
    PV_LV_08_0,
    PV_LV_08_5,
    PV_LV_09_0,
    PV_LV_09_5,
    PV_LV_10_0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DifficultyAttribute {
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub original: bool,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub extra: bool,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub slide: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Difficulties {
    pub attribute: Option<DifficultyAttribute>,
    pub easy: Option<Vec<Difficulty>>,
    pub normal: Option<Vec<Difficulty>>,
    pub hard: Option<Vec<Difficulty>>,
    pub extreme: Option<Vec<Difficulty>>,
    pub encore: Option<Vec<Difficulty>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mdata {
    pub flag: Option<i32>,
    pub dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sabi {
    pub start_time: Option<f32>,
    pub play_time: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Perfomer {
    #[serde(rename = "type")]
    pub performer_type: Option<PerformerType>,
    pub chara: Option<Chara>,
    pub costume: Option<i32>,
    pub pv_costume: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub fixed: bool,
    pub pseudo_same_id: Option<i32>,
    pub exclude: Option<i32>,
    pub size: Option<PerformerSize>,
    pub item_zujo: Option<i32>,
    pub item_face: Option<i32>,
    pub item_neck: Option<i32>,
    pub item_back: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PerformerSize {
    Normal,
    PlayChara,
    PvChara,
    Short,
    Tall,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PerformerType {
    Vocal,
    PseudoDefault,
    PseudoSame,
    PseudoSwim,
    PseudoSwimS,
    PseudoMyChara,
    Guest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Chara {
    #[serde(rename = "MIK")]
    Miku,
    #[serde(rename = "RIN")]
    Rin,
    #[serde(rename = "LEN")]
    Len,
    #[serde(rename = "LUK")]
    Luka,
    #[serde(rename = "NER")]
    Neru,
    #[serde(rename = "HAK")]
    Haku,
    #[serde(rename = "KAI")]
    Kaito,
    #[serde(rename = "MEI")]
    Meiko,
    #[serde(rename = "SAK")]
    Sakine,
    #[serde(rename = "TET")]
    Teto,
    #[serde(rename = "EXT")]
    Extra,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChrCam {
    pub id: Option<i32>,
    pub chara: Option<Chara>,
    pub org_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChrMot {
    pub id: Option<i32>,
    pub chara: Option<Chara>,
    pub org_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChrEffData {
    #[serde(rename = "type")]
    pub data_type: Option<ChrEffType>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChrEffType {
    #[serde(rename = "AUTH3D")]
    Auth3D,
    #[serde(rename = "AUTH3D_OBJ")]
    Auth3DObj,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChrEff {
    pub id: Option<i32>,
    pub name: Option<Chara>,
    pub data: Option<Vec<ChrEffData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EyesAdjust {
    Direction,
    Clearance,
    Off,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EyesRotRate {
    pub chara: Chara,
    pub xp_rate: f32,
    pub xn_rate: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExAuth {
    pub org_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExSong {
    pub chara: Chara,
    pub chara2: Option<Chara>,
    pub chara3: Option<Chara>,
    pub chara4: Option<Chara>,
    pub chara5: Option<Chara>,
    pub chara6: Option<Chara>,
    pub file: String,
    pub name: Option<String>,
    pub name_en: Option<String>,
    pub ex_auth: Option<Vec<ExAuth>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsageInit {
    pub motion: String,
    pub frame: i32,
    pub stage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageParam {
    pub stage: Option<String>,
    pub mhd_id: Option<i32>,
    pub collision_file: Option<String>,
    pub wind_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Disp2D {
    pub set_name: Option<String>,
    pub target_shadow_type: Option<i32>,
    pub title_start_2d_field: Option<i32>,
    pub title_end_2d_field: Option<i32>,
    pub title_start_2d_low_field: Option<i32>,
    pub title_end_2d_low_field: Option<i32>,
    pub title_start_3d_field: Option<i32>,
    pub title_end_3d_field: Option<i32>,
    pub title_2d_layer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PvExpression {
    pub file_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnotherSong {
    pub name: Option<String>,
    pub name_en: Option<String>,
    pub song_file_name: Option<String>,
    pub vocal_disp_name: Option<String>,
    pub vocal_disp_name_en: Option<String>,
    pub vocal_chara_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FrameTextureType {
    PrePp,
    PostPp,
    Fb,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthReplaceByModule {
    pub id: Option<i32>,
    pub module_id: Option<i32>,
    pub org_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub song_name: String,
    pub song_name_en: String,
    pub song_name_reading: String,
    pub song_name_reading_en: Option<String>,
    pub bpm: i32,
    pub song_file_name: String,
    pub date: i32,
    pub difficulty: Option<Difficulties>,
    pub remix_parent: Option<i32>,
    pub mdata: Option<Mdata>,
    pub lyric: Option<Vec<String>>,
    pub lyric_en: Option<Vec<String>>,
    pub sabi: Option<Sabi>,
    pub edit: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub disable_calc_motfrm_limit: Option<bool>,
    pub performer: Option<Vec<Perfomer>>,
    pub chrcam: Option<Vec<ChrCam>>,
    pub chrmot: Option<Vec<ChrMot>>,
    pub chreff: Option<Vec<ChrEff>>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub eyes_xrot_adjust: bool,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub is_old_pv: bool,
    pub eyes_base_adjust_type: Option<EyesAdjust>,
    pub eyes_rot_rate: Option<Vec<EyesRotRate>>,
    pub se_name: Option<String>,
    pub pvbranch_success_se_name: Option<String>,
    pub slide_name: Option<String>,
    pub chainslide_first_name: Option<String>,
    pub chainslide_sub_name: Option<String>,
    pub chainslide_success_name: Option<String>,
    pub chainslide_failure_name: Option<String>,
    pub slidertouch_name: Option<String>,
    pub motion: Option<Vec<String>>,
    #[serde(rename = "motion2P")]
    pub motion_2p: Option<Vec<String>>,
    #[serde(rename = "motion3P")]
    pub motion_3p: Option<Vec<String>>,
    #[serde(rename = "motion4P")]
    pub motion_4p: Option<Vec<String>>,
    #[serde(rename = "motion5P")]
    pub motion_5p: Option<Vec<String>>,
    #[serde(rename = "motion6P")]
    pub motion_6p: Option<Vec<String>>,
    pub npr: Option<Npr>,
    pub pv_item: Option<Vec<String>>,
    pub hand_item: Option<Vec<String>>,
    pub edit_effect: Option<Vec<String>>,
    pub edit_effect_low_field: Option<Vec<i32>>,
    pub title_image: Option<TitleImage>,
    pub songinfo: Option<SongInfo>,
    pub songinfo_en: Option<SongInfo>,
    pub movie_list: Option<Vec<MovieList>>,
    pub movie_file_name: Option<String>,
    pub movie_surface: Option<MovieSurface>,
    pub movie_pv_type: Option<MoviePvType>,
    pub effect_se_file_name: Option<String>,
    pub effect_se_name_list: Option<Vec<String>>,
    pub high_speed_rate: Option<f32>,
    pub hidden_timing: Option<f32>,
    pub sudden_timing: Option<f32>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub edit_chara_scale: bool,
    pub ex_song: Option<Vec<ExSong>>,
    pub osage_init: Option<Vec<OsageInit>>,
    pub stage_param: Option<Vec<StageParam>>,
    pub disp2d: Option<Disp2D>,
    #[serde(default)]
    pub use_osage_play_data: bool,
    pub pv_expression: Option<PvExpression>,
    pub another_song: Option<Vec<AnotherSong>>,
    #[serde(default)]
    #[serde(deserialize_with = "diva_bool")]
    pub pre_play_script: bool,
    pub frame_texture: Option<String>,
    pub frame_texture_a: Option<String>,
    pub frame_texture_b: Option<String>,
    pub frame_texture_c: Option<String>,
    pub frame_texture_d: Option<String>,
    pub frame_texture_e: Option<String>,
    pub frame_texture_type: Option<FrameTextureType>,
    pub frame_texture_a_type: Option<FrameTextureType>,
    pub frame_texture_b_type: Option<FrameTextureType>,
    pub frame_texture_c_type: Option<FrameTextureType>,
    pub frame_texture_d_type: Option<FrameTextureType>,
    pub frame_texture_e_type: Option<FrameTextureType>,
    pub auth_replace_by_module: Option<Vec<AuthReplaceByModule>>,
    pub pack: Option<i32>,
    pub rank_board_id: Option<i32>,
    pub resolution_scale: Option<f32>,
    pub resolution_scale_neo: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct PvDb {
    pub pvs: BTreeMap<u32, Entry>,
}

struct PvDbVisitor {
    marker: std::marker::PhantomData<fn() -> PvDb>,
}

impl PvDbVisitor {
    fn new() -> Self {
        Self {
            marker: std::marker::PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for PvDbVisitor {
    type Value = PvDb;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("pv_%03d")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
        let mut pvs = BTreeMap::new();
        loop {
            let entry = access.next_entry::<&str, Entry>();
            let Ok(entry) = entry else {
                continue;
            };
            let Some((key, value)) = entry else {
                break;
            };
            let Ok(id) = key.trim_start_matches("pv_").parse::<u32>() else {
                continue;
            };
            pvs.insert(id, value);
        }

        Ok(Self::Value { pvs })
    }
}

impl<'de> Deserialize<'de> for PvDb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(PvDbVisitor::new())
    }
}

fn diva_bool<'de, D: serde::Deserializer<'de>, T: From<bool>>(
    deserializer: D,
) -> Result<T, D::Error> {
    Ok((Option::<i32>::deserialize(deserializer)? == Some(1)).into())
}

impl PvDb {
    pub fn parse<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        let input = std::fs::read_to_string(path).ok()?;
        let input = input.lines().dedup().collect::<Vec<_>>().join("\n");
        serde_divatree::from_str::<Self>(&input).ok()
    }

    pub fn from_str(str: &str) -> Option<Self> {
        let input = str.lines().dedup().collect::<Vec<_>>().join("\n");
        serde_divatree::from_str::<Self>(&input).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_base() {
        let input = std::fs::read_to_string("/home/brogamer/pv_db.txt").unwrap();
        let db = serde_divatree::from_str::<PvDb>(&input).unwrap();
        let ids = db.pvs.into_keys().collect::<Vec<_>>();
        let expected = vec![
            1, 2, 5, 7, 8, 9, 10, 12, 13, 14, 16, 22, 28, 30, 31, 32, 37, 38, 39, 40, 41, 42, 43,
            44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 65, 66, 68,
            79, 81, 82, 83, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 102, 103, 104, 201,
            202, 212, 213, 215, 218, 219, 220, 221, 222, 224, 225, 226, 227, 228, 231, 232, 234,
            238, 239, 240, 241, 242, 243, 244, 246, 247, 248, 249, 250, 251, 253, 254, 255, 257,
            259, 260, 261, 262, 263, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276,
            277, 278, 279, 280, 281, 433, 435, 600, 601, 602, 603, 604, 605, 607, 608, 609, 610,
            611, 612, 613, 614, 615, 616, 617, 619, 620, 621, 622, 623, 624, 625, 626, 627, 628,
            629, 630, 631, 637, 638, 639, 640, 641, 642, 710, 722, 723, 724, 725, 726, 727, 728,
            729, 730, 731, 732, 733, 734, 736, 737, 738, 739, 740, 832,
        ];
        assert_eq!(ids, expected);
    }

    #[test]
    fn read_mdata() {
        let input = std::fs::read_to_string("/home/brogamer/mdata_pv_db.txt").unwrap();
        let input = input.lines().dedup().collect::<Vec<_>>().join("\n");
        let db = serde_divatree::from_str::<PvDb>(&input).unwrap();
        let ids = db.pvs.into_keys().collect::<Vec<_>>();
        let expected = vec![
            3, 4, 5, 6, 11, 15, 17, 18, 20, 21, 23, 24, 25, 29, 63, 64, 67, 84, 101, 203, 204, 205,
            206, 208, 209, 210, 211, 214, 216, 223, 233, 235, 236, 401, 402, 403, 404, 405, 407,
            408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 420, 421, 422, 423, 424,
            425, 426, 427, 428, 429, 430, 431, 432, 434, 436, 437, 438, 439, 440, 441, 442, 443,
            618,
        ];
        assert_eq!(ids, expected);
    }
}
