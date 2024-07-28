#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct CharacterList {
    #[serde(flatten)]
    pub character_list: BTreeMap<String, List>,
}

#[derive(Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub rarity: u8,
    pub path: String,
    pub element: String,
    pub max_sp: u16,
    pub ranks: Vec<String>,
    pub skills: Vec<String>,
    pub skill_trees: Vec<String>,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
}