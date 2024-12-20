#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MiscProperty {
    #[serde(alias = "MaxHP")]
    pub max_hp: Property,
    #[serde(alias = "Attack")]
    pub attack: Property,
    #[serde(alias = "Defence")]
    pub defence: Property,
    #[serde(alias = "Speed")]
    pub speed: Property,
    #[serde(alias = "CriticalChance")]
    pub critical_chance: Property,
    #[serde(alias = "CriticalDamage")]
    pub critical_damage: Property,
    #[serde(alias = "BreakDamageAddedRatio")]
    pub break_damage_added_ratio: Property,
    #[serde(alias = "BreakDamageAddedRatioBase")]
    pub break_damage_added_ratio_base: Property,
    #[serde(alias = "HealRatio")]
    pub heal_ratio: Property,
    #[serde(alias = "MaxSP")]
    pub max_sp: Property,
    #[serde(alias = "SpecialMaxSP")]
    pub special_max_sp: Property,
    #[serde(alias = "SPRatio")]
    pub sp_ratio: Property,
    #[serde(alias = "StatusProbability")]
    pub status_probability: Property,
    #[serde(alias = "StatusResistance")]
    pub status_resistance: Property,
    #[serde(alias = "CriticalChanceBase")]
    pub critical_chance_base: Property,
    #[serde(alias = "CriticalDamageBase")]
    pub critical_damage_base: Property,
    #[serde(alias = "HealRatioBase")]
    pub heal_ratio_base: Property,
    #[serde(alias = "StanceBreakAddedRatio")]
    pub stance_break_added_ratio: Property,
    #[serde(alias = "SPRatioBase")]
    pub sp_ratio_base: Property,
    #[serde(alias = "StatusProbabilityBase")]
    pub status_probability_base: Property,
    #[serde(alias = "StatusResistanceBase")]
    pub status_resistance_base: Property,
    #[serde(alias = "PhysicalAddedRatio")]
    pub physical_added_ratio: Property,
    #[serde(alias = "PhysicalResistance")]
    pub physical_resistance: Property,
    #[serde(alias = "FireAddedRatio")]
    pub fire_added_ratio: Property,
    #[serde(alias = "FireResistance")]
    pub fire_resistance: Property,
    #[serde(alias = "IceAddedRatio")]
    pub ice_added_ratio: Property,
    #[serde(alias = "IceResistance")]
    pub ice_resistance: Property,
    #[serde(alias = "ThunderAddedRatio")]
    pub thunder_added_ratio: Property,
    #[serde(alias = "ThunderResistance")]
    pub thunder_resistance: Property,
    #[serde(alias = "WindAddedRatio")]
    pub wind_added_ratio: Property,
    #[serde(alias = "WindResistance")]
    pub wind_resistance: Property,
    #[serde(alias = "QuantumAddedRatio")]
    pub quantum_added_ratio: Property,
    #[serde(alias = "QuantumResistance")]
    pub quantum_resistance: Property,
    #[serde(alias = "ImaginaryAddedRatio")]
    pub imaginary_added_ratio: Property,
    #[serde(alias = "ImaginaryResistance")]
    pub imaginary_resistance: Property,
    #[serde(alias = "BaseHP")]
    pub base_hp: Property,
    #[serde(alias = "HPDelta")]
    pub hp_delta: Property,
    #[serde(alias = "HPAddedRatio")]
    pub hp_added_ratio: Property,
    #[serde(alias = "BaseAttack")]
    pub base_attack: Property,
    #[serde(alias = "AttackDelta")]
    pub attack_delta: Property,
    #[serde(alias = "AttackAddedRatio")]
    pub attack_added_ratio: Property,
    #[serde(alias = "BaseDefence")]
    pub base_defence: Property,
    #[serde(alias = "DefenceDelta")]
    pub defence_delta: Property,
    #[serde(alias = "DefenceAddedRatio")]
    pub defence_added_ratio: Property,
    #[serde(alias = "BaseSpeed")]
    pub base_speed: Property,
    #[serde(alias = "HealTakenRatio")]
    pub heal_taken_ratio: Property,
    #[serde(alias = "PhysicalResistanceDelta")]
    pub physical_resistance_delta: Property,
    #[serde(alias = "FireResistanceDelta")]
    pub fire_resistance_delta: Property,
    #[serde(alias = "IceResistanceDelta")]
    pub ice_resistance_delta: Property,
    #[serde(alias = "ThunderResistanceDelta")]
    pub thunder_resistance_delta: Property,
    #[serde(alias = "WindResistanceDelta")]
    pub wind_resistance_delta: Property,
    #[serde(alias = "QuantumResistanceDelta")]
    pub quantum_resistance_delta: Property,
    #[serde(alias = "ImaginaryResistanceDelta")]
    pub imaginary_resistance_delta: Property,
    #[serde(alias = "SpeedDelta")]
    pub speed_delta: Property,
    #[serde(alias = "SpeedAddedRatio")]
    pub speed_added_ratio: Property,
    #[serde(alias = "AllDamageTypeAddedRatio")]
    pub all_damage_type_added_ratio: Property,
}

#[derive(Deserialize)]
pub struct Property {
    pub r#type: String,
    pub name: String,
    pub field: String,
    pub affix: bool,
    pub ratio: bool,
    pub percent: bool,
    pub order: u8,
    pub icon: String,
}