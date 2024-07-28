#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MiscAchievement {
    #[serde(flatten)]
    pub achievement: BTreeMap<String, Achievement>,
}

#[derive(Deserialize)]
pub struct Achievement {
    id: String,
    series_id: String,
    title: String,
    desc: String,
    hide_desc: String,
    hide: bool,
}