
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde_json::Value;

use grand_exchange_api::request;
use grand_exchange_api::configuration::ConfigInfo;

static BASE_URL: &str = "http://services.runescape.com/m=itemdb_rs/api/catalogue/category.json?category=";

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalogue {
    types: Vec<Value>,
    alpha: Vec<AlphaResult>
}

#[derive(Debug, Serialize, Deserialize)]
struct AlphaResult {
    letter: String,
    items: u8
}

#[derive(Debug)]
pub enum ItemCategory {
    Miscellaneous = 0,
    Ammo = 1,
    Arrows = 2,
    Bolts = 3,
    ConstructionMaterials = 4,
    ConstructionProjects = 5,
    CookingIngredients = 6,
    Costumes = 7,
    CraftingMaterials = 8,
    Familiars = 9,
    FarmingProduce = 10,
    FletchingMaterials = 11,
    FoodAndDrink = 12,
    HerbloreMaterials = 13,
    HuntingEquipment = 14,
    HuntingProduce = 15,
    Jewellery = 16,
    MageArmour = 17,
    MageWeapons = 18,
    MeleeArmourLowLevel = 19,
    MeleeArmourMidLevel = 20,
    MeleeArmourHighLevel = 21,
    MeleeWeaponsLowLevel = 22,
    MeleeWeaponsMidLevel = 23,
    MeleeWeaponsHighLevel = 24,
    MiningAndSmithing = 25,
    Potions = 26,
    PrayerArmour = 27,
    PrayerMaterials = 28,
    RangeArmour = 29,
    RangeWeapons = 30,
    Runecrafting = 31,
    RunesSpellsAndTeleports = 32,
    Seeds = 33,
    SummoningScrolls = 34,
    ToolsAndContainers = 35,
    WoodcuttingProduct = 36,
    PocketItems = 37
}

impl ItemCategory {
    pub fn from_u8(int: u8) -> Option<ItemCategory> {
        match int {
            0 => Some(ItemCategory::Miscellaneous),
            1 => Some(ItemCategory::Ammo),
            2 => Some(ItemCategory::Arrows),
            3 => Some(ItemCategory::Bolts),
            4 => Some(ItemCategory::ConstructionMaterials),
            5 => Some(ItemCategory::ConstructionProjects),
            6 => Some(ItemCategory::CookingIngredients),
            7 => Some(ItemCategory::Costumes),
            8 => Some(ItemCategory::CraftingMaterials),
            9 => Some(ItemCategory::Familiars),
            10 => Some(ItemCategory::FarmingProduce),
            11 => Some(ItemCategory::FletchingMaterials),
            12 => Some(ItemCategory::FoodAndDrink),
            13 => Some(ItemCategory::HerbloreMaterials),
            14 => Some(ItemCategory::HuntingEquipment),
            15 => Some(ItemCategory::HuntingProduce),
            16 => Some(ItemCategory::Jewellery),
            17 => Some(ItemCategory::MageArmour),
            18 => Some(ItemCategory::MageWeapons),
            19 => Some(ItemCategory::MeleeArmourLowLevel),
            20 => Some(ItemCategory::MeleeArmourMidLevel),
            21 => Some(ItemCategory::MeleeArmourHighLevel),
            22 => Some(ItemCategory::MeleeWeaponsLowLevel),
            23 => Some(ItemCategory::MeleeWeaponsMidLevel),
            24 => Some(ItemCategory::MeleeWeaponsHighLevel),
            25 => Some(ItemCategory::MiningAndSmithing),
            26 => Some(ItemCategory::Potions),
            27 => Some(ItemCategory::PrayerArmour),
            28 => Some(ItemCategory::PrayerMaterials),
            29 => Some(ItemCategory::RangeArmour),
            30 => Some(ItemCategory::RangeWeapons),
            31 => Some(ItemCategory::Runecrafting),
            32 => Some(ItemCategory::RunesSpellsAndTeleports),
            33 => Some(ItemCategory::Seeds),
            34 => Some(ItemCategory::SummoningScrolls),
            35 => Some(ItemCategory::ToolsAndContainers),
            36 => Some(ItemCategory::WoodcuttingProduct),
            37 => Some(ItemCategory::PocketItems),
            _ => None
        }
    }

    pub fn from_category(cat: ItemCategory) -> Option<u8> {
        match cat {
            ItemCategory::Miscellaneous => Some(0),
            ItemCategory::Ammo => Some(1),
            ItemCategory::Arrows => Some(2),
            ItemCategory::Bolts => Some(3),
            ItemCategory::ConstructionMaterials => Some(4),
            ItemCategory::ConstructionProjects => Some(5),
            ItemCategory::CookingIngredients => Some(6),
            ItemCategory::Costumes => Some(7),
            ItemCategory::CraftingMaterials => Some(8),
            ItemCategory::Familiars => Some(9),
            ItemCategory::FarmingProduce => Some(10),
            ItemCategory::FletchingMaterials => Some(11),
            ItemCategory::FoodAndDrink => Some(12),
            ItemCategory::HerbloreMaterials => Some(13),
            ItemCategory::HuntingEquipment => Some(14),
            ItemCategory::HuntingProduce => Some(15),
            ItemCategory::Jewellery => Some(16),
            ItemCategory::MageArmour => Some(17),
            ItemCategory::MageWeapons => Some(18),
            ItemCategory::MeleeArmourLowLevel => Some(19),
            ItemCategory::MeleeArmourMidLevel => Some(20),
            ItemCategory::MeleeArmourHighLevel => Some(21),
            ItemCategory::MeleeWeaponsLowLevel => Some(22),
            ItemCategory::MeleeWeaponsMidLevel => Some(23),
            ItemCategory::MeleeWeaponsHighLevel => Some(24),
            ItemCategory::MiningAndSmithing => Some(25),
            ItemCategory::Potions => Some(26),
            ItemCategory::PrayerArmour => Some(27),
            ItemCategory::PrayerMaterials => Some(28),
            ItemCategory::RangeArmour => Some(29),
            ItemCategory::RangeWeapons => Some(30),
            ItemCategory::Runecrafting => Some(31),
            ItemCategory::RunesSpellsAndTeleports => Some(32),
            ItemCategory::Seeds => Some(33),
            ItemCategory::SummoningScrolls => Some(34),
            ItemCategory::ToolsAndContainers => Some(35),
            ItemCategory::WoodcuttingProduct => Some(36),
            ItemCategory::PocketItems => Some(37),
            _ => None
        }
    }
}

pub fn query(category: ItemCategory) -> Option<Catalogue> {
    let s = ItemCategory::from_category(category).unwrap().to_string();
    let url = BASE_URL.to_owned() + s.as_str();
    return self::request::<Catalogue>(&url);
}
