#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct RelicList {
    #[serde(flatten)]
    pub relic_list: BTreeMap<String, List>,
}

#[derive(Deserialize)]
pub struct List {
    pub id: String,
    pub set_id: String,
    pub name: String,
    pub rarity: u8,
    pub r#type: String,
    pub max_level: u8,
    pub main_affix_id: String,
    pub sub_affix_id: String,
    pub icon: String,
}