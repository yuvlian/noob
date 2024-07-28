#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MiscDescription {
    #[serde(flatten)]
    pub description: BTreeMap<String, Description>,
}

#[derive(Deserialize)]
pub struct Description {
    pub id: String,
    pub title: String,
    pub desc: String,
}