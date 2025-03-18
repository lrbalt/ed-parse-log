use crate::common_types::OnFootItem;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UnknownItem {}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCollectItems {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
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
    name_localised: Option<String>,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
    #[serde(rename = "Type")]
    item_type: Option<BackpackItemType>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBackpackChange {
    added: Option<Vec<BackpackItem>>,
    removed: Option<Vec<BackpackItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUseConsumable {
    name: OnFootItem,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Type")]
    item_type: BackpackItemType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    #[serde(rename = "suit_increasedo2capacity")]
    IncreasedO2Capacity,
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SuitModule {
    slot_name: String,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: String,
    class: u64,
    weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: String,
    modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuySuit {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    price: u64,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_mods: Vec<SuitMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellSuit {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_mods: Vec<SuitMod>,
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    price: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCreateSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    suit_mods: Vec<SuitMod>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: String,
    modules: Vec<SuitModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogRenameSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeleteSuitLoadout {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    loadout_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUpgradeSuit {
    #[serde(rename = "SuitID")]
    suit_id: u64,
    #[serde(rename = "Name")]
    suit_name: String,
    #[serde(rename = "Name_Localised")]
    suit_name_localised: String,
    class: u8,
    cost: u64,
    resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDropItems {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Type")]
    item_type: String,
    #[serde(rename = "OwnerID")]
    owner_id: u64,
    #[serde(rename = "MissionID")]
    mission_id: Option<u64>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogLoadoutEquipModule {
    loadout_name: String,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    slot_name: String,
    module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: String,
    class: u64,
    weapon_mods: Vec<WeaponMod>,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct UpgradeResource {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogUpgradeWeapon {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    class: u64,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    cost: u64,
    resources: Vec<UpgradeResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyWeapon {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    class: u64,
    price: u64,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    weapon_mods: Vec<WeaponMod>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellWeapon {
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    class: u64,
    price: u64,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    weapon_mods: Vec<WeaponMod>,
}
