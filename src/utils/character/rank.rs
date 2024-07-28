#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct CharacterRank {
    #[serde(flatten)]
    pub character_rank: BTreeMap<String, Rank>,
}

#[derive(Deserialize)]
pub struct Rank {
    pub id: String,
    pub name: String,
    pub rank: u8,
    pub desc: String,
    pub materials: Vec<Material>,
    pub level_up_skills: Vec<LevelUpSkill>,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct Material {
    pub id: String,
    pub num: u32,
}

#[derive(Deserialize)]
pub struct LevelUpSkill {
    pub id: String,
    pub num: u8,
}