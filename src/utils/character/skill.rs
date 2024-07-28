#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct CharacterSkill {
    #[serde(flatten)]
    pub character_skill: BTreeMap<String, Skill>,
}

#[derive(Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub max_level: u8,
    pub element: String,
    pub r#type: String,
    pub type_text: String,
    pub effect: String,
    pub effect_text: String,
    pub simple_desc: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub icon: String,
}