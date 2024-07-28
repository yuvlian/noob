pub mod character;
use character::list::CharacterList;
use character::promotion::CharacterPromotion;
use character::rank::CharacterRank;
use character::skill_tree::CharacterSkillTree;
use character::skill::CharacterSkill;

pub mod lightcone;
use lightcone::list::LightconeList;
use lightcone::promotion::LightconePromotion;
use lightcone::rank::LightconeRank;

pub mod misc;
use misc::achievement::MiscAchievement;
use misc::avatar::MiscAvatar;
use misc::description::MiscDescription;
use misc::element::MiscElement;
use misc::item::MiscItem;
use misc::nickname::MiscNickname;
use misc::path::MiscPath;
use misc::property::MiscProperty;

pub mod relic;
use relic::list::RelicList;
use relic::main_affix::RelicMainAffix;
use relic::set::RelicSet;
use relic::sub_affix::RelicSubAffix;

use constcat::concat as constcat; // need this cuz of how macros work...
// they're compiled before variables so the std concat wont work with consts

// usage: get!(func_name, url: &str, struct: T)
macro_rules! get {
    ($func_name:ident, $url:expr, $response_type:ty) => {
        pub async fn $func_name() -> Result<$response_type, Box<dyn std::error::Error>> {
            let response: $response_type = serde_json::from_str(&reqwest::get($url).await?.text().await?)?;
            Ok(response)
        }
    };
}

const BASE_URL: &str = "https://raw.githubusercontent.com/Mar-7th/StarRailRes/master/index_min/en/";

const CHARACTERS_URL: &str = constcat!(BASE_URL, "characters.json");
const CHARACTER_PROMOTIONS_URL: &str = constcat!(BASE_URL, "character_promotions.json");
const CHARACTER_RANKS_URL: &str = constcat!(BASE_URL, "character_ranks.json");
const CHARACTER_SKILL_TREES_URL: &str = constcat!(BASE_URL, "character_skill_trees.json");
const CHARACTER_SKILLS_URL: &str = constcat!(BASE_URL, "character_skills.json");

const LIGHT_CONES_URL: &str = constcat!(BASE_URL, "light_cones.json");
const LIGHT_CONE_PROMOTIONS_URL: &str = constcat!(BASE_URL, "light_cone_promotions.json");
const LIGHT_CONE_RANKS_URL: &str = constcat!(BASE_URL, "light_cone_ranks.json");

const RELICS_URL: &str = constcat!(BASE_URL, "relics.json");
const RELIC_MAIN_AFFIXES_URL: &str = constcat!(BASE_URL, "relic_main_affixes.json");
const RELIC_SETS_URL: &str = constcat!(BASE_URL, "relic_sets.json");
const RELIC_SUB_AFFIXES_URL: &str = constcat!(BASE_URL, "relic_sub_affixes.json");

const ACHIEVEMENTS_URL: &str = constcat!(BASE_URL, "achievements.json");
const AVATARS_URL: &str = constcat!(BASE_URL, "avatars.json");
const DESCRIPTIONS_URL: &str = constcat!(BASE_URL, "descriptions.json");
const ELEMENTS_URL: &str = constcat!(BASE_URL, "elements.json");
const ITEMS_URL: &str = constcat!(BASE_URL, "items.json");
const NICKNAMES_URL: &str = constcat!(BASE_URL, "nickname.json");
const PATHS_URL: &str = constcat!(BASE_URL, "paths.json");
const PROPERTIES_URL: &str = constcat!(BASE_URL, "properties.json");

get!(character_lists, CHARACTERS_URL, CharacterList);
get!(character_promotions, CHARACTER_PROMOTIONS_URL, CharacterPromotion);
get!(character_ranks, CHARACTER_RANKS_URL, CharacterRank);
get!(character_skill_trees, CHARACTER_SKILL_TREES_URL, CharacterSkillTree);
get!(character_skills, CHARACTER_SKILLS_URL, CharacterSkill);

get!(lightcone_lists, LIGHT_CONES_URL, LightconeList);
get!(lightcone_promotions, LIGHT_CONE_PROMOTIONS_URL, LightconePromotion);
get!(lightcone_ranks, LIGHT_CONE_RANKS_URL, LightconeRank);

get!(misc_achievements, ACHIEVEMENTS_URL, MiscAchievement);
get!(misc_avatars, AVATARS_URL, MiscAvatar);
get!(misc_descriptions, DESCRIPTIONS_URL, MiscDescription);
get!(misc_elements, ELEMENTS_URL, MiscElement);
get!(misc_items, ITEMS_URL, MiscItem);
get!(misc_nicknames, NICKNAMES_URL, MiscNickname);
get!(misc_paths, PATHS_URL, MiscPath);
get!(misc_properties, PROPERTIES_URL, MiscProperty);

get!(relic_lists, RELICS_URL, RelicList);
get!(relic_main_affixes, RELIC_MAIN_AFFIXES_URL, RelicMainAffix);
get!(relic_sets, RELIC_SETS_URL, RelicSet);
get!(relic_sub_affixes, RELIC_SUB_AFFIXES_URL, RelicSubAffix);