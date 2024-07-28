#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct RelicSubAffix {
    #[serde(flatten)]
    pub relic_sub_affix: BTreeMap<String, SubAffix>,
}

#[derive(Deserialize)]
pub struct SubAffix {
    pub id: String,
    pub affixes: SubAffixType,
}

#[derive(Deserialize)]
pub struct SubAffixType {
    #[serde(alias = "1")]
    pub i1: SubAffixProperty,
    #[serde(alias = "2")]
    pub i2: SubAffixProperty,
    #[serde(alias = "3")]
    pub i3: SubAffixProperty,
    #[serde(alias = "4")]
    pub i4: SubAffixProperty,
    #[serde(alias = "5")]
    pub i5: SubAffixProperty,
    #[serde(alias = "6")]
    pub i6: SubAffixProperty,
    #[serde(alias = "7")]
    pub i7: SubAffixProperty,
    #[serde(alias = "8")]
    pub i8: SubAffixProperty,
    #[serde(alias = "9")]
    pub i9: SubAffixProperty,
    #[serde(alias = "10")]
    pub i10: SubAffixProperty,
    #[serde(alias = "11")]
    pub i11: SubAffixProperty,
    #[serde(alias = "12")]
    pub i12: SubAffixProperty,
}

#[derive(Deserialize)]
pub struct SubAffixProperty {
    pub affix_id: String,
    pub property: String,
    pub base: f64,
    pub step: f64,
    pub step_num: u8,
}