#![allow(unused_variables)]

mod utils;
use std::time::Instant;
use utils::{
    character_lists,
    character_skills,
    character_skill_trees,
    character_promotions,
    character_ranks,

    lightcone_lists,
    lightcone_promotions,
    lightcone_ranks,

    relic_lists,
    relic_main_affixes,
    relic_sets,
    relic_sub_affixes,

    misc_achievements,
    misc_avatars,
    misc_descriptions,
    misc_elements,
    misc_items,
    misc_nicknames,
    misc_paths,
    misc_properties,
};

macro_rules! deserialize_test {
    ($func:ident, $file:expr) => {
        match $func().await {
            Ok(result) => {
                print!("[LOG] Deserializing \"{}\" => ", $file);
                println!("Passed!");
                result
            },
            Err(e) => return Err(e),
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[LOG] Starting tests and benchmark...");
    let start_time = Instant::now();
    // Character stuff
    let characters = deserialize_test!(character_lists, "characters.json");
    let character_skills = deserialize_test!(character_skills, "character_skills.json");
    let character_skill_trees = deserialize_test!(character_skill_trees, "character_skill_trees.json");
    let character_promotions = deserialize_test!(character_promotions, "character_promotions.json");
    let character_ranks = deserialize_test!(character_ranks, "character_ranks.json");

    // lightcone
    let lightcones = deserialize_test!(lightcone_lists, "light_cones.json");
    let lightcone_promotions = deserialize_test!(lightcone_promotions, "light_cone_promotions.json");
    let lightcone_ranks = deserialize_test!(lightcone_ranks, "light_cone_ranks.json");

    // relic
    let relic_lists = deserialize_test!(relic_lists, "relics.json");
    let relic_main_affixes = deserialize_test!(relic_main_affixes, "relic_main_affixes.json");
    let relic_sets = deserialize_test!(relic_sets, "relic_sets.json");
    let relic_sub_affixes = deserialize_test!(relic_sub_affixes, "relic_sub_affixes.json");

    // misc
    let misc_achievements = deserialize_test!(misc_achievements, "achievements.json");
    let misc_avatars = deserialize_test!(misc_avatars, "avatars.json");
    let misc_descriptions = deserialize_test!(misc_descriptions, "descriptions.json");
    let misc_elements = deserialize_test!(misc_elements, "elements.json");
    let misc_items = deserialize_test!(misc_items, "items.json");
    let misc_nicknames = deserialize_test!(misc_nicknames, "nicknames.json");
    let misc_paths = deserialize_test!(misc_paths, "paths.json");
    let misc_properties = deserialize_test!(misc_properties, "properties.json");
    
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("[LOG] All tests passed.");
    print!("[LOG] Elapsed time: {:?}", elapsed_time);
    Ok(())
}