#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MiscItem {
    #[serde(flatten)]
    pub item: BTreeMap<String, Item>,
}

#[derive(Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub sub_type: String,
    pub rarity: u8,
    pub icon: String,
    #[serde(rename = "come_from")]
    pub source: Vec<String>,
}