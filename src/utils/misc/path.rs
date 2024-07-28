#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MiscPath {
    #[serde(alias = "Warrior")]
    pub warrior: Path,
    #[serde(alias = "Rogue")]
    pub rogue: Path,
    #[serde(alias = "Mage")]
    pub mage: Path,
    #[serde(alias = "Shaman")]
    pub shaman: Path,
    #[serde(alias = "Warlock")]
    pub warlock: Path,
    #[serde(alias = "Knight")]
    pub knight: Path,
    #[serde(alias = "Priest")]
    pub priest: Path,
    #[serde(alias = "Unknown")]
    pub unknown: Path,
}

#[derive(Deserialize)]
pub struct Path {
    pub id: String,
    pub text: String,
    pub name: String,
    pub desc: String,
    pub icon: String,
}