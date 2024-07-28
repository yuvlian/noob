#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct RelicSet {
    #[serde(flatten)]
    pub relic_set: BTreeMap<String, Set>,
}

#[derive(Deserialize)]
pub struct Set {
    pub id: String,
    pub name: String,
    pub desc: Vec<String>,
    pub properties: Vec<Vec<Property>>,
    pub icon: String,
    pub guide_overview: Vec<String>,
}

#[derive(Deserialize)]
pub struct Property {
    pub r#type: String,
    pub value: f64,
}