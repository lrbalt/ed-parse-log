use crate::{
    EDString,
    common_types::{Credits, OnFootItem},
};
use ed_parse_log_files_macros::Extractable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UnknownItem {}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCollectItems {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    #[serde(rename = "Type")]
    item_type: BackpackItemType,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    count: u64,
    stolen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BackpackItemType {
    Component,
    Data,
    Item,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct BackpackItem {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
    #[serde(rename = "Type")]
    item_type: Option<BackpackItemType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBackpackChange {
    added: Option<Vec<BackpackItem>>,
    removed: Option<Vec<BackpackItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUseConsumable {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: EDString,
    #[serde(rename = "Type")]
    item_type: BackpackItemType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBackpack {
    items: Vec<BackpackItem>,
    components: Vec<BackpackItem>,
    consumables: Vec<BackpackItem>,
    data: Vec<BackpackItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SuitMod {
    #[serde(rename = "suit_backpackcapacity")]
    Backpackcapacity,
    #[serde(rename = "suit_increasedammoreserves")]
    IncreasedAmmoReserves,
    #[serde(rename = "suit_reducedtoolbatteryconsumption")]
    ReducedToolBatteryConsumption,
    #[serde(rename = "suit_increasedbatterycapacity")]
    IncreasedBatteryCapacity,
    #[serde(rename = "suit_nightvision")]
    NightVision,
    #[serde(rename = "suit_increasedshieldregen")]
    IncreasedShieldRegen,
    #[serde(rename = "suit_improvedjumpassist")]
    ImprovedJumpAssist,
    #[serde(rename = "suit_increasedsprintduration")]
    IncreasedSprintDuration,
    #[serde(rename = "suit_increasedmeleedamage")]
    IncreasedMeleeDamage,
    #[serde(rename = "suit_quieterfootsteps")]
    QuieterFootsteps,
    #[serde(rename = "suit_increasedo2capacity")]
    IncreasedO2Capacity,
    #[serde(rename = "suit_improvedradar")]
    ImprovedRadar,
    #[serde(rename = "suit_adsmovementspeed")]
    AdsMovementSpeed,
    #[serde(rename = "suit_improvedarmourrating")]
    ImprovedArmourRating,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WeaponMod {
    #[serde(rename = "weapon_clipsize")]
    Clipsize,
    #[serde(rename = "weapon_range")]
    Range,
    #[serde(rename = "weapon_suppression_pressurised")]
    SuppressionPressurised,
    #[serde(rename = "weapon_backpackreloading")]
    BackpackReloading,
    #[serde(rename = "weapon_suppression_unpressurised")]
    SuppressionUnpressurised,
    #[serde(rename = "weapon_stability")]
    Stability,
    #[serde(rename = "weapon_scope")]
    Scope,
    #[serde(rename = "weapon_reloadspeed")]
    ReloadSpeed,
    #[serde(rename = "weapon_accuracy")]
    Accuracy,
    #[serde(rename = "weapon_handling")]
    Handling,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SuitModule {
    slot_name: EDString,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    module_name: EDString,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: EDString,
    class: u64,
    weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
    modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuySuit {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub price: Credits,
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellSuit {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<SuitMod>,
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub price: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCreateSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
    modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRenameSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeleteSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUpgradeSuit {
    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    #[serde(rename = "Name")]
    pub suit_name: EDString,
    #[serde(rename = "Name_Localised")]
    pub suit_name_localised: EDString,
    pub class: u8,
    pub cost: Credits,
    pub resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDropItems {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    #[serde(rename = "Type")]
    item_type: EDString,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLoadoutEquipModule {
    loadout_name: EDString,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: EDString,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: EDString,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    slot_name: EDString,
    module_name: EDString,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: EDString,
    class: u64,
    weapon_mods: Vec<WeaponMod>,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct UpgradeResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUpgradeWeapon {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub class: u64,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub cost: Credits,
    pub resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyWeapon {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub class: u64,
    pub price: Credits,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellWeapon {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub class: u64,
    pub price: Credits,
    #[serde(rename = "SuitModuleID")]
    pub suit_module_id: u64,
    pub weapon_mods: Vec<WeaponMod>,
}
