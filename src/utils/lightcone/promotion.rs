#![allow(dead_code)]

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
pub struct LightconePromotion {
    #[serde(flatten)]
    pub lightcone_promotion: BTreeMap<String, Promotion>,
}

#[derive(Deserialize)]
pub struct Promotion {
    pub id: String,
    pub values: Vec<Value>,
    pub materials: Vec<Vec<Material>>,
}

#[derive(Deserialize)]
pub struct Value {
    pub hp: ValueType,
    pub atk: ValueType,
    pub def: ValueType,
}

#[derive(Deserialize)]
pub struct ValueType {
    pub base: f64,
    pub step: f64,
}

#[derive(Deserialize)]
pub struct Material {
    pub id: String,
    pub num: u32,
}