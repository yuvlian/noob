#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct RelicMainAffix {
    #[serde(flatten)]
    pub relic_main_affix: BTreeMap<String, MainAffix>,
}

#[derive(Deserialize)]
pub struct MainAffix {
    pub id: String,
    pub affixes: MainAffixType,
}

#[derive(Deserialize)]
pub struct MainAffixType {
    #[serde(alias = "1")]
    pub i1: MainAffixProperty,
    #[serde(alias = "2")]
    pub i2: Option<MainAffixProperty>,
    #[serde(alias = "3")]
    pub i3: Option<MainAffixProperty>,
    #[serde(alias = "4")]
    pub i4: Option<MainAffixProperty>,
    #[serde(alias = "5")]
    pub i5: Option<MainAffixProperty>,
    #[serde(alias = "6")]
    pub i6: Option<MainAffixProperty>,
    #[serde(alias = "7")]
    pub i7: Option<MainAffixProperty>,
    #[serde(alias = "8")]
    pub i8: Option<MainAffixProperty>,
    #[serde(alias = "9")]
    pub i9: Option<MainAffixProperty>,
    #[serde(alias = "10")]
    pub i10: Option<MainAffixProperty>,
}

#[derive(Deserialize)]
pub struct MainAffixProperty {
    pub affix_id: String,
    pub property: String,
    pub base: f64,
    pub step: f64,
}