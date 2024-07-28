#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MiscElement {
    #[serde(alias = "Physical")]
    pub physical: Element,
    #[serde(alias = "Fire")]
    pub fire: Element,
    #[serde(alias = "Ice")]
    pub ice: Element,
    #[serde(alias = "Thunder")]
    pub thunder: Element,
    #[serde(alias = "Wind")]
    pub wind: Element,
    #[serde(alias = "Quantum")]
    pub quantum: Element,
    #[serde(alias = "Imaginary")]
    pub imaginary: Element,
    #[serde(alias = "Lightning")]
    pub lightning: Element,
}

#[derive(Deserialize)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub desc: String,
    pub color: String,
    pub icon: String,
}