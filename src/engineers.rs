use crate::{
    EDString,
    common_types::ModuleEngineeringModifiers,
    loadout::{EngineeringBlueprint, EngineeringExperimentalEffect},
    market::MarketItemType,
    ship_module::{ShipModule, ShipModuleSlot, serde_ship_module},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum Engineer {
    #[strum(to_string = "Baltanos")]
    #[serde(rename = "Baltanos")]
    Baltanos,
    #[strum(to_string = "Bill Turner")]
    #[serde(rename = "Bill Turner")]
    BillTurner,
    #[strum(to_string = "Broo Tarquin")]
    #[serde(rename = "Broo Tarquin")]
    BrooTarquin,
    #[strum(to_string = "Chloe Sedesi")]
    #[serde(rename = "Chloe Sedesi")]
    ChloeSedesi,
    #[strum(to_string = "Colonel Bris Dekker")]
    #[serde(rename = "Colonel Bris Dekker")]
    ColonelBrisDekker,
    #[strum(to_string = "Didi Vatermann")]
    #[serde(rename = "Didi Vatermann")]
    DidiVatermann,
    #[strum(to_string = "Domino Green")]
    #[serde(rename = "Domino Green")]
    DominoGreen,
    #[strum(to_string = "The Dweller")]
    #[serde(rename = "The Dweller")]
    TheDweller,
    #[strum(to_string = "Elvira Martuuk")]
    #[serde(rename = "Elvira Martuuk")]
    ElviraMartuuk,
    #[strum(to_string = "Eleanor Bresa")]
    #[serde(rename = "Eleanor Bresa")]
    EleanorBresa,
    #[strum(to_string = "Etienne Dorn")]
    #[serde(rename = "Etienne Dorn")]
    EtienneDorn,
    #[strum(to_string = "Felicity Farseer")]
    #[serde(rename = "Felicity Farseer")]
    FelicityFarseer,
    #[strum(to_string = "Hera Tani")]
    #[serde(rename = "Hera Tani")]
    HeraTani,
    #[strum(to_string = "Hero Ferrari")]
    #[serde(rename = "Hero Ferrari")]
    HeroFerrari,
    #[strum(to_string = "Jude Navarro")]
    #[serde(rename = "Jude Navarro")]
    JudeNavarro,
    #[strum(to_string = "Juri Ishmaak")]
    #[serde(rename = "Juri Ishmaak")]
    JuriIshmaak,
    #[strum(to_string = "Kit Fowler")]
    #[serde(rename = "Kit Fowler")]
    KitFowler,
    #[strum(to_string = "Lei Cheung")]
    #[serde(rename = "Lei Cheung")]
    LeiCheung,
    #[strum(to_string = "Liz Ryder")]
    #[serde(rename = "Liz Ryder")]
    LizRyder,
    #[strum(to_string = "Lori Jameson")]
    #[serde(rename = "Lori Jameson")]
    LoriJameson,
    #[strum(to_string = "Marsha Hicks")]
    #[serde(rename = "Marsha Hicks")]
    MarshaHicks,
    #[strum(to_string = "Mel Brandon")]
    #[serde(rename = "Mel Brandon")]
    MelBrandon,
    #[strum(to_string = "Marco Qwent")]
    #[serde(rename = "Marco Qwent")]
    MarcoQwent,
    #[strum(to_string = "Oden Geiger")]
    #[serde(rename = "Oden Geiger")]
    OdenGeiger,
    #[strum(to_string = "Petra Olmanova")]
    #[serde(rename = "Petra Olmanova")]
    PetraOlmanova,
    #[strum(to_string = "Professor Palin")]
    #[serde(rename = "Professor Palin")]
    ProfessorPalin,
    #[strum(to_string = "Ram Tah")]
    #[serde(rename = "Ram Tah")]
    RamTah,
    #[strum(to_string = "The Sarge")]
    #[serde(rename = "The Sarge")]
    TheSarge,
    #[strum(to_string = "Rosa Dayette")]
    #[serde(rename = "Rosa Dayette")]
    RosaDayette,
    #[strum(to_string = "Selene Jean")]
    #[serde(rename = "Selene Jean")]
    SeleneJean,
    #[strum(to_string = "Terra Velasquez")]
    #[serde(rename = "Terra Velasquez")]
    TerraVelasquez,
    #[strum(to_string = "Tiana Fortune")]
    #[serde(rename = "Tiana Fortune")]
    TianaFortune,
    #[strum(to_string = "Tod 'The Blaster' McQuinn")]
    #[serde(rename = "Tod 'The Blaster' McQuinn")]
    TodTheBlasterMcQuinn,
    #[strum(to_string = "Uma Laszlo")]
    #[serde(rename = "Uma Laszlo")]
    UmaLaszlo,
    #[strum(to_string = "Wellington Beck")]
    #[serde(rename = "Wellington Beck")]
    WellingtonBeck,
    #[strum(to_string = "Yarden Bond")]
    #[serde(rename = "Yarden Bond")]
    YardenBond,
    #[strum(to_string = "Yi Shen")]
    #[serde(rename = "Yi Shen")]
    YiShen,
    #[strum(to_string = "Zacariah Nemo")]
    #[serde(rename = "Zacariah Nemo")]
    ZacariahNemo,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EngineerProgressState {
    Invited,
    Unlocked,
    Known,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" })]
pub struct EngineerProgress {
    engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    engineer_id: Option<u64>,
    progress: EngineerProgressState,
    rank_progress: Option<u64>,
    rank: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-06-13T18:58:38Z", "event":"EngineerProgress", "Engineer":"Felicity Farseer", "EngineerID":300100, "Progress":"Invited" })]
#[testcase({ "timestamp":"2022-06-15T18:12:12Z", "event":"EngineerProgress", "Engineers":[ { "Engineer":"Tod 'The Blaster' McQuinn", "EngineerID":300260, "Progress":"Known" } ] })]
pub struct EDLogEngineerProgress {
    #[serde(flatten)]
    pub engineer: Option<EngineerProgress>,
    pub engineers: Option<Vec<EngineerProgress>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EngineerCraftIngredient {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ExperimentalEffect {
    apply_experimental_effect: EngineeringExperimentalEffect,
    experimental_effect: EngineeringExperimentalEffect,
    #[serde(rename = "ExperimentalEffect_Localised")]
    experimental_effect_localised: EDString,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-07-12T17:50:29Z", "event":"EngineerCraft", "Slot":"TinyHardpoint3", "Module":"hpt_shieldbooster_size0_class5", "Ingredients":[ 
    { "Name":"conductiveceramics", "Name_Localised":"Conductive Ceramics", "Count":1 }, 
    { "Name":"refinedfocuscrystals", "Name_Localised":"Refined Focus Crystals", "Count":1 }, 
    { "Name":"imperialshielding", "Name_Localised":"Imperial Shielding", "Count":1 } ], 
    "Engineer":"Didi Vatermann", "EngineerID":300000, "BlueprintID":128673794, "BlueprintName":"ShieldBooster_Resistive", "Level":5, "Quality":0.400000, "Modifiers":[ 
        { "Label":"Integrity", "Value":42.239998, "OriginalValue":48.000000, "LessIsGood":0 }, 
        { "Label":"PowerDraw", "Value":1.500000, "OriginalValue":1.200000, "LessIsGood":1 }, 
        {"Label":"KineticResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ThermicResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 }, 
        { "Label":"ExplosiveResistance", "Value":15.200001, "OriginalValue":0.000000, "LessIsGood":0 } ] })]
pub struct EDLogEngineerCraft {
    pub slot: ShipModuleSlot,
    #[serde(with = "serde_ship_module")]
    pub module: ShipModule,
    pub ingredients: Vec<EngineerCraftIngredient>,
    pub engineer: Option<Engineer>,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "BlueprintID")]
    pub blueprint_id: u64,
    pub blueprint_name: EngineeringBlueprint,
    pub level: u64,
    pub quality: f64,
    #[serde(flatten)]
    pub experimental_effect: Option<ExperimentalEffect>,
    pub modifiers: Vec<ModuleEngineeringModifiers>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum EngineerContributionType {
    Bond,
    Bounty,
    Commodity,
    Credits,
    Materials,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2024-01-20T19:22:48Z", "event":"EngineerContribution", "Engineer":"Colonel Bris Dekker", 
    "EngineerID":300140, "Type":"Bond", "Quantity":126004, "TotalQuantity":1000000 })]
#[testcase({ "timestamp":"2024-01-03T19:35:39Z", "event":"EngineerContribution", "Engineer":"The Sarge", "EngineerID":300040, 
    "Type":"Materials", "Material":"shieldpatternanalysis", "Material_Localised":"Aberrant Shield Pattern Analysis", 
    "Quantity":50, "TotalQuantity":50 })]
#[testcase({ "timestamp":"2024-01-03T14:41:59Z", "event":"EngineerContribution", "Engineer":"Zacariah Nemo", "EngineerID":300050, 
    "Type":"Commodity", "Commodity":"xihecompanions", "Commodity_Localised":"Xihe Biomorphic Companions", "Quantity":5, 
    "TotalQuantity":25 })]
#[testcase({ "timestamp":"2023-07-23T21:19:45Z", "event":"EngineerContribution", "Engineer":"Mel Brandon", "EngineerID":300280, 
    "Type":"Bounty", "Quantity":100000, "TotalQuantity":100000 })]
pub struct EDLogEngineerContribution {
    pub engineer: Engineer,
    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,
    #[serde(rename = "Type")]
    pub contribution_type: EngineerContributionType,
    pub commodity: Option<MarketItemType>,
    #[serde(rename = "Commodity_Localised")]
    pub commodity_localised: Option<EDString>,
    pub material: Option<EDString>,
    #[serde(rename = "Material_Localised")]
    pub material_localised: Option<EDString>,
    pub quantity: u64,
    pub total_quantity: u64,
}
