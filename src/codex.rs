use ed_parse_log_files_macros::CodexCategorize;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Copy,
    Display,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    EnumIter,
    CodexCategorize,
)]
pub enum CodexNames {
    //
    // Stars
    //
    #[serde(rename = "$Codex_Ent_SupermassiveBlack_Holes_Name;")]
    #[strum(to_string = "Supermassive Black Hole")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    SupermassiveBlackHole,
    #[serde(rename = "$Codex_Ent_Black_Holes_Name;")]
    #[strum(to_string = "Black Hole")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    BlackHole,
    #[serde(rename = "$Codex_Ent_Neutron_Stars_Name;")]
    #[strum(to_string = "Neutron Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    NeutronStar,
    #[serde(rename = "$Codex_Ent_AeBe_Type_Name;")]
    #[strum(to_string = "AeBe Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    AeBeTypeStar,
    #[serde(rename = "$Codex_Ent_A_Type_Name;")]
    #[strum(to_string = "A Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    ATypeStar,
    #[serde(rename = "$Codex_Ent_A_TypeGiant_Name;")]
    #[strum(to_string = "A Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    ATypeGiant,
    #[serde(rename = "$Codex_Ent_B_Type_Name;")]
    #[strum(to_string = "B Types")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    BTypes,
    #[serde(rename = "$Codex_Ent_B_TypeGiant_Name;")]
    #[strum(to_string = "B Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    BTypeGiant,
    #[serde(rename = "$Codex_Ent_B_TypeSuperGiant_Name;")]
    #[strum(to_string = "B Type Supergiants")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    BTypeSuperGiant,
    #[serde(rename = "$Codex_Ent_C_TypeGiant_Name;")]
    #[strum(to_string = "C Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CTypeGiant,
    #[serde(rename = "$Codex_Ent_CN_TypeGiant_Name;")]
    #[strum(to_string = "CN Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CNTypeGiant,
    #[serde(rename = "$Codex_Ent_CJ_TypeGiant_Name;")]
    #[strum(to_string = "CJ Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CJTypeGiant,
    #[serde(rename = "$Codex_Ent_CS_TypeGiant_Name;")]
    #[strum(to_string = "CS Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CSTypeGiant,
    #[serde(rename = "$Codex_Ent_CH_TypeGiant_Name;")]
    #[strum(to_string = "CH Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CHTypeGiant,
    #[serde(rename = "$Codex_Ent_CHD_TypeGiant_Name;")]
    #[strum(to_string = "CHD Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    CHDTypeGiant,
    #[serde(rename = "$Codex_Ent_D_Type_Name;")]
    #[strum(to_string = "D Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DTypeStar,
    #[serde(rename = "$Codex_Ent_DA_Type_Name;")]
    #[strum(to_string = "DA Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DATypeStar,
    #[serde(rename = "$Codex_Ent_DAB_Type_Name;")]
    #[strum(to_string = "DAB Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DABTypeStar,
    #[serde(rename = "$Codex_Ent_DAO_Type_Name;")]
    #[strum(to_string = "DAO Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DAOTypeStar,
    #[serde(rename = "$Codex_Ent_DAV_Type_Name;")]
    #[strum(to_string = "DAV Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DAVTypeStar,
    #[serde(rename = "$Codex_Ent_DAZ_Type_Name;")]
    #[strum(to_string = "DAZ Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DAZTypeStar,
    #[serde(rename = "$Codex_Ent_DB_Type_Name;")]
    #[strum(to_string = "DB Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DBTypeStar,
    #[serde(rename = "$Codex_Ent_DBV_Type_Name;")]
    #[strum(to_string = "DBV Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DBVTypeStar,
    #[serde(rename = "$Codex_Ent_DBZ_Type_Name;")]
    #[strum(to_string = "DBZ Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DBZTypeStar,
    #[serde(rename = "$Codex_Ent_DO_Type_Name;")]
    #[strum(to_string = "DO Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DOTypeStar,
    #[serde(rename = "$Codex_Ent_DOVType_Name;")]
    #[strum(to_string = "DOV Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DOVTypeStar,
    #[serde(rename = "$Codex_Ent_DC_Type_Name;")]
    #[strum(to_string = "DC Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DCTypeStar,
    #[serde(rename = "$Codex_Ent_DCV_Type_Name;")]
    #[strum(to_string = "DCV Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DCVTypeStar,
    #[serde(rename = "$Codex_Ent_DQ_Type_Name;")]
    #[strum(to_string = "DQ Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DQTypeStar,
    #[serde(rename = "$Codex_Ent_DX_Type_Name;")]
    #[strum(to_string = "DX Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    DXTypeStar,
    #[serde(rename = "$Codex_Ent_F_TypeGiant_Name;")]
    #[strum(to_string = "F Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    FTypeGiant,
    #[serde(rename = "$Codex_Ent_F_Type_Name;")]
    #[strum(to_string = "F Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    FTypeStar,
    #[serde(rename = "$Codex_Ent_G_Type_Name;")]
    #[strum(to_string = "G Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    GTypeStar,
    #[serde(rename = "$Codex_Ent_G_TypeGiant_Name;")]
    #[strum(to_string = "G Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    GTypeGiant,
    #[serde(rename = "$Codex_Ent_K_Type_Name;")]
    #[strum(to_string = "K Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    KTypeStar,
    #[serde(rename = "$Codex_Ent_K_TypeGiant_Name;")]
    #[strum(to_string = "K Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    KTypeGiant,
    #[serde(rename = "$Codex_Ent_L_Type_Name;")]
    #[strum(to_string = "L Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    LTypeStar,
    #[serde(rename = "$Codex_Ent_M_TypeGiant_Name;")]
    #[strum(to_string = "M Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    MTypeGiant,
    #[serde(rename = "$Codex_Ent_M_Type_Name;")]
    #[strum(to_string = "M Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    MTypeStar,
    #[serde(rename = "$Codex_Ent_MS_TypeGiant_Name;")]
    #[strum(to_string = "MS Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    MSTypeGiant,
    #[serde(rename = "$Codex_Ent_O_Type_Name;")]
    #[strum(to_string = "O Type Stars")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    OTypeStars,
    #[serde(rename = "$Codex_Ent_S_TypeGiant_Name;")]
    #[strum(to_string = "S Type Giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    STypeGiant,
    #[serde(rename = "$Codex_Ent_TTS_Type_Name;")]
    #[strum(to_string = "T Tauri Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    TTauriStar,
    #[serde(rename = "$Codex_Ent_T_Type_Name;")]
    #[strum(to_string = "T Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    TTypeStar,
    #[serde(rename = "$Codex_Ent_W_Type_Name;")]
    #[strum(to_string = "W Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    WTypeStar,
    #[serde(rename = "$Codex_Ent_WC_Type_Name;")]
    #[strum(to_string = "WC Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    WCTypeStar,
    #[serde(rename = "$Codex_Ent_WN_Type_Name;")]
    #[strum(to_string = "WN Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    WNTypeStar,
    #[serde(rename = "$Codex_Ent_WNC_Type_Name;")]
    #[strum(to_string = "WNC Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    WNCTypeStar,
    #[serde(rename = "$Codex_Ent_WO_Type_Name;")]
    #[strum(to_string = "WO Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    WOTypeStar,
    #[serde(rename = "$Codex_Ent_Y_Type_Name;")]
    #[strum(to_string = "Y Type Star")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::Stars)]
    YTypeStar,
    //
    // Terraformables
    //
    #[serde(rename = "$Codex_Ent_TRF_Ter_Rocky_Name;")]
    #[strum(to_string = "Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    Terraformable,
    #[serde(rename = "$Codex_Ent_TRF_Rocky_No_Atmos_Name;")]
    #[strum(to_string = "Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    TerraformableRockyNoAtmos,
    #[serde(rename = "$Codex_Ent_TRF_High_Metal_Content_No_Atmos_Name;")]
    #[strum(to_string = "Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    TerraformableHighMetalContentNoAtmos,
    #[serde(rename = "$Codex_Ent_TRF_Ter_High_Metal_Content_Name;")]
    #[strum(to_string = "Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    TerraformableHighMetalContent,
    #[serde(rename = "$Codex_Ent_TRF_Water_Worlds_Name;")]
    #[strum(to_string = "Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    TerraformableWaterWorlds,
    //
    // Non Terraformables
    //
    #[serde(rename = "$Codex_Ent_Earth_Likes_Name;")]
    #[strum(to_string = "Earth-Like Planet")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    EarthLikePlanet,
    #[serde(rename = "$Codex_Ent_Standard_Ammonia_Worlds_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardAmmoniaWorlds,
    #[serde(rename = "$Codex_Ent_Standard_Giant_With_Ammonia_Life_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardGiantWithAmmoniaLife,
    #[serde(rename = "$Codex_Ent_Standard_Giant_With_Water_Life_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardGiantWithWaterLife,
    #[serde(rename = "$Codex_Ent_Standard_Helium_Rich_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardHeliumRich,
    #[serde(rename = "$Codex_Ent_Standard_Ice_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardIceNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardRockyIce,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerIce,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_Ice_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardRockyIceNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Rocky_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerRockyIce,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardRockyNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Rocky_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerRocky,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_I_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardSudarskyClassI,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_II_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardSudarskyClassII,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_III_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardSudarskyClassIII,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_IV_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardSudarskyClassIV,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_V_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardSudarskyClassV,
    #[serde(rename = "$Codex_Ent_Standard_Ter_High_Metal_Content_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerHighMetalContent,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Metal_Rich_Content_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerMetalRichContent,
    #[serde(rename = "$Codex_Ent_Standard_Metal_Rich_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardMetalRichNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_High_Metal_Content_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardHighMetalContentNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Metal_Rich_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardTerMetalRich,
    #[serde(rename = "$Codex_Ent_Standard_Water_Worlds_Name;")]
    #[strum(to_string = "Non Terraformable")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::TerrestrialPlanets)]
    StandardWaterWorlds,
    #[serde(rename = "$Codex_Ent_Standard_Water_Giant_Name;")]
    #[strum(to_string = "Standard gas giant")]
    #[CodexCategory(category =CodexCategory::StellarBodies, sub_category = CodexSubCategory::GasGiants)]
    StandardWaterGiant,
    //
    // Biological and Geological
    //
    #[serde(rename = "$Codex_Ent_Gas_Clds_Red_Name;")]
    #[strum(to_string = "Rubicundum Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    RubicundumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Green_Name;")]
    #[strum(to_string = "Viride Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    VirideLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Pink_Name;")]
    #[strum(to_string = "Roseum Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    RoseumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Blue_Name;")]
    #[strum(to_string = "Caeruleum Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    CaeruleumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Light_Name;")]
    #[strum(to_string = "Proto-Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    ProtoLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Yellow_Name;")]
    #[strum(to_string = "Croceum Lagrange Cloud")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    CroceumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Yw_Name;")]
    #[strum(to_string = "Flavum Metallic Crystals")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FlavumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Org_Moll03_V3_Earth_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Viride Umbrella Mollusc")]
    VirideUmbrellaMollusc,
    #[serde(rename = "$Codex_Ent_L_Org_Moll03_V6_Def_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Luteolum Reel Mollusc")]
    LuteolumReelMollusc,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Gr_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Prasinum Metallic Crystals")]
    PrasinumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Pur_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Purpureum Metallic Crystals")]
    PurpureumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Red_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Rubeum Metallic Crystals")]
    RubeumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_SPOI_Ball_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Solid Mineral Spheres")]
    SolidMineralSpheres,
    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Crystalline Shards")]
    CrystallineShards,
    #[serde(rename = "$Codex_Ent_Gas_Vents_SilicateVapourGeysers_Name;")]
    #[strum(to_string = "Silicate Vapour Gas Vent")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    SilicateVapourGasVent,
    #[serde(rename = "$Codex_Ent_Lava_Spouts_SilicateMagma_Name;")]
    #[strum(to_string = "Silicate Magma Lava Spout")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    SilicateMagmaLavaSpout,
    #[serde(rename = "$Codex_Ent_IceGeysers_CarbonDioxideGeysers_Name;")]
    #[strum(to_string = "Carbon Dioxide Ice Geyser")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    CarbonDioxideIceGeyser,
    #[serde(rename = "$Codex_Ent_IceGeysers_WaterGeysers_Name;")]
    #[strum(to_string = "Water Ice Geyser")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    WaterIceGeyser,
    #[serde(rename = "$Codex_Ent_L_Seed_SdRt02_V3_Name;")]
    #[strum(to_string = "Stolon Tree")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StolonTree,
    #[serde(rename = "$Codex_Ent_IceFumarole_WaterGeysers_Name;")]
    #[strum(to_string = "Water Ice Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    WaterIceFumarole,
    #[serde(rename = "$Codex_Ent_IceGeysers_NitrogenGeysers_Name;")]
    #[strum(to_string = "Nitrogen Ice Geyser")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    NitrogenIceGeyser,
    #[serde(rename = "$Codex_Ent_Gas_Vents_SulphurDioxideMagma_Name;")]
    #[strum(to_string = "Sulphur Dioxide Gas Vent")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    SulphurDioxideGasVent,
    #[serde(rename = "$Codex_Ent_IceFumarole_CarbonDioxideGeysers_Name;")]
    #[strum(to_string = "Carbon Dioxide Ice Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    CarbonDioxideIceFumarole,
    #[serde(rename = "$Codex_Ent_IceFumarole_MethaneGeysers_Name;")]
    #[strum(to_string = "Methane Ice Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    MethaneIceFumarole,
    #[serde(rename = "$Codex_Ent_IceFumarole_NitrogenGeysers_Name;")]
    #[strum(to_string = "Nitrogen Ice Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    NitrogenIceFumarole,
    #[serde(rename = "$Codex_Ent_IceGeysers_MethaneGeysers_Name;")]
    #[strum(to_string = "Methane Ice Geyser")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    MethaneIceGeyser,
    #[serde(rename = "$Codex_Ent_IceGeysers_AmmoniaGeysers_Name;")]
    #[strum(to_string = "Ammonia Ice Geyser")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    AmmoniaIceGeyser,
    #[serde(rename = "$Codex_Ent_Lava_Spouts_IronMagma_Name;")]
    #[strum(to_string = "Iron Magma Lava Spout")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    IronMagmaLavaSpout,
    #[serde(rename = "$Codex_Ent_IceFumarole_AmmoniaGeysers_Name;")]
    #[strum(to_string = "Ammonia Ice Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    AmmoniaIceFumarole,
    #[serde(rename = "$Codex_Ent_Gas_Vents_WaterGeysers_Name;")]
    #[strum(to_string = "Water Gas Vent")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    WaterGasVent,

