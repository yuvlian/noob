#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct LightconeList {
    #[serde(flatten)]
    pub lightcone_list: BTreeMap<String, List>,
}

#[derive(Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    pub rarity: u8,
    pub path: String,
    pub desc: String,
    pub icon: String,
    pub preview: String,
    pub portrait: String,
}