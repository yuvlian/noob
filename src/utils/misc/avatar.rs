#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MiscAvatar {
    #[serde(flatten)]
    pub avatar: BTreeMap<String, Avatar>,
}

#[derive(Deserialize)]
pub struct Avatar {
    id: String,
    name: String,
    icon: String,
}