    // Aleoids
    #[serde(rename = "$Codex_Ent_Aleoids_01_F_Name;")]
    #[strum(to_string = "Aleoida Arcus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaArcusTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_01_K_Name;")]
    #[strum(to_string = "Aleoida Arcus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaArcusTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_02_A_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaCoronamusGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_02_L_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaCoronamusLime,
    #[serde(rename = "$Codex_Ent_Aleoids_03_A_Name;")]
    #[strum(to_string = "Aleoida Spica - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaSpicaGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_03_F_Name;")]
    #[strum(to_string = "Aleoida Spica - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaSpicaTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_03_K_Name;")]
    #[strum(to_string = "Aleoida Spica - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaSpicaTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_03_L_Name;")]
    #[strum(to_string = "Aleoida Spica - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaSpicaLime,
    #[serde(rename = "$Codex_Ent_Aleoids_03_M_Name;")]
    #[strum(to_string = "Aleoida Spica - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaSpicaEmerald,
    #[serde(rename = "$Codex_Ent_Aleoids_04_F_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_04_K_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_04_L_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeLime,
    #[serde(rename = "$Codex_Ent_Aleoids_04_M_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeEmerald,
    #[serde(rename = "$Codex_Ent_Aleoids_04_N_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Ocher")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeOcher,
    #[serde(rename = "$Codex_Ent_Aleoids_05_A_Name;")]
    #[strum(to_string = "Aleoida Gravis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaGravisGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_05_M_Name;")]
    #[strum(to_string = "Aleoida Gravis - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaGravisEmerald,
    #[serde(rename = "$Codex_Ent_Aleoids_01_A_Name;")]
    #[strum(to_string = "Aleoida Arcus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaArcusGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_02_F_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaCoronamusTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_02_K_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaCoronamusTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_04_A_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaLaminiaeGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_05_K_Name;")]
    #[strum(to_string = "Aleoida Gravis - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaGravisTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_05_F_Name;")]
    #[strum(to_string = "Aleoida Gravis - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AleoidaGravisTeal,
    // Bacterium
    #[serde(rename = "$Codex_Ent_Bacterial_01_N_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusIndigo,
    #[serde(rename = "$Codex_Ent_Bacterial_01_L_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusSage,
    #[serde(rename = "$Codex_Ent_Bacterial_06_N_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumIndigo,
    #[serde(rename = "$Codex_Ent_Bacterial_09_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Volu - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVoluCyan,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Polonium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Cadmium_Name;")]
    #[strum(to_string = "Bacterium Omentum - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumOmentumLime,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Tungsten_Name;")]
    #[strum(to_string = "Bacterium Omentum - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumOmentumBlue,
    #[serde(rename = "$Codex_Ent_Bacterial_13_Tin_Name;")]
    #[strum(to_string = "Bacterium Verrata - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVerrataBlue,
    #[serde(rename = "$Codex_Ent_Bacterial_12_T_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusRed,
    #[serde(rename = "$Codex_Ent_Bacterial_01_T_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusRed,
    #[serde(rename = "$Codex_Ent_Bacterial_06_T_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumRed,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Niobium_Name;")]
    #[strum(to_string = "Bacterium Tela - Magenta")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaMagenta,
    #[serde(rename = "$Codex_Ent_Bacterial_03_Tin_Name;")]
    #[strum(to_string = "Bacterium Scopulum - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumScopulumMulberry,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Technetium_Name;")]
    #[strum(to_string = "Bacterium Informem - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Polonium_Name;")]
    #[strum(to_string = "Bacterium Informem - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemLime,
    #[serde(rename = "$Codex_Ent_Bacterial_09_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Volu - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVoluCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Technetium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisGold,
    #[serde(rename = "$Codex_Ent_Bacterial_12_A_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Polonium_Name;")]
    #[strum(to_string = "Bacterium Acies - Magenta")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesMagenta,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Molybdenum_Name;")]
    #[strum(to_string = "Bacterium Omentum - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumOmentumAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_06_G_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_01_G_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_01_K_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Acies - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Polonium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaOrange,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Antimony_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaCyan,
    #[serde(rename = "$Codex_Ent_Bacterial_06_L_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumSage,
    #[serde(rename = "$Codex_Ent_Bacterial_06_M_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Cadmium_Name;")]
    #[strum(to_string = "Bacterium Tela - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaGold,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Antimony_Name;")]
    #[strum(to_string = "Bacterium Informem - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemRed,
    #[serde(rename = "$Codex_Ent_Bacterial_09_Antimony_Name;")]
    #[strum(to_string = "Bacterium Volu - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVoluRed,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Informem - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemGold,
    #[serde(rename = "$Codex_Ent_Bacterial_12_G_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Molybdenum_Name;")]
    #[strum(to_string = "Bacterium Tela - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_01_A_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_01_M_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_01_F_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAurasusLime,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Acies - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesWhite,
    #[serde(rename = "$Codex_Ent_Bacterial_06_A_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Antimony_Name;")]
    #[strum(to_string = "Bacterium Acies - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesCyan,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Technetium_Name;")]
    #[strum(to_string = "Bacterium Acies - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesLime,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Acies - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAciesAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaRed,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Technetium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaGold,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaMulberry,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumVesiculaLime,
    #[serde(rename = "$Codex_Ent_Bacterial_06_F_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumLime,
    #[serde(rename = "$Codex_Ent_Bacterial_06_K_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumAlcyoneumGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Tin_Name;")]
    #[strum(to_string = "Bacterium Tela - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Informem - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Informem - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumInformemCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Antimony_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisLime,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumBullarisRed,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Tin_Name;")]
    #[strum(to_string = "Bacterium Omentum - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumOmentumRed,
    #[serde(rename = "$Codex_Ent_Bacterial_12_N_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusIndigo,
    #[serde(rename = "$Codex_Ent_Bacterial_12_K_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Mercury_Name;")]
    #[strum(to_string = "Bacterium Tela - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaOrange,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Tungsten_Name;")]
    #[strum(to_string = "Bacterium Tela - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumTelaGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_12_F_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusLime,
    #[serde(rename = "$Codex_Ent_Bacterial_12_M_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_12_L_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BacteriumCerbrusSage,
    // Bark Mounds
    #[serde(rename = "$Codex_Ent_Cone_Name;")]
    #[strum(to_string = "Bark Mounds")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    BarkMound,
    // Brain Trees
    #[serde(rename = "$Codex_Ent_SeedEFGH_01_Name;")]
    #[strum(to_string = "Aureum Brain Tree")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    AureumBrainTree,
    #[serde(rename = "$Codex_Ent_Seed_Name;")]
    #[strum(to_string = "Roseum Brain Tree")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    RoseumBrainTree,
    #[serde(rename = "$Codex_Ent_SeedABCD_01_Name;")]
    #[strum(to_string = "Gypseeum Brain Tree")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    GypseeumBrainTree,
    #[serde(rename = "$Codex_Ent_SeedEFGH_Name;")]
    #[strum(to_string = "Lividum Brain Tree")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    LividumBrainTree,
    // Cactoids
    #[serde(rename = "$Codex_Ent_Cactoid_01_F_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaCortexumYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_01_G_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaCortexumTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_01_L_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaCortexumMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_01_M_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaCortexumAmethyst,
    #[serde(rename = "$Codex_Ent_Cactoid_02_A_Name;")]
    #[strum(to_string = "Cactoida Lapis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisGreen,
    #[serde(rename = "$Codex_Ent_Cactoid_02_F_Name;")]
    #[strum(to_string = "Cactoida Lapis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_02_G_Name;")]
    #[strum(to_string = "Cactoida Lapis - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_02_L_Name;")]
    #[strum(to_string = "Cactoida Lapis - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_02_M_Name;")]
    #[strum(to_string = "Cactoida Lapis - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisAmethyst,
    #[serde(rename = "$Codex_Ent_Cactoid_02_T_Name;")]
    #[strum(to_string = "Cactoida Lapis - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaLapisOrange,
    #[serde(rename = "$Codex_Ent_Cactoid_03_G_Name;")]
    #[strum(to_string = "Cactoida Vermis - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaVermisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_03_L_Name;")]
    #[strum(to_string = "Cactoida Vermis - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaVermisMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_04_A_Name;")]
    #[strum(to_string = "Cactoida Pullulanta - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPullulantaGreen,
    #[serde(rename = "$Codex_Ent_Cactoid_04_F_Name;")]
    #[strum(to_string = "Cactoida Pullulanta - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPullulantaYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_05_A_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisGreen,
    #[serde(rename = "$Codex_Ent_Cactoid_05_F_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_05_G_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_05_L_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_05_M_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisAmethyst,
    #[serde(rename = "$Codex_Ent_Cactoid_05_N_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisSage,
    #[serde(rename = "$Codex_Ent_Cactoid_05_T_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    CactoidaPeperatisOrange,
    // Conchas
    #[serde(rename = "$Codex_Ent_Conchas_02_A_Name;")]
    #[strum(to_string = "Concha Aureolas - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasTeal,
    #[serde(rename = "$Codex_Ent_Conchas_02_K_Name;")]
    #[strum(to_string = "Concha Aureolas - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasRed,
    #[serde(rename = "$Codex_Ent_Conchas_02_N_Name;")]
    #[strum(to_string = "Concha Aureolas - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasEmerald,
    #[serde(rename = "$Codex_Ent_Conchas_02_F_Name;")]
    #[strum(to_string = "Concha Aureolas - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasGrey,
    #[serde(rename = "$Codex_Ent_Conchas_01_Mercury_Name;")]
    #[strum(to_string = "Concha Renibus - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusMulberry,
    #[serde(rename = "$Codex_Ent_Conchas_03_F_Name;")]
    #[strum(to_string = "Concha Labiata - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaLabiataGrey,
    #[serde(rename = "$Codex_Ent_Conchas_01_Cadmium_Name;")]
    #[strum(to_string = "Concha Renibus - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusRed,
    #[serde(rename = "$Codex_Ent_Conchas_01_Tin_Name;")]
    #[strum(to_string = "Concha Renibus - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusAquamarine,
    #[serde(rename = "$Codex_Ent_Conchas_01_Tungsten_Name;")]
    #[strum(to_string = "Concha Renibus - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusWhite,
    #[serde(rename = "$Codex_Ent_Conchas_02_G_Name;")]
    #[strum(to_string = "Concha Aureolas - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasTurquoise,
    #[serde(rename = "$Codex_Ent_Conchas_02_L_Name;")]
    #[strum(to_string = "Concha Aureolas - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaAureolasOrange,
    #[serde(rename = "$Codex_Ent_Conchas_01_Molybdenum_Name;")]
    #[strum(to_string = "Concha Renibus - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusPeach,
    #[serde(rename = "$Codex_Ent_Conchas_01_Niobium_Name;")]
    #[strum(to_string = "Concha Renibus - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaRenibusBlue,
    #[serde(rename = "$Codex_Ent_Conchas_03_A_Name;")]
    #[strum(to_string = "Concha Labiata - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaLabiataTeal,
    #[serde(rename = "$Codex_Ent_Conchas_03_G_Name;")]
    #[strum(to_string = "Concha Labiata - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaLabiataTurquoise,
    #[serde(rename = "$Codex_Ent_Conchas_03_K_Name;")]
    #[strum(to_string = "Concha Labiata - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ConchaLabiataRed,
    // Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_01_K_Name;")]
    #[strum(to_string = "Clypeus Lacrimam - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusLacrimamGrey,
    #[serde(rename = "$Codex_Ent_Clypeus_02_A_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusOrange,
    #[serde(rename = "$Codex_Ent_Clypeus_02_F_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusMauve,
    #[serde(rename = "$Codex_Ent_Clypeus_02_M_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusTurquoise,
    #[serde(rename = "$Codex_Ent_Clypeus_02_K_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusGrey,
    #[serde(rename = "$Codex_Ent_Clypeus_01_M_Name;")]
    #[strum(to_string = "Clypeus Lacrimam - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusLacrimamTurquoise,
    #[serde(rename = "$Codex_Ent_Clypeus_01_G_Name;")]
    #[strum(to_string = "Clypeus Lacrimam - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusLacrimamAmethyst,
    #[serde(rename = "$Codex_Ent_Clypeus_03_G_Name;")]
    #[strum(to_string = "Clypeus Speculumi - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusSpeculumiAmethyst,
    #[serde(rename = "$Codex_Ent_Clypeus_03_F_Name;")]
    #[strum(to_string = "Clypeus Speculumi - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusSpeculumiMauve,
    #[serde(rename = "$Codex_Ent_Clypeus_02_L_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusTeal,
    #[serde(rename = "$Codex_Ent_Clypeus_02_G_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ClypeusMargaritusAmethyst,
    // Electricae
    #[serde(rename = "$Codex_Ent_Electricae_01_Ruthenium_Name;")]
    #[strum(to_string = "Electricae Pluma - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaePlumaBlue,
    #[serde(rename = "$Codex_Ent_Electricae_02_Antimony_Name;")]
    #[strum(to_string = "Electricae Radialem - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaeRadialemCyan,
    #[serde(rename = "$Codex_Ent_Electricae_01_Yttrium_Name;")]
    #[strum(to_string = "Electricae Pluma - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaePlumaMulberry,
    #[serde(rename = "$Codex_Ent_Electricae_01_Polonium_Name;")]
    #[strum(to_string = "Electricae Pluma - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaePlumaCyan,
    #[serde(rename = "$Codex_Ent_Electricae_02_Ruthenium_Name;")]
    #[strum(to_string = "Electricae Radialem - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaeRadialemBlue,
    #[serde(rename = "$Codex_Ent_Electricae_02_Technetium_Name;")]
    #[strum(to_string = "Electricae Radialem - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaeRadialemAquamarine,
    #[serde(rename = "$Codex_Ent_Electricae_02_Tellurium_Name;")]
    #[strum(to_string = "Electricae Radialem - Magenta")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ElectricaeRadialemMagenta,
    // Fonticulus
    #[serde(rename = "$Codex_Ent_Fonticulus_01_K_Name;")]
    #[strum(to_string = "Fonticulua Segmentatus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaSegmentatusEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_D_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisTurquoise,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_N_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisSage,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_M_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaUpupamAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_T_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaUpupamOrange,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_L_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaDigitosMauve,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_G_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaLapidaTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_L_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaLapidaMauve,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_TTS_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaLapidaRed,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_L_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisMauve,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_A_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisGreen,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_K_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaDigitosEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_01_M_Name;")]
    #[strum(to_string = "Fonticulua Segmentatus - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaSegmentatusAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_F_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisYellow,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_K_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_G_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaUpupamTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_K_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaUpupamEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_F_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaUpupamYellow,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_G_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_M_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_T_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaCampestrisOrange,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_K_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaLapidaEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_M_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaLapidaAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_G_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaDigitosTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_M_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Amethyst")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FonticuluaDigitosAmethyst,
    // Fumerolas
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Cadmium_Name;")]
    #[strum(to_string = "Fumerola Carbosis - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaCarbosisOrange,
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Tungsten_Name;")]
    #[strum(to_string = "Fumerola Carbosis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaCarbosisYellow,
    #[serde(rename = "$Codex_Ent_Fumerolas_02_Tin_Name;")]
    #[strum(to_string = "Fumerola Extremus - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaExtremusPeach,
    #[serde(rename = "$Codex_Ent_Fumerolas_03_Niobium_Name;")]
    #[strum(to_string = "Fumerola Nitris - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaNitrisRed,
    // Fumaroles
    #[serde(rename = "$Codex_Ent_Fumarole_SilicateVapourGeysers_Name;")]
    #[strum(to_string = "Silicate Vapour Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    FumaroleSilicateVapour,
    #[serde(rename = "$Codex_Ent_Fumarole_SulphurDioxideMagma_Name;")]
    #[strum(to_string = "Sulphur Dioxide Fumarole")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::GeologyAndAnomalies)]
    FumaroleSulphurDioxide,
    #[serde(rename = "$Codex_Ent_Fumerolas_02_Niobium_Name;")]
    #[strum(to_string = "Fumerola Extremus - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaExtremusWhite,
    #[serde(rename = "$Codex_Ent_Fumerolas_03_Mercury_Name;")]
    #[strum(to_string = "Fumerola Nitris - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaNitrisPeach,
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Niobium_Name;")]
    #[strum(to_string = "Fumerola Carbosis - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaCarbosisCobalt,
    #[serde(rename = "$Codex_Ent_Fumerolas_04_Tungsten_Name;")]
    #[strum(to_string = "Fumerola Aquatis - Cobalt")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FumerolaAquatisCobalt,
    // Fungoids
    #[serde(rename = "$Codex_Ent_Fungoids_03_Antimony_Name;")]
    #[strum(to_string = "Fungoida Bullarum - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaBullarumRed,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Niobium_Name;")]
    #[strum(to_string = "Fungoida Gelata - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataGreen,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Polonium_Name;")]
    #[strum(to_string = "Fungoida Setisis - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisWhite,
    #[serde(rename = "$Codex_Ent_Fungoids_03_Yttrium_Name;")]
    #[strum(to_string = "Fungoida Bullarum - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaBullarumOrange,
    #[serde(rename = "$Codex_Ent_Fungoids_03_Polonium_Name;")]
    #[strum(to_string = "Fungoida Bullarum - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaBullarumMulberry,
    #[serde(rename = "$Codex_Ent_Fungoids_03_Ruthenium_Name;")]
    #[strum(to_string = "Fungoida Bullarum - Magenta")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaBullarumMagenta,
    #[serde(rename = "$Codex_Ent_Fungoids_03_Tellurium_Name;")]
    #[strum(to_string = "Fungoida Bullarum - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaBullarumGold,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Mercury_Name;")]
    #[strum(to_string = "Fungoida Gelata - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataLime,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Tungsten_Name;")]
    #[strum(to_string = "Fungoida Gelata - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataOrange,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Molybdenum_Name;")]
    #[strum(to_string = "Fungoida Gelata - Mulberry")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataMulberry,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Niobium_Name;")]
    #[strum(to_string = "Fungoida Stabitis - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisWhite,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Cadmium_Name;")]
    #[strum(to_string = "Fungoida Gelata - Cyan")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataCyan,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Antimony_Name;")]
    #[strum(to_string = "Fungoida Setisis - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisPeach,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Technetium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisLime,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Ruthenium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisGold,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Tellurium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisYellow,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Molybdenum_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Magenta")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisMagenta,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Yttrium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaSetisisOrange,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Cadmium_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisBlue,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Mercury_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisGreen,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Tin_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisOrange,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Tungsten_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaStabitisPeach,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Tin_Name;")]
    #[strum(to_string = "Fungoida Gelata - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FungoidaGelataRed,
    // Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_02_M_Name;")]
    #[strum(to_string = "Frutexa Acus - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaAcusGrey,
    #[serde(rename = "$Codex_Ent_Shrubs_03_N_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaMetallicumRed,
    #[serde(rename = "$Codex_Ent_Shrubs_04_F_Name;")]
    #[strum(to_string = "Frutexa Flammasis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlammasisGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_04_M_Name;")]
    #[strum(to_string = "Frutexa Flammasis - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlammasisGrey,
    #[serde(rename = "$Codex_Ent_Shrubs_04_N_Name;")]
    #[strum(to_string = "Frutexa Flammasis - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlammasisRed,
    #[serde(rename = "$Codex_Ent_Shrubs_03_G_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaMetallicumEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_01_G_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlabellumEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_03_M_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaMetallicumGrey,
    #[serde(rename = "$Codex_Ent_Shrubs_01_L_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlabellumTeal,
    #[serde(rename = "$Codex_Ent_Shrubs_02_G_Name;")]
    #[strum(to_string = "Frutexa Acus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaAcusEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_03_F_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaMetallicumGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_03_L_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaMetallicumTeal,
    #[serde(rename = "$Codex_Ent_Shrubs_04_G_Name;")]
    #[strum(to_string = "Frutexa Flammasis - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlammasisEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_01_F_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaFlabellumGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_02_F_Name;")]
    #[strum(to_string = "Frutexa Acus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaAcusGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_06_G_Name;")]
    #[strum(to_string = "Frutexa Sponsae - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    FrutexaSponsaeEmerald,
    // Osseus
    #[serde(rename = "$Codex_Ent_Osseus_01_A_Name;")]
    #[strum(to_string = "Osseus Fractus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusFractusLime,
    #[serde(rename = "$Codex_Ent_Osseus_02_Niobium_Name;")]
    #[strum(to_string = "Osseus Discus - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusAquamarine,
    #[serde(rename = "$Codex_Ent_Osseus_06_K_Name;")]
    #[strum(to_string = "Osseus Pellebantus - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPellebantusIndigo,
    #[serde(rename = "$Codex_Ent_Osseus_02_Cadmium_Name;")]
    #[strum(to_string = "Osseus Discus - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusWhite,
    #[serde(rename = "$Codex_Ent_Osseus_04_Antimony_Name;")]
    #[strum(to_string = "Osseus Pumice - White")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPumiceWhite,
    #[serde(rename = "$Codex_Ent_Osseus_04_Tellurium_Name;")]
    #[strum(to_string = "Osseus Pumice - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPumiceGreen,
    #[serde(rename = "$Codex_Ent_Osseus_04_Ruthenium_Name;")]
    #[strum(to_string = "Osseus Pumice - Gold")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPumiceGold,
    #[serde(rename = "$Codex_Ent_Osseus_01_T_Name;")]
    #[strum(to_string = "Osseus Fractus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusFractusEmerald,
    #[serde(rename = "$Codex_Ent_Osseus_01_F_Name;")]
    #[strum(to_string = "Osseus Fractus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusFractusTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_01_G_Name;")]
    #[strum(to_string = "Osseus Fractus - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusFractusGrey,
    #[serde(rename = "$Codex_Ent_Osseus_02_Tin_Name;")]
    #[strum(to_string = "Osseus Discus - Blue")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusBlue,
    #[serde(rename = "$Codex_Ent_Osseus_02_Tungsten_Name;")]
    #[strum(to_string = "Osseus Discus - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusRed,
    #[serde(rename = "$Codex_Ent_Osseus_02_Mercury_Name;")]
    #[strum(to_string = "Osseus Discus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusLime,
    #[serde(rename = "$Codex_Ent_Osseus_02_Molybdenum_Name;")]
    #[strum(to_string = "Osseus Discus - Peach")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusDiscusPeach,
    #[serde(rename = "$Codex_Ent_Osseus_03_A_Name;")]
    #[strum(to_string = "Osseus Spiralis - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusSpiralisLime,
    #[serde(rename = "$Codex_Ent_Osseus_03_T_Name;")]
    #[strum(to_string = "Osseus Spiralis - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusSpiralisEmerald,
    #[serde(rename = "$Codex_Ent_Osseus_03_G_Name;")]
    #[strum(to_string = "Osseus Spiralis - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusSpiralisGrey,
    #[serde(rename = "$Codex_Ent_Osseus_03_K_Name;")]
    #[strum(to_string = "Osseus Spiralis - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusSpiralisIndigo,
    #[serde(rename = "$Codex_Ent_Osseus_03_F_Name;")]
    #[strum(to_string = "Osseus Spiralis - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusSpiralisTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_01_K_Name;")]
    #[strum(to_string = "Osseus Fractus - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusFractusIndigo,
    #[serde(rename = "$Codex_Ent_Osseus_04_Technetium_Name;")]
    #[strum(to_string = "Osseus Pumice - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPumiceLime,
    #[serde(rename = "$Codex_Ent_Osseus_04_Yttrium_Name;")]
    #[strum(to_string = "Osseus Pumice - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPumiceYellow,
    #[serde(rename = "$Codex_Ent_Osseus_05_A_Name;")]
    #[strum(to_string = "Osseus Cornibus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusCornibusLime,
    #[serde(rename = "$Codex_Ent_Osseus_05_F_Name;")]
    #[strum(to_string = "Osseus Cornibus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusCornibusTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_06_F_Name;")]
    #[strum(to_string = "Osseus Pellebantus - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPellebantusTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_06_G_Name;")]
    #[strum(to_string = "Osseus Pellebantus - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    OsseusPellebantusGrey,
    // Recepta
    #[serde(rename = "$Codex_Ent_Recepta_01_M_Name;")]
    #[strum(to_string = "Recepta Umbrux - Maroon")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaUmbruxMaroon,
    #[serde(rename = "$Codex_Ent_Recepta_02_Tin_Name;")]
    #[strum(to_string = "Recepta Deltahedronix - Orange")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaDeltahedronixOrange,
    #[serde(rename = "$Codex_Ent_Recepta_01_K_Name;")]
    #[strum(to_string = "Recepta Umbrux - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaUmbruxRed,
    #[serde(rename = "$Codex_Ent_Recepta_03_Antimony_Name;")]
    #[strum(to_string = "Recepta Conditivus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaConditivusLime,
    #[serde(rename = "$Codex_Ent_Recepta_01_F_Name;")]
    #[strum(to_string = "Recepta Umbrux - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaUmbruxMauve,
    #[serde(rename = "$Codex_Ent_Recepta_01_TTS_Name;")]
    #[strum(to_string = "Recepta Umbrux - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaUmbruxSage,
    #[serde(rename = "$Codex_Ent_Recepta_03_Technetium_Name;")]
    #[strum(to_string = "Recepta Conditivus - Aquamarine")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaConditivusAquamarine,
    #[serde(rename = "$Codex_Ent_Recepta_03_Yttrium_Name;")]
    #[strum(to_string = "Recepta Conditivus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    ReceptaConditivusGreen,
    // Radicoida
    #[serde(rename = "$Codex_Ent_Ingensradices_Unicus_Name;")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[strum(to_string = "Radicoida Unica")]
    RadicoidaUnica,
    // Rubeum
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[serde(rename = "$Codex_Ent_SphereEFGH_01_Name;")]
    #[strum(to_string = "Rubeum Bioluminescent Anemone")]
    RubeumBioluminescentAnemone,
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[serde(rename = "$Codex_Ent_SphereEFGH_Name;")]
    #[strum(to_string = "Blatteum Bioluminescent Anemone")]
    BlatteumBioluminescentAnemone,
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    #[serde(rename = "$Codex_Ent_SphereEFGH_02_Name;")]
    #[strum(to_string = "Prasinum Bioluminescent Anemone")]
    PrasinumBioluminescentAnemone,
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    // Stratum
    #[serde(rename = "$Codex_Ent_Stratum_03_K_Name;")]
    #[strum(to_string = "Stratum Laminamus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumLaminamusLime,
    #[serde(rename = "$Codex_Ent_Stratum_05_K_Name;")]
    #[strum(to_string = "Stratum Limaxus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumLimaxusLime,
    #[serde(rename = "$Codex_Ent_Stratum_05_M_Name;")]
    #[strum(to_string = "Stratum Limaxus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumLimaxusGreen,
    #[serde(rename = "$Codex_Ent_Stratum_08_F_Name;")]
    #[strum(to_string = "Stratum Frigus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumFrigusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_04_F_Name;")]
    #[strum(to_string = "Stratum Araneamus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumAraneamusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_08_M_Name;")]
    #[strum(to_string = "Stratum Frigus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumFrigusGreen,
    #[serde(rename = "$Codex_Ent_Stratum_07_T_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumTectonicasGrey,
    #[serde(rename = "$Codex_Ent_Stratum_01_F_Name;")]
    #[strum(to_string = "Stratum Excutitus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumExcutitusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_01_K_Name;")]
    #[strum(to_string = "Stratum Excutitus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumExcutitusLime,
    #[serde(rename = "$Codex_Ent_Stratum_02_F_Name;")]
    #[strum(to_string = "Stratum Paleas - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumPaleasEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_02_K_Name;")]
    #[strum(to_string = "Stratum Paleas - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumPaleasLime,
    #[serde(rename = "$Codex_Ent_Stratum_02_L_Name;")]
    #[strum(to_string = "Stratum Paleas - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumPaleasTurquoise,
    #[serde(rename = "$Codex_Ent_Stratum_02_M_Name;")]
    #[strum(to_string = "Stratum Paleas - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumPaleasGreen,
    #[serde(rename = "$Codex_Ent_Stratum_06_F_Name;")]
    #[strum(to_string = "Stratum Cucumisis - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumCucumisisEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_06_K_Name;")]
    #[strum(to_string = "Stratum Cucumisis - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumCucumisisLime,
    #[serde(rename = "$Codex_Ent_Stratum_07_F_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumTectonicasEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_07_K_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumTectonicasLime,
    #[serde(rename = "$Codex_Ent_Stratum_07_L_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumTectonicasTurquoise,
    #[serde(rename = "$Codex_Ent_Stratum_03_F_Name;")]
    #[strum(to_string = "Stratum Laminamus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumLaminamusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_07_M_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumTectonicasGreen,
    #[serde(rename = "$Codex_Ent_Stratum_05_F_Name;")]
    #[strum(to_string = "Stratum Limaxus - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    StratumLimaxusEmerald,
    // Tubus
    #[serde(rename = "$Codex_Ent_Tubus_02_T_Name;")]
    #[strum(to_string = "Tubus Sororibus - Mauve")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusSororibusMauve,
    #[serde(rename = "$Codex_Ent_Tubus_02_M_Name;")]
    #[strum(to_string = "Tubus Sororibus - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusSororibusTeal,
    #[serde(rename = "$Codex_Ent_Tubus_04_A_Name;")]
    #[strum(to_string = "Tubus Rosarium - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusRosariumIndigo,
    #[serde(rename = "$Codex_Ent_Tubus_03_F_Name;")]
    #[strum(to_string = "Tubus Cavas - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCavasGrey,
    #[serde(rename = "$Codex_Ent_Tubus_03_K_Name;")]
    #[strum(to_string = "Tubus Cavas - Maroon")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCavasMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_04_G_Name;")]
    #[strum(to_string = "Tubus Rosarium - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusRosariumRed,
    #[serde(rename = "$Codex_Ent_Tubus_01_F_Name;")]
    #[strum(to_string = "Tubus Conifer - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusConiferGrey,
    #[serde(rename = "$Codex_Ent_Tubus_01_G_Name;")]
    #[strum(to_string = "Tubus Conifer - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusConiferRed,
    #[serde(rename = "$Codex_Ent_Tubus_01_A_Name;")]
    #[strum(to_string = "Tubus Conifer - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusConiferIndigo,
    #[serde(rename = "$Codex_Ent_Tubus_01_M_Name;")]
    #[strum(to_string = "Tubus Conifer - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusConiferTeal,
    #[serde(rename = "$Codex_Ent_Tubus_02_K_Name;")]
    #[strum(to_string = "Tubus Sororibus - Maroon")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusSororibusMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_03_A_Name;")]
    #[strum(to_string = "Tubus Cavas - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCavasIndigo,
    #[serde(rename = "$Codex_Ent_Tubus_03_G_Name;")]
    #[strum(to_string = "Tubus Cavas - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCavasRed,
    #[serde(rename = "$Codex_Ent_Tubus_04_F_Name;")]
    #[strum(to_string = "Tubus Rosarium - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusRosariumGrey,
    #[serde(rename = "$Codex_Ent_Tubus_04_K_Name;")]
    #[strum(to_string = "Tubus Rosarium - Maroon")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusRosariumMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_04_L_Name;")]
    #[strum(to_string = "Tubus Rosarium - Turquoise")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusRosariumTurquoise,
    #[serde(rename = "$Codex_Ent_Tubus_05_F_Name;")]
    #[strum(to_string = "Tubus Compagibus - Grey")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCompagibusGrey,
    #[serde(rename = "$Codex_Ent_Tubus_05_G_Name;")]
    #[strum(to_string = "Tubus Compagibus - Red")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCompagibusRed,
    #[serde(rename = "$Codex_Ent_Tubus_05_K_Name;")]
    #[strum(to_string = "Tubus Compagibus - Maroon")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCompagibusMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_05_A_Name;")]
    #[strum(to_string = "Tubus Compagibus - Indigo")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TubusCompagibusIndigo,
    // Tussocks
    #[serde(rename = "$Codex_Ent_Tussocks_08_M_Name;")]
    #[strum(to_string = "Tussock Albata - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockAlbataEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_09_T_Name;")]
    #[strum(to_string = "Tussock Propagito - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoTeal,
    #[serde(rename = "$Codex_Ent_Tussocks_02_K_Name;")]
    #[strum(to_string = "Tussock Ventusa - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVentusaGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_07_G_Name;")]
    #[strum(to_string = "Tussock Serrati - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockSerratiLime,
    #[serde(rename = "$Codex_Ent_Tussocks_09_L_Name;")]
    #[strum(to_string = "Tussock Propagito - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoSage,
    #[serde(rename = "$Codex_Ent_Tussocks_15_K_Name;")]
    #[strum(to_string = "Tussock Capillum - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCapillumGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_05_T_Name;")]
    #[strum(to_string = "Tussock Catena - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaTeal,
    #[serde(rename = "$Codex_Ent_Tussocks_09_M_Name;")]
    #[strum(to_string = "Tussock Propagito - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_15_G_Name;")]
    #[strum(to_string = "Tussock Capillum - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCapillumLime,
    #[serde(rename = "$Codex_Ent_Tussocks_05_F_Name;")]
    #[strum(to_string = "Tussock Catena - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_01_F_Name;")]
    #[strum(to_string = "Tussock Pennata - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPennataYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_04_M_Name;")]
    #[strum(to_string = "Tussock Cultro - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCultroEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_05_M_Name;")]
    #[strum(to_string = "Tussock Catena - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_05_K_Name;")]
    #[strum(to_string = "Tussock Catena - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_06_K_Name;")]
    #[strum(to_string = "Tussock Pennatis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPennatisGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_07_F_Name;")]
    #[strum(to_string = "Tussock Serrati - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockSerratiYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_02_F_Name;")]
    #[strum(to_string = "Tussock Ventusa - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVentusaYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_03_G_Name;")]
    #[strum(to_string = "Tussock Ignis - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockIgnisLime,
    #[serde(rename = "$Codex_Ent_Tussocks_04_G_Name;")]
    #[strum(to_string = "Tussock Cultro - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCultroLime,
    #[serde(rename = "$Codex_Ent_Tussocks_04_K_Name;")]
    #[strum(to_string = "Tussock Cultro - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCultroGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_04_L_Name;")]
    #[strum(to_string = "Tussock Cultro - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCultroSage,
    #[serde(rename = "$Codex_Ent_Tussocks_05_G_Name;")]
    #[strum(to_string = "Tussock Catena - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaLime,
    #[serde(rename = "$Codex_Ent_Tussocks_05_L_Name;")]
    #[strum(to_string = "Tussock Catena - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCatenaSage,
    #[serde(rename = "$Codex_Ent_Tussocks_06_L_Name;")]
    #[strum(to_string = "Tussock Pennatis - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPennatisSage,
    #[serde(rename = "$Codex_Ent_Tussocks_08_F_Name;")]
    #[strum(to_string = "Tussock Albata - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockAlbataYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_08_G_Name;")]
    #[strum(to_string = "Tussock Albata - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockAlbataLime,
    #[serde(rename = "$Codex_Ent_Tussocks_08_K_Name;")]
    #[strum(to_string = "Tussock Albata - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockAlbataGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_09_K_Name;")]
    #[strum(to_string = "Tussock Propagito - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_10_F_Name;")]
    #[strum(to_string = "Tussock Divisa - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockDivisaYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_10_K_Name;")]
    #[strum(to_string = "Tussock Divisa - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockDivisaGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_10_M_Name;")]
    #[strum(to_string = "Tussock Divisa - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockDivisaEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_10_T_Name;")]
    #[strum(to_string = "Tussock Divisa - Teal")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockDivisaTeal,
    #[serde(rename = "$Codex_Ent_Tussocks_10_G_Name;")]
    #[strum(to_string = "Tussock Divisa - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockDivisaLime,
    #[serde(rename = "$Codex_Ent_Tussocks_12_M_Name;")]
    #[strum(to_string = "Tussock Triticum - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockTriticumEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_13_F_Name;")]
    #[strum(to_string = "Tussock Stigmasis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockStigmasisYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_14_M_Name;")]
    #[strum(to_string = "Tussock Virgam - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVirgamEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_15_F_Name;")]
    #[strum(to_string = "Tussock Capillum - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCapillumYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_14_K_Name;")]
    #[strum(to_string = "Tussock Virgam - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVirgamGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_14_L_Name;")]
    #[strum(to_string = "Tussock Virgam - Sage")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVirgamSage,
    #[serde(rename = "$Codex_Ent_Tussocks_11_G_Name;")]
    #[strum(to_string = "Tussock Caputus - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCaputusLime,
    #[serde(rename = "$Codex_Ent_Tussocks_12_G_Name;")]
    #[strum(to_string = "Tussock Triticum - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockTriticumLime,
    #[serde(rename = "$Codex_Ent_Tussocks_14_G_Name;")]
    #[strum(to_string = "Tussock Virgam - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVirgamLime,
    #[serde(rename = "$Codex_Ent_Tussocks_12_F_Name;")]
    #[strum(to_string = "Tussock Triticum - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockTriticumYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_04_F_Name;")]
    #[strum(to_string = "Tussock Cultro - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCultroYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_11_F_Name;")]
    #[strum(to_string = "Tussock Caputus - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCaputusYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_03_M_Name;")]
    #[strum(to_string = "Tussock Ignis - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockIgnisEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_11_K_Name;")]
    #[strum(to_string = "Tussock Caputus - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCaputusGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_03_K_Name;")]
    #[strum(to_string = "Tussock Ignis - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockIgnisGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_07_K_Name;")]
    #[strum(to_string = "Tussock Serrati - Green")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockSerratiGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_02_G_Name;")]
    #[strum(to_string = "Tussock Ventusa - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockVentusaLime,
    #[serde(rename = "$Codex_Ent_Tussocks_03_F_Name;")]
    #[strum(to_string = "Tussock Ignis - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockIgnisYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_09_F_Name;")]
    #[strum(to_string = "Tussock Propagito - Yellow")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_09_G_Name;")]
    #[strum(to_string = "Tussock Propagito - Lime")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockPropagitoLime,
    #[serde(rename = "$Codex_Ent_Tussocks_15_M_Name;")]
    #[strum(to_string = "Tussock Capillum - Emerald")]
    #[CodexCategory(category =CodexCategory::Biology, sub_category = CodexSubCategory::OrganicStructures)]
    TussockCapillumEmerald,
    //
    // Thargoid / Guadian
    //
    #[serde(rename = "$Codex_Ent_Guardian_Beacons_Name;")]
    #[strum(to_string = "Guardian Beacon")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::GuardianObjects)]
    GuardianBeacon,
    #[serde(rename = "$Codex_Ent_Guardian_Data_Logs_Name;")]
    #[strum(to_string = "Guardian Codex")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::GuardianObjects)]
    GuardianCodex,
    #[serde(rename = "$Codex_Ent_Guardian_Terminal_Name;")]
    #[strum(to_string = "Guardian Data Terminal")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::GuardianObjects)]
    GuardianDataTerminal,
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Spikes_Name;")]
    #[strum(to_string = "Thargoid Barnacle Barbs")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::ThargoidObjects)]
    ThargoidBarnacleBarbs,
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_01_Name;")]
    #[strum(to_string = "Common Thargoid Barnacle")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::ThargoidObjects)]
    CommonThargoidBarnacle,
    #[serde(rename = "$Codex_Ent_Basilisk_Name;")]
    #[strum(to_string = "Thargoid Interceptor Basilisk")]
    #[CodexCategory(category =CodexCategory::Xenological, sub_category = CodexSubCategory::ThargoidObjects)]
    ThargoidInterceptorBasilisk,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Copy,
    Display,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    EnumIter,
)]
pub enum CodexSubCategory {
    #[serde(rename = "$Codex_SubCategory_Stars;")]
    #[strum(to_string = "Stars")]
    Stars,
    #[serde(rename = "$Codex_SubCategory_Terrestrials;")]
    #[strum(to_string = "Terrestrial planets")]
    TerrestrialPlanets,
    #[serde(rename = "$Codex_SubCategory_Gas_Giants;")]
    #[strum(to_string = "Gas giant planets")]
    GasGiants,
    #[serde(rename = "$Codex_SubCategory_Organic_Structures;")]
    #[strum(to_string = "Organic structures")]
    OrganicStructures,
    #[serde(rename = "$Codex_SubCategory_Geology_and_Anomalies;")]
    #[strum(to_string = "Geology and anomalies")]
    GeologyAndAnomalies,
    #[serde(rename = "$Codex_SubCategory_Thargoid;")]
    #[strum(to_string = "Thargoid objects")]
    ThargoidObjects,
    #[serde(rename = "$Codex_SubCategory_Guardian;")]
    #[strum(to_string = "Guardian objects")]
    GuardianObjects,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Copy,
    Display,
    Eq,
    Hash,
    PartialEq,
    Ord,
    PartialOrd,
    EnumIter,
)]
pub enum CodexCategory {
    #[serde(rename = "$Codex_Category_StellarBodies;")]
    #[strum(to_string = "Astronomical Bodies")]
    StellarBodies,
    #[serde(rename = "$Codex_Category_Biology;")]
    #[strum(to_string = "Biological and Geological")]
    Biology,
    #[serde(rename = "$Codex_Category_Civilisations;")]
    #[strum(to_string = "Xenological")]
    Xenological,
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    Copy,
    Display,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
    EnumIter,
)]
pub enum CodexRegion {
    #[serde(rename = "$Codex_RegionName_1;")]
    #[strum(to_string = "Galactic Centre")]
    GalacticCentre,
    #[serde(rename = "$Codex_RegionName_2;")]
    #[strum(to_string = "Empyrean Straits")]
    EmpyreanStraits,
    #[serde(rename = "$Codex_RegionName_3;")]
    #[strum(to_string = "Ryker's Hope")]
    RykersHope,
    #[serde(rename = "$Codex_RegionName_4;")]
    #[strum(to_string = "Odin's Hold")]
    OdinsHold,
    #[serde(rename = "$Codex_RegionName_5;")]
    #[strum(to_string = "Norma Arm")]
    NormaArm,
    #[serde(rename = "$Codex_RegionName_6;")]
    #[strum(to_string = "Arcadian Stream")]
    ArcadianStream,
    #[serde(rename = "$Codex_RegionName_7;")]
    #[strum(to_string = "Izanami")]
    Izanami,
    #[serde(rename = "$Codex_RegionName_8;")]
    #[strum(to_string = "Inner Orion-Perseus Conflux")]
    InnerOrionPerseusConflux,
    #[serde(rename = "$Codex_RegionName_9;")]
    #[strum(to_string = "Inner Scutum-Centaurus Arm")]
    InnerScutumCentaurusArm,
    #[serde(rename = "$Codex_RegionName_10;")]
    #[strum(to_string = "Norma Expanse")]
    NormaExpanse,
    #[serde(rename = "$Codex_RegionName_11;")]
    #[strum(to_string = "Trojan Belt")]
    TrojanBelt,
    #[serde(rename = "$Codex_RegionName_12;")]
    #[strum(to_string = "The Veils")]
    TheVeils,
    #[serde(rename = "$Codex_RegionName_13;")]
    #[strum(to_string = "Newton's Vault")]
    NewtonsVault,
    #[serde(rename = "$Codex_RegionName_14;")]
    #[strum(to_string = "The Conduit")]
    TheConduit,
    #[serde(rename = "$Codex_RegionName_15;")]
    #[strum(to_string = "Outer Orion-Perseus Conflux")]
    OuterOrionPerseusConflux,
    #[serde(rename = "$Codex_RegionName_16;")]
    #[strum(to_string = "Orion-Cygnus Arm")]
    OrionCygnusArm,
    #[serde(rename = "$Codex_RegionName_17;")]
    #[strum(to_string = "Temple")]
    Temple,
    #[serde(rename = "$Codex_RegionName_18;")]
    #[strum(to_string = "Inner Orion Spur")]
    InnerOrionSpur,
    #[serde(rename = "$Codex_RegionName_19;")]
    #[strum(to_string = "Hawking's Gap")]
    HawkingsGap,
    #[serde(rename = "$Codex_RegionName_20;")]
    #[strum(to_string = "Dryman’s Point")]
    DrymansPoint,
    #[serde(rename = "$Codex_RegionName_21;")]
    #[strum(to_string = "Sagittarius-Carina Arm")]
    SagittariusCarinaArm,
    #[serde(rename = "$Codex_RegionName_22;")]
    #[strum(to_string = "Mare Somnia")]
    MareSomnia,
    #[serde(rename = "$Codex_RegionName_23;")]
    #[strum(to_string = "Acheron")]
    Acheron,
    #[serde(rename = "$Codex_RegionName_24;")]
    #[strum(to_string = "Formorian Frontier")]
    FormorianFrontier,
    #[serde(rename = "$Codex_RegionName_25;")]
    #[strum(to_string = "Hieronymus Delta")]
    HieronymusDelta,
    #[serde(rename = "$Codex_RegionName_26;")]
    #[strum(to_string = "Outer Scutum-Centaurus Arm")]
    OuterScutumCentaurusArm,
    #[serde(rename = "$Codex_RegionName_27;")]
    #[strum(to_string = "Outer Arm")]
    OuterArm,
    #[serde(rename = "$Codex_RegionName_28;")]
    #[strum(to_string = "Aquila’s Halo")]
    AquilasHalo,
    #[serde(rename = "$Codex_RegionName_29;")]
    #[strum(to_string = "Errant Marches")]
    ErrantMarches,
    #[serde(rename = "$Codex_RegionName_30;")]
    #[strum(to_string = "Perseus Arm")]
    PerseusArm,
    #[serde(rename = "$Codex_RegionName_31;")]
    #[strum(to_string = "The Formidine Rift")]
    TheFormidineRift,
    #[serde(rename = "$Codex_RegionName_32;")]
    #[strum(to_string = "Vulcan Gate")]
    VulcanGate,
    #[serde(rename = "$Codex_RegionName_33;")]
    #[strum(to_string = "Elysian Shore")]
    ElysianShore,
    #[serde(rename = "$Codex_RegionName_34;")]
    #[strum(to_string = "Sanguineous Rim")]
    SanguineousRim,
    #[serde(rename = "$Codex_RegionName_35;")]
    #[strum(to_string = "Outer Orion Spur")]
    OuterOrionSpur,
    #[serde(rename = "$Codex_RegionName_36;")]
    #[strum(to_string = "Achilles’s Altar")]
    AchillessAltar,
    #[serde(rename = "$Codex_RegionName_37;")]
    #[strum(to_string = "Xibalba")]
    Xibalba,
    #[serde(rename = "$Codex_RegionName_38;")]
    #[strum(to_string = "Lyra’s Song")]
    LyrasSong,
    #[serde(rename = "$Codex_RegionName_39;")]
    #[strum(to_string = "Tenebrae")]
    Tenebrae,
    #[serde(rename = "$Codex_RegionName_40;")]
    #[strum(to_string = "The Abyss")]
    TheAbyss,
    #[serde(rename = "$Codex_RegionName_41;")]
    #[strum(to_string = "Kepler's Crest")]
    KeplersCrest,
    #[serde(rename = "$Codex_RegionName_42;")]
    #[strum(to_string = "The Void")]
    TheVoid,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum CodexGenus {
    #[serde(alias = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,
    #[serde(alias = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(alias = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,
    #[serde(alias = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,
    #[serde(alias = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,
    #[serde(alias = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,
    #[serde(alias = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,
    #[serde(alias = "$Codex_Ent_Fumerolas_Genus_Name;")]
    Fumerola,
    #[serde(alias = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,
    #[serde(alias = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,
    #[serde(alias = "$Codex_Ent_Ingensradices_Genus_Name;")]
    Radicoida,
    #[serde(alias = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,
    #[serde(alias = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,
    #[serde(alias = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,
    #[serde(alias = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,
    #[serde(alias = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,

    #[serde(alias = "$Codex_Ent_Cone_Name;")]
    #[strum(to_string = "Bark Mounds")]
    BarkMounds,
    #[serde(alias = "$Codex_Ent_Brancae_Name;")]
    #[strum(to_string = "Brain Trees")]
    BrainTrees,
    #[serde(alias = "$Codex_Ent_Sphere_Name;")]
    #[strum(to_string = "Luteolum Anemone")]
    LuteolumAnemone,
    #[serde(alias = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    #[strum(to_string = "Crystalline Shards")]
    CrystallineShards,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
pub enum CodexSpecies {
    #[serde(alias = "$Codex_Ent_Aleoids_01_Name;")]
    #[strum(to_string = "Aleoida Arcus")]
    AleoidaArcus,
    #[serde(alias = "$Codex_Ent_Aleoids_02_Name;")]
    #[strum(to_string = "Aleoida Coronamus")]
    AleoidaCoronamus,
    #[serde(alias = "$Codex_Ent_Aleoids_03_Name;")]
    #[strum(to_string = "Aleoida Spica")]
    AleoidaSpica,
    #[serde(alias = "$Codex_Ent_Aleoids_04_Name;")]
    #[strum(to_string = "Aleoida Laminiae")]
    AleoidaLaminiae,
    #[serde(alias = "$Codex_Ent_Aleoids_05_Name;")]
    #[strum(to_string = "Aleoida Gravis")]
    AleoidaGravis,
    #[serde(alias = "$Codex_Ent_Bacterial_01_Name;")]
    #[strum(to_string = "Bacterium Aurasus")]
    BacteriumAurasus,
    #[serde(alias = "$Codex_Ent_Bacterial_03_Name;")]
    #[strum(to_string = "Bacterium Scopulum")]
    BacteriumScopulum,
    #[serde(alias = "$Codex_Ent_Bacterial_04_Name;")]
    #[strum(to_string = "Bacterium Acies")]
    BacteriumAcies,
    #[serde(alias = "$Codex_Ent_Bacterial_05_Name;")]
    #[strum(to_string = "Bacterium Vesicula")]
    BacteriumVesicula,
    #[serde(alias = "$Codex_Ent_Bacterial_06_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum")]
    BacteriumAlcyoneum,
    #[serde(alias = "$Codex_Ent_Bacterial_07_Name;")]
    #[strum(to_string = "Bacterium Tela")]
    BacteriumTela,
    #[serde(alias = "$Codex_Ent_Bacterial_08_Name;")]
    #[strum(to_string = "Bacterium Informem")]
    BacteriumInformem,
    #[serde(alias = "$Codex_Ent_Bacterial_09_Name;")]
    #[strum(to_string = "Bacterium Volu")]
    BacteriumVolu,
    #[serde(alias = "$Codex_Ent_Bacterial_10_Name;")]
    #[strum(to_string = "Bacterium Bullaris")]
    BacteriumBullaris,
    #[serde(alias = "$Codex_Ent_Bacterial_11_Name;")]
    #[strum(to_string = "Bacterium Omentum")]
    BacteriumOmentum,
    #[serde(alias = "$Codex_Ent_Bacterial_12_Name;")]
    #[strum(to_string = "Bacterium Cerbrus")]
    BacteriumCerbrus,
    #[serde(alias = "$Codex_Ent_Bacterial_13_Name;")]
    #[strum(to_string = "Bacterium Verrata")]
    BacteriumVerrata,
    #[serde(alias = "$Codex_Ent_Cone_Name;")]
    #[strum(to_string = "Bark Mounds")]
    BarkMounds,
    #[serde(alias = "$Codex_Ent_SphereEFGH_Name;")]
    #[strum(to_string = "Blatteum Bioluminescent Anemone")]
    BioluminescentAnemoneBlatteum,
    #[serde(alias = "$Codex_Ent_SphereEFGH_01_Name;")]
    #[strum(to_string = "Rubeum Bioluminescent Anemone")]
    BioluminescentAnemonRubeum,
    #[serde(alias = "$Codex_Ent_SphereEFGH_02_Name;")]
    #[strum(to_string = "Prasinum Bioluminescent Anemone")]
    BioluminescentAnemonPrasinum,
    #[serde(alias = "$Codex_Ent_Seed_Name;")]
    #[strum(to_string = "Roseum Brain Tree")]
    BrainTreeRoseum,
    #[serde(alias = "$Codex_Ent_SeedABCD_01_Name;")]
    #[strum(to_string = "Gypseeum Brain Tree")]
    BrainTreeGypseeum,
    #[serde(alias = "$Codex_Ent_SeedEFGH_Name;")]
    #[strum(to_string = "Lividum Brain Tree")]
    BrainTreeLividum,
    #[serde(alias = "$Codex_Ent_Cactoid_01_Name;")]
    #[strum(to_string = "Cactoida Cortexum")]
    CactoidaCortexum,
    #[serde(alias = "$Codex_Ent_Cactoid_02_Name;")]
    #[strum(to_string = "Cactoida Lapis")]
    CactoidaLapis,
    #[serde(alias = "$Codex_Ent_Cactoid_03_Name;")]
    #[strum(to_string = "Cactoida Vermis")]
    CactoidaVermis,
    #[serde(alias = "$Codex_Ent_Cactoid_04_Name;")]
    #[strum(to_string = "Cactoida Pullulanta")]
    CactoidaPullulanta,
    #[serde(alias = "$Codex_Ent_Cactoid_05_Name;")]
    #[strum(to_string = "Cactoida Peperatis")]
    CactoidaPeperatis,
    #[serde(alias = "$Codex_Ent_Clypeus_01_Name;")]
    #[strum(to_string = "Clypeus Lacrimam")]
    ClypeusLacrimam,
    #[serde(alias = "$Codex_Ent_Clypeus_02_Name;")]
    #[strum(to_string = "Clypeus Margaritus")]
    ClypeusMargaritus,
    #[serde(alias = "$Codex_Ent_Clypeus_03_Name;")]
    #[strum(to_string = "Clypeus Speculumi")]
    ClypeusSpeculumi,
    #[serde(alias = "$Codex_Ent_Conchas_01_Name;")]
    #[strum(to_string = "Concha Renibus")]
    ConchaRenibus,
    #[serde(alias = "$Codex_Ent_Conchas_02_Name;")]
    #[strum(to_string = "Concha Aureolas")]
    ConchaAureolas,
    #[serde(alias = "$Codex_Ent_Conchas_03_Name;")]
    #[strum(to_string = "Concha Labiata")]
    ConchaLabiata,
    #[serde(alias = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    #[strum(to_string = "Crystalline Shards")]
    CrystallineShards,
    #[serde(alias = "$Codex_Ent_Electricae_01_Name;")]
    #[strum(to_string = "Electricae Pluma")]
    ElectricaePluma,
    #[serde(alias = "$Codex_Ent_Electricae_02_Name;")]
    #[strum(to_string = "Electricae Radialem")]
    ElectricaeRadialem,
    #[serde(alias = "$Codex_Ent_Fonticulus_01_Name;")]
    #[strum(to_string = "Fonticulua Segmentatus")]
    FonticuluaSegmentatus,
    #[serde(alias = "$Codex_Ent_Fonticulus_02_Name;")]
    #[strum(to_string = "Fonticulua Campestris")]
    FonticuluaCampestris,
    #[serde(alias = "$Codex_Ent_Fonticulus_03_Name;")]
    #[strum(to_string = "Fonticulua Upupam")]
    FonticuluaUpupam,
    #[serde(alias = "$Codex_Ent_Fonticulus_04_Name;")]
    #[strum(to_string = "Fonticulua Lapida")]
    FonticuluaLapida,
    #[serde(alias = "$Codex_Ent_Fonticulus_06_Name;")]
    #[strum(to_string = "Fonticulua Digitos")]
    FonticuluaDigitos,
    #[serde(alias = "$Codex_Ent_Shrubs_01_Name;")]
    #[strum(to_string = "Frutexa Flabellum")]
    FrutexaFlabellum,
    #[serde(alias = "$Codex_Ent_Shrubs_02_Name;")]
    #[strum(to_string = "Frutexa Acus")]
    FrutexaAcus,
    #[serde(alias = "$Codex_Ent_Shrubs_03_Name;")]
    #[strum(to_string = "Frutexa Metallicum")]
    FrutexaMetallicum,
    #[serde(alias = "$Codex_Ent_Shrubs_04_Name;")]
    #[strum(to_string = "Frutexa Flammasis")]
    FrutexaFlammasis,
    #[serde(alias = "$Codex_Ent_Shrubs_05_Name;")]
    #[strum(to_string = "Frutexa Fera")]
    FrutexaUnknown,
    #[serde(alias = "$Codex_Ent_Shrubs_06_Name;")]
    #[strum(to_string = "Frutexa Sponsae")]
    FrutexaSponsae,
    #[serde(alias = "$Codex_Ent_Fumerolas_01_Name;")]
    #[strum(to_string = "Fumerola Carbosis")]
    FumerolaCarbosis,
    #[serde(alias = "$Codex_Ent_Fumerolas_02_Name;")]
    #[strum(to_string = "Fumerola Extremus")]
    FumerolaExtremus,
    #[serde(alias = "$Codex_Ent_Fumerolas_03_Name;")]
    #[strum(to_string = "Fumerola Nitris")]
    FumerolaNitris,
    #[serde(alias = "$Codex_Ent_Fumerolas_04_Name;")]
    #[strum(to_string = "Fumerola Aquatis")]
    FumerolaAquatis,
    #[serde(alias = "$Codex_Ent_Fungoids_01_Name;")]
    #[strum(to_string = "Fungoida Setisis")]
    FungoidaSetisis,
    #[serde(alias = "$Codex_Ent_Fungoids_02_Name;")]
    #[strum(to_string = "Fungoida Stabitis")]
    FungoidaStabitis,
    #[serde(alias = "$Codex_Ent_Fungoids_03_Name;")]
    #[strum(to_string = "Fungoida Bullarum")]
    FungoidaBullarum,
    #[serde(alias = "$Codex_Ent_Fungoids_04_Name;")]
    #[strum(to_string = "Fungoida Gelata")]
    FungoidaGelata,
    #[serde(alias = "$Codex_Ent_Osseus_01_Name;")]
    #[strum(to_string = "Osseus Fractus")]
    OsseusFractus,
    #[serde(alias = "$Codex_Ent_Osseus_02_Name;")]
    #[strum(to_string = "Osseus Discus")]
    OsseusDiscus,
    #[serde(alias = "$Codex_Ent_Osseus_03_Name;")]
    #[strum(to_string = "Osseus Spiralis")]
    OsseusSpiralis,
    #[serde(alias = "$Codex_Ent_Osseus_04_Name;")]
    #[strum(to_string = "Osseus Pumice")]
    OsseusPumice,
    #[serde(alias = "$Codex_Ent_Osseus_05_Name;")]
    #[strum(to_string = "Osseus Cornibus")]
    OsseusCornibus,
    #[serde(alias = "$Codex_Ent_Osseus_06_Name;")]
    #[strum(to_string = "Osseus Pellebantus")]
    OsseusPellebantus,
    #[serde(alias = "$Codex_Ent_Ingensradices_Unicus_Name;")]
    #[strum(to_string = "Radicoida Unica")]
    RadicoidaUnica,
    #[serde(alias = "$Codex_Ent_Recepta_01_Name;")]
    #[strum(to_string = "Recepta Umbrux")]
    ReceptaUmbrux,
    #[serde(alias = "$Codex_Ent_Recepta_02_Name;")]
    #[strum(to_string = "Recepta Deltahedronix")]
    ReceptaDeltahedronix,
    #[serde(alias = "$Codex_Ent_Recepta_03_Name;")]
    #[strum(to_string = "Recepta Conditivus")]
    ReceptaConditivus,
    #[serde(alias = "$Codex_Ent_Stratum_01_Name;")]
    #[strum(to_string = "Stratum Excutitus")]
    StratumExcutitus,
    #[serde(alias = "$Codex_Ent_Stratum_02_Name;")]
    #[strum(to_string = "Stratum Paleas")]
    StratumPaleas,
    #[serde(alias = "$Codex_Ent_Stratum_03_Name;")]
    #[strum(to_string = "Stratum Laminamus")]
    StratumLaminamus,
    #[serde(alias = "$Codex_Ent_Stratum_04_Name;")]
    #[strum(to_string = "Stratum Araneamus")]
    StratumAraneamus,
    #[serde(alias = "$Codex_Ent_Stratum_05_Name;")]
    #[strum(to_string = "Stratum Limaxus")]
    StratumLimaxus,
    #[serde(alias = "$Codex_Ent_Stratum_06_Name;")]
    #[strum(to_string = "Stratum Cucumisis")]
    StratumCucumisis,
    #[serde(alias = "$Codex_Ent_Stratum_07_Name;")]
    #[strum(to_string = "Stratum Tectonicas")]
    StratumTectonicas,
    #[serde(alias = "$Codex_Ent_Stratum_08_Name;")]
    #[strum(to_string = "Stratum Frigus")]
    StratumFrigus,
    #[serde(alias = "$Codex_Ent_Tubus_01_Name;")]
    #[strum(to_string = "Tubus Conifer")]
    TubusConifer,
    #[serde(alias = "$Codex_Ent_Tubus_02_Name;")]
    #[strum(to_string = "Tubus Sororibus")]
    TubusSororibus,
    #[serde(alias = "$Codex_Ent_Tubus_03_Name;")]
    #[strum(to_string = "Tubus Cavas")]
    TubusCavas,
    #[serde(alias = "$Codex_Ent_Tubus_04_Name;")]
    #[strum(to_string = "Tubus Rosarium")]
    TubusRosarium,
    #[serde(alias = "$Codex_Ent_Tubus_05_Name;")]
    #[strum(to_string = "Tubus Compagibus")]
    TubusCompagibus,
    #[serde(alias = "$Codex_Ent_Tussocks_01_Name;")]
    #[strum(to_string = "Tussock Pennata")]
    TussockPennata,
    #[serde(alias = "$Codex_Ent_Tussocks_02_Name;")]
    #[strum(to_string = "Tussock Ventusa")]
    TussockVentusa,
    #[serde(alias = "$Codex_Ent_Tussocks_03_Name;")]
    #[strum(to_string = "Tussock Ignis")]
    TussockIgnis,
    #[serde(alias = "$Codex_Ent_Tussocks_04_Name;")]
    #[strum(to_string = "Tussock Cultro")]
    TussockCultro,
    #[serde(alias = "$Codex_Ent_Tussocks_05_Name;")]
    #[strum(to_string = "Tussock Catena")]
    TussockCatena,
    #[serde(alias = "$Codex_Ent_Tussocks_06_Name;")]
    #[strum(to_string = "Tussock Pennatis")]
    TussockPennatis,
    #[serde(alias = "$Codex_Ent_Tussocks_07_Name;")]
    #[strum(to_string = "Tussock Serrati")]
    TussockSerrati,
    #[serde(alias = "$Codex_Ent_Tussocks_08_Name;")]
    #[strum(to_string = "Tussock Albata")]
    TussockAlbata,
    #[serde(alias = "$Codex_Ent_Tussocks_09_Name;")]
    #[strum(to_string = "Tussock Propagito")]
    TussockPropagito,
    #[serde(alias = "$Codex_Ent_Tussocks_10_Name;")]
    #[strum(to_string = "Tussock Divisa")]
    TussockDivisa,
    #[serde(alias = "$Codex_Ent_Tussocks_11_Name;")]
    #[strum(to_string = "Tussock Caputus")]
    TussockCaputus,
    #[serde(alias = "$Codex_Ent_Tussocks_12_Name;")]
    #[strum(to_string = "Tussock Triticum")]
    TussockTriticum,
    #[serde(alias = "$Codex_Ent_Tussocks_13_Name;")]
    #[strum(to_string = "Tussock Stigmasis")]
    TussockStigmasis,
    #[serde(alias = "$Codex_Ent_Tussocks_14_Name;")]
    #[strum(to_string = "Tussock Virgam")]
    TussockVirgam,
    #[serde(alias = "$Codex_Ent_Tussocks_15_Name;")]
    #[strum(to_string = "Tussock Capillum")]
    TussockCapillum,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Display)]
pub enum GenusType {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,
    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(rename = "$Codex_Ent_Brancae_Name;")]
    #[strum(to_string = "Brain Trees")]
    BrainTrees,
    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,
    #[serde(rename = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,
    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,
    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    #[strum(to_string = "Crystalline Shards")]
    CrystallineShards,
    #[serde(rename = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,
    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,
    #[serde(rename = "$Codex_Ent_Fumerolas_Genus_Name;")]
    Fumerola,
    #[serde(rename = "$Codex_Ent_Sphere_Name;")]
    #[strum(to_string = "Luteolum Anemone")]
    LuteolumAnemone,
    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,
    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,
    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,
    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,
    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,
    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,
    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,
}

#[test]
fn test_codex_categorize_derive() {
    #[derive(CodexCategorize)]
    enum TestCodexNames {
        #[CodexCategory(category=CodexCategory::StellarBodies, sub_category=CodexSubCategory::Stars)]
        BlackHole,
        #[CodexCategory(category=CodexCategory::StellarBodies, sub_category=CodexSubCategory::Stars)]
        NeutronStar,
    }

    assert_eq!(
        TestCodexNames::BlackHole.category(),
        CodexCategory::StellarBodies
    );
    assert_eq!(
        TestCodexNames::BlackHole.sub_category(),
        CodexSubCategory::Stars
    );
    assert_eq!(
        TestCodexNames::NeutronStar.category(),
        CodexCategory::StellarBodies
    );
    assert_eq!(
        TestCodexNames::NeutronStar.sub_category(),
        CodexSubCategory::Stars
    );
}
