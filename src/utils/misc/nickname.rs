#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct MiscNickname {
    #[serde(alias = "characters")]
    pub nickname_character: Nickname,
    #[serde(alias = "light_cones")]
    pub nickname_lightcone: Nickname,
    #[serde(alias = "relic_sets")]
    pub nickname_relic_set: Nickname,
}

#[derive(Deserialize)]
pub struct Nickname {
    #[serde(flatten)]
    pub id: BTreeMap<String, Vec<String>>
}