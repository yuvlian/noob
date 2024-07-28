# StarRailRust
fetch & deserialize everything in https://github.com/Mar-7th/StarRailRes/tree/master/index_min/en

except for the SU stuff, still todo!()

this is actually for a website builder that i'll make soon (hence the main.rs), but you can use this repo as a library too

add this to cargo.toml dependencies if you want to do so

```
starrailrust = { git = "https://github.com/yuvlian/starrailrust" }
```

example usage
```
use starrailres::character_lists;
use starrailres::character_skills;
use starrailres::character_skill_trees;
use starrailres::character_ranks;
use starrailres::character;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let character_list_data = character_lists().await?;
    let character_skill_data = character_skills().await?;
    let character_skill_trees_data = character_skill_trees().await?;
    let character_rank_data = character_ranks().await?;

    let skill_map: BTreeMap<String, &character::skill::Skill> = character_skill_data
        .character_skill
        .iter()
        .map(|(id, skill)| (id.clone(), skill))
        .collect();

    let skill_tree_map: BTreeMap<String, &character::skill_tree::SkillTree> = character_skill_trees_data
        .character_skill_tree
        .iter()
        .map(|(id, skill_tree)| (id.clone(), skill_tree))
        .collect();

    let rank_map: BTreeMap<String, String> = character_rank_data
        .character_rank
        .iter()
        .map(|(id, rank)| (id.clone(), rank.name.clone()))
        .collect();

    for (key, character) in &character_list_data.character_list {
        println!("Character Key: {}", key);
        println!("Character ID: {}", character.id);
        println!("Character Name: {}", character.name);
        println!("Character Tag: {}", character.tag);
        println!("Character Rarity: {}", character.rarity);
        println!("Character Path: {}", character.path);
        println!("Character Element: {}", character.element);
        println!("Character Max SP: {}", character.max_sp);

        println!("Character Ranks:");
        for rank_id in &character.ranks {
            if let Some(rank_name) = rank_map.get(rank_id) {
                println!("  - {}", rank_name);
            } else {
                println!("  - Unknown rank with ID: {}", rank_id);
            }
        }

        println!("Character Skills:");
        for skill_id in &character.skills {
            if let Some(skill) = skill_map.get(skill_id) {
                println!("  - Name: {}", skill.name);
                println!("    Effect: {}", skill.effect);
            } else {
                println!("  - Unknown skill with ID: {}", skill_id);
            }
        }

        println!("Character Skill Trees:");
        for skill_tree_id in &character.skill_trees {
            if let Some(skill_tree) = skill_tree_map.get(skill_tree_id) {
                println!("  - Name: {}", skill_tree.name);
                println!("    Anchor: {}", skill_tree.anchor);
            } else {
                println!("  - Unknown skill tree with ID: {}", skill_tree_id);
            }
        }

        println!("Character Icon: {}", character.icon);
        println!("Character Preview: {}", character.preview);
        println!("Character Portrait: {}\n", character.portrait);
    }
    Ok(())
}
```
