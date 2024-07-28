#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct LightconeRank {
    #[serde(flatten)]
    pub lightcone_rank: BTreeMap<String, Rank>,
}

#[derive(Deserialize)]
pub struct Rank {
    pub id: String,
    pub skill: String,
    pub desc: String,
    pub params: Vec<Vec<f64>>,
    pub properties: Vec<Vec<Property>>,
}

#[derive(Deserialize)]
pub struct Property {
    pub r#type: String,
    pub value: f64,
}