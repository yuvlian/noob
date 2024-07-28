#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct CharacterSkillTree {
    #[serde(flatten)]
    pub character_skill_tree: BTreeMap<String, SkillTree>,
}

#[derive(Deserialize)]
pub struct SkillTree {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub anchor: String,
    pub pre_points: Vec<String>,
    pub level_up_skills: Vec<LevelUpSkill>,
    pub levels: Vec<Level>,
    pub icon: String,
}

#[derive(Deserialize)]
pub struct LevelUpSkill {
    pub id: String,
    pub num: u8,
}

#[derive(Deserialize)]
pub struct Level {
    pub promotion: u8,
    pub level: u8,
    pub properties: Vec<Property>,
    pub materials: Vec<Material>,
}

#[derive(Deserialize)]
pub struct Property {
    pub r#type: String,
    pub value: f64,
}

#[derive(Deserialize)]
pub struct Material {
    pub id: String,
    pub num: u32,
}