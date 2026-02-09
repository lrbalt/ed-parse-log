use crate::{EDString, common_types::CodexBodyInformation};
use ed_parse_log_files_macros::{Extractable, testcase};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(
    Serialize, Deserialize, Clone, Debug, Copy, Display, Hash, Eq, PartialEq, Ord, PartialOrd,
)]
pub enum CodexNames {
    //
    // Stars
    //
    #[serde(rename = "$Codex_Ent_SupermassiveBlack_Holes_Name;")]
    #[strum(to_string = "Supermassive Black Hole")]
    SupermassiveBlackHole,
    #[serde(rename = "$Codex_Ent_Black_Holes_Name;")]
    #[strum(to_string = "Black Hole")]
    BlackHole,
    #[serde(rename = "$Codex_Ent_Neutron_Stars_Name;")]
    #[strum(to_string = "Neutron Star")]
    NeutronStar,
    #[serde(rename = "$Codex_Ent_AeBe_Type_Name;")]
    #[strum(to_string = "AeBe Type Star")]
    AeBeTypeStar,
    #[serde(rename = "$Codex_Ent_A_Type_Name;")]
    #[strum(to_string = "A Type Star")]
    ATypeStar,
    #[serde(rename = "$Codex_Ent_A_TypeGiant_Name;")]
    #[strum(to_string = "A Type Giant")]
    ATypeGiant,
    #[serde(rename = "$Codex_Ent_B_Type_Name;")]
    #[strum(to_string = "B Types")]
    BTypes,
    #[serde(rename = "$Codex_Ent_B_TypeGiant_Name;")]
    #[strum(to_string = "B Type Giant")]
    BTypeGiant,
    #[serde(rename = "$Codex_Ent_D_Type_Name;")]
    #[strum(to_string = "D Type Star")]
    DTypeStar,
    #[serde(rename = "$Codex_Ent_DA_Type_Name;")]
    #[strum(to_string = "DA Type Star")]
    DATypeStar,
    #[serde(rename = "$Codex_Ent_DAB_Type_Name;")]
    #[strum(to_string = "DAB Type Star")]
    DABTypeStar,
    #[serde(rename = "$Codex_Ent_DAZ_Type_Name;")]
    #[strum(to_string = "DAZ Type Star")]
    DAZTypeStar,
    #[serde(rename = "$Codex_Ent_DB_Type_Name;")]
    #[strum(to_string = "DB Type Star")]
    DBTypeStar,
    #[serde(rename = "$Codex_Ent_DBV_Type_Name;")]
    #[strum(to_string = "DBV Type Star")]
    DBVTypeStar,
    #[serde(rename = "$Codex_Ent_DC_Type_Name;")]
    #[strum(to_string = "DC Type Star")]
    DCTypeStar,
    #[serde(rename = "$Codex_Ent_DCV_Type_Name;")]
    #[strum(to_string = "DCV Type Star")]
    DCVTypeStar,
    #[serde(rename = "$Codex_Ent_DQ_Type_Name;")]
    #[strum(to_string = "DQ Type Star")]
    DQTypeStar,
    #[serde(rename = "$Codex_Ent_F_TypeGiant_Name;")]
    #[strum(to_string = "F Type Giant")]
    FTypeGiant,
    #[serde(rename = "$Codex_Ent_F_Type_Name;")]
    #[strum(to_string = "F Type Star")]
    FTypeStar,
    #[serde(rename = "$Codex_Ent_G_Type_Name;")]
    #[strum(to_string = "G Type Star")]
    GTypeStar,
    #[serde(rename = "$Codex_Ent_G_TypeGiant_Name;")]
    #[strum(to_string = "G Type Giant")]
    GTypeGiant,
    #[serde(rename = "$Codex_Ent_K_Type_Name;")]
    #[strum(to_string = "K Type Star")]
    KTypeStar,
    #[serde(rename = "$Codex_Ent_K_TypeGiant_Name;")]
    #[strum(to_string = "K Type Giant")]
    KTypeGiant,
    #[serde(rename = "$Codex_Ent_L_Type_Name;")]
    #[strum(to_string = "L Type Star")]
    LTypeStar,
    #[serde(rename = "$Codex_Ent_M_Type_Name;")]
    #[strum(to_string = "M Type Star")]
    MTypeStar,
    #[serde(rename = "$Codex_Ent_O_Type_Name;")]
    #[strum(to_string = "O Type Stars")]
    OTypeStars,
    #[serde(rename = "$Codex_Ent_S_TypeGiant_Name;")]
    #[strum(to_string = "S Type Giant")]
    STypeGiant,
    #[serde(rename = "$Codex_Ent_TTS_Type_Name;")]
    #[strum(to_string = "T Tauri Star")]
    TTauriStar,
    #[serde(rename = "$Codex_Ent_T_Type_Name;")]
    #[strum(to_string = "T Type Star")]
    TTypeStar,
    #[serde(rename = "$Codex_Ent_Y_Type_Name;")]
    #[strum(to_string = "Y Type Star")]
    YTypeStar,
    //
    // Terraformables
    //
    #[serde(rename = "$Codex_Ent_TRF_Ter_Rocky_Name;")]
    #[strum(to_string = "Terraformable")]
    Terraformable,
    #[serde(rename = "$Codex_Ent_TRF_Rocky_No_Atmos_Name;")]
    #[strum(to_string = "Terraformable")]
    TerraformableRockyNoAtmos,
    #[serde(rename = "$Codex_Ent_TRF_High_Metal_Content_No_Atmos_Name;")]
    #[strum(to_string = "Terraformable")]
    TerraformableHighMetalContentNoAtmos,
    #[serde(rename = "$Codex_Ent_TRF_Ter_High_Metal_Content_Name;")]
    #[strum(to_string = "Terraformable")]
    TerraformableHighMetalContent,
    #[serde(rename = "$Codex_Ent_TRF_Water_Worlds_Name;")]
    #[strum(to_string = "Terraformable")]
    TerraformableWaterWorlds,
    //
    // Non Terraformables
    //
    #[serde(rename = "$Codex_Ent_Earth_Likes_Name;")]
    #[strum(to_string = "Earth-Like Planet")]
    EarthLikePlanet,
    #[serde(rename = "$Codex_Ent_Standard_Ammonia_Worlds_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardAmmoniaWorlds,
    #[serde(rename = "$Codex_Ent_Standard_Giant_With_Ammonia_Life_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardGiantWithAmmoniaLife,
    #[serde(rename = "$Codex_Ent_Standard_Giant_With_Water_Life_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardGiantWithWaterLife,
    #[serde(rename = "$Codex_Ent_Standard_Helium_Rich_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardHeliumRich,
    #[serde(rename = "$Codex_Ent_Standard_Ice_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardIceNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardRockyIce,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerIce,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_Ice_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardRockyIceNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Rocky_Ice_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerRockyIce,
    #[serde(rename = "$Codex_Ent_Standard_Rocky_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardRockyNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Rocky_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerRocky,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_I_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardSudarskyClassI,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_II_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardSudarskyClassII,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_III_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardSudarskyClassIII,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_IV_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardSudarskyClassIV,
    #[serde(rename = "$Codex_Ent_Standard_Sudarsky_Class_V_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardSudarskyClassV,
    #[serde(rename = "$Codex_Ent_Standard_Ter_High_Metal_Content_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerHighMetalContent,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Metal_Rich_Content_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerMetalRichContent,
    #[serde(rename = "$Codex_Ent_Standard_Metal_Rich_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardMetalRichNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_High_Metal_Content_No_Atmos_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardHighMetalContentNoAtmos,
    #[serde(rename = "$Codex_Ent_Standard_Ter_Metal_Rich_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardTerMetalRich,
    #[serde(rename = "$Codex_Ent_Standard_Water_Worlds_Name;")]
    #[strum(to_string = "Non Terraformable")]
    StandardWaterWorlds,
    #[serde(rename = "$Codex_Ent_Standard_Water_Giant_Name;")]
    #[strum(to_string = "Standard gas giant")]
    StandardWaterGiant,
    //
    // Biological and Geological
    //
    #[serde(rename = "$Codex_Ent_Gas_Clds_Red_Name;")]
    #[strum(to_string = "Rubicundum Lagrange Cloud")]
    RubicundumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Pink_Name;")]
    #[strum(to_string = "Roseum Lagrange Cloud")]
    RoseumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Blue_Name;")]
    #[strum(to_string = "Caeruleum Lagrange Cloud")]
    CaeruleumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Light_Name;")]
    #[strum(to_string = "Proto-Lagrange Cloud")]
    ProtoLagrangeCloud,
    #[serde(rename = "$Codex_Ent_Gas_Clds_Yellow_Name;")]
    #[strum(to_string = "Croceum Lagrange Cloud")]
    CroceumLagrangeCloud,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Yw_Name;")]
    #[strum(to_string = "Flavum Metallic Crystals")]
    FlavumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Org_Moll03_V3_Earth_Name;")]
    #[strum(to_string = "Viride Umbrella Mollusc")]
    VirideUmbrellaMollusc,
    #[serde(rename = "$Codex_Ent_L_Org_Moll03_V6_Def_Name;")]
    #[strum(to_string = "Luteolum Reel Mollusc")]
    LuteolumReelMollusc,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Gr_Name;")]
    #[strum(to_string = "Prasinum Metallic Crystals")]
    PrasinumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Pur_Name;")]
    #[strum(to_string = "Purpureum Metallic Crystals")]
    PurpureumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_L_Cry_MetCry_Red_Name;")]
    #[strum(to_string = "Rubeum Metallic Crystals")]
    RubeumMetallicCrystals,
    #[serde(rename = "$Codex_Ent_SPOI_Ball_Name;")]
    #[strum(to_string = "Solid Mineral Spheres")]
    SolidMineralSpheres,

    #[serde(rename = "$Codex_Ent_Aleoids_01_F_Name;")]
    #[strum(to_string = "Aleoida Arcus - Teal")]
    AleoidaArcusTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_01_K_Name;")]
    #[strum(to_string = "Aleoida Arcus - Turquoise")]
    AleoidaArcusTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_03_A_Name;")]
    #[strum(to_string = "Aleoida Spica - Green")]
    AleoidaSpicaGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_04_K_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Turquoise")]
    AleoidaLaminiaeTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_04_L_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Lime")]
    AleoidaLaminiaeLime,
    #[serde(rename = "$Codex_Ent_Aleoids_05_F_Name;")]
    #[strum(to_string = "Aleoida Gravis - Teal")]
    AleoidaGravisTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_02_K_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Turquoise")]
    AleoidaCoronamusTurquoise,
    #[serde(rename = "$Codex_Ent_Aleoids_01_A_Name;")]
    #[strum(to_string = "Aleoida Arcus - Green")]
    AleoidaArcusGreen,
    #[serde(rename = "$Codex_Ent_Aleoids_02_F_Name;")]
    #[strum(to_string = "Aleoida Coronamus - Teal")]
    AleoidaCoronamusTeal,
    #[serde(rename = "$Codex_Ent_Aleoids_04_A_Name;")]
    #[strum(to_string = "Aleoida Laminiae - Green")]
    AleoidaLaminiaeGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Polonium_Name;")]
    #[strum(to_string = "Bacterium Acies - Magenta")]
    BacteriumAciesMagenta,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Molybdenum_Name;")]
    #[strum(to_string = "Bacterium Omentum - Aquamarine")]
    BacteriumOmentumAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_06_G_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Emerald")]
    BacteriumAlcyoneumEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_01_G_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Emerald")]
    BacteriumAurasusEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_01_K_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Green")]
    BacteriumAurasusGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Acies - Cobalt")]
    BacteriumAciesCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Polonium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Orange")]
    BacteriumVesiculaOrange,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Antimony_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Cyan")]
    BacteriumVesiculaCyan,
    #[serde(rename = "$Codex_Ent_Bacterial_06_L_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Sage")]
    BacteriumAlcyoneumSage,
    #[serde(rename = "$Codex_Ent_Bacterial_06_M_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Teal")]
    BacteriumAlcyoneumTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Cadmium_Name;")]
    #[strum(to_string = "Bacterium Tela - Gold")]
    BacteriumTelaGold,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Informem - Gold")]
    BacteriumInformemGold,
    #[serde(rename = "$Codex_Ent_Bacterial_12_G_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Emerald")]
    BacteriumCerbrusEmerald,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Molybdenum_Name;")]
    #[strum(to_string = "Bacterium Tela - Yellow")]
    BacteriumTelaYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_01_A_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Yellow")]
    BacteriumAurasusYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_01_M_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Teal")]
    BacteriumAurasusTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_01_F_Name;")]
    #[strum(to_string = "Bacterium Aurasus - Lime")]
    BacteriumAurasusLime,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Acies - White")]
    BacteriumAciesWhite,
    #[serde(rename = "$Codex_Ent_Bacterial_06_A_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Yellow")]
    BacteriumAlcyoneumYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Antimony_Name;")]
    #[strum(to_string = "Bacterium Acies - Cyan")]
    BacteriumAciesCyan,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Technetium_Name;")]
    #[strum(to_string = "Bacterium Acies - Lime")]
    BacteriumAciesLime,
    #[serde(rename = "$Codex_Ent_Bacterial_04_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Acies - Aquamarine")]
    BacteriumAciesAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Red")]
    BacteriumVesiculaRed,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Technetium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Gold")]
    BacteriumVesiculaGold,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Mulberry")]
    BacteriumVesiculaMulberry,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Vesicula - Lime")]
    BacteriumVesiculaLime,
    #[serde(rename = "$Codex_Ent_Bacterial_06_F_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Lime")]
    BacteriumAlcyoneumLime,
    #[serde(rename = "$Codex_Ent_Bacterial_06_K_Name;")]
    #[strum(to_string = "Bacterium Alcyoneum - Green")]
    BacteriumAlcyoneumGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Tin_Name;")]
    #[strum(to_string = "Bacterium Tela - Cobalt")]
    BacteriumTelaCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Informem - Yellow")]
    BacteriumInformemYellow,
    #[serde(rename = "$Codex_Ent_Bacterial_08_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Informem - Cobalt")]
    BacteriumInformemCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Antimony_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Cobalt")]
    BacteriumBullarisCobalt,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Ruthenium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Aquamarine")]
    BacteriumBullarisAquamarine,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Tellurium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Lime")]
    BacteriumBullarisLime,
    #[serde(rename = "$Codex_Ent_Bacterial_10_Yttrium_Name;")]
    #[strum(to_string = "Bacterium Bullaris - Red")]
    BacteriumBullarisRed,
    #[serde(rename = "$Codex_Ent_Bacterial_11_Tin_Name;")]
    #[strum(to_string = "Bacterium Omentum - Red")]
    BacteriumOmentumRed,
    #[serde(rename = "$Codex_Ent_Bacterial_12_N_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Indigo")]
    BacteriumCerbrusIndigo,
    #[serde(rename = "$Codex_Ent_Bacterial_12_K_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Green")]
    BacteriumCerbrusGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Mercury_Name;")]
    #[strum(to_string = "Bacterium Tela - Orange")]
    BacteriumTelaOrange,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Tungsten_Name;")]
    #[strum(to_string = "Bacterium Tela - Green")]
    BacteriumTelaGreen,
    #[serde(rename = "$Codex_Ent_Bacterial_12_F_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Lime")]
    BacteriumCerbrusLime,
    #[serde(rename = "$Codex_Ent_Bacterial_12_M_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Teal")]
    BacteriumCerbrusTeal,
    #[serde(rename = "$Codex_Ent_Bacterial_12_L_Name;")]
    #[strum(to_string = "Bacterium Cerbrus - Sage")]
    BacteriumCerbrusSage,
    #[serde(rename = "$Codex_Ent_Seed_Name;")]
    #[strum(to_string = "Roseum Brain Tree")]
    RoseumBrainTree,
    #[serde(rename = "$Codex_Ent_SeedABCD_01_Name;")]
    #[strum(to_string = "Gypseeum Brain Tree")]
    GypseeumBrainTree,
    #[serde(rename = "$Codex_Ent_SeedEFGH_Name;")]
    #[strum(to_string = "Lividum Brain Tree")]
    LividumBrainTree,
    #[serde(rename = "$Codex_Ent_Cactoid_01_G_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Teal")]
    CactoidaCortexumTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_02_G_Name;")]
    #[strum(to_string = "Cactoida Lapis - Teal")]
    CactoidaLapisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_02_L_Name;")]
    #[strum(to_string = "Cactoida Lapis - Mauve")]
    CactoidaLapisMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_03_G_Name;")]
    #[strum(to_string = "Cactoida Vermis - Teal")]
    CactoidaVermisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_04_F_Name;")]
    #[strum(to_string = "Cactoida Pullulanta - Yellow")]
    CactoidaPullulantaYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_05_G_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Teal")]
    CactoidaPeperatisTeal,
    #[serde(rename = "$Codex_Ent_Cactoid_05_L_Name;")]
    #[strum(to_string = "Cactoida Peperatis - Mauve")]
    CactoidaPeperatisMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_01_F_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Yellow")]
    CactoidaCortexumYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_01_L_Name;")]
    #[strum(to_string = "Cactoida Cortexum - Mauve")]
    CactoidaCortexumMauve,
    #[serde(rename = "$Codex_Ent_Cactoid_02_A_Name;")]
    #[strum(to_string = "Cactoida Lapis - Green")]
    CactoidaLapisGreen,
    #[serde(rename = "$Codex_Ent_Cactoid_02_F_Name;")]
    #[strum(to_string = "Cactoida Lapis - Yellow")]
    CactoidaLapisYellow,
    #[serde(rename = "$Codex_Ent_Cactoid_03_L_Name;")]
    #[strum(to_string = "Cactoida Vermis - Mauve")]
    CactoidaVermisMauve,
    #[serde(rename = "$Codex_Ent_Conchas_01_Cadmium_Name;")]
    #[strum(to_string = "Concha Renibus - Red")]
    ConchaRenibusRed,
    #[serde(rename = "$Codex_Ent_Conchas_01_Tin_Name;")]
    #[strum(to_string = "Concha Renibus - Aquamarine")]
    ConchaRenibusAquamarine,
    #[serde(rename = "$Codex_Ent_Conchas_01_Tungsten_Name;")]
    #[strum(to_string = "Concha Renibus - White")]
    ConchaRenibusWhite,
    #[serde(rename = "$Codex_Ent_Conchas_02_G_Name;")]
    #[strum(to_string = "Concha Aureolas - Turquoise")]
    ConchaAureolasTurquoise,
    #[serde(rename = "$Codex_Ent_Conchas_02_L_Name;")]
    #[strum(to_string = "Concha Aureolas - Orange")]
    ConchaAureolasOrange,
    #[serde(rename = "$Codex_Ent_Conchas_01_Molybdenum_Name;")]
    #[strum(to_string = "Concha Renibus - Peach")]
    ConchaRenibusPeach,
    #[serde(rename = "$Codex_Ent_Conchas_01_Niobium_Name;")]
    #[strum(to_string = "Concha Renibus - Blue")]
    ConchaRenibusBlue,
    #[serde(rename = "$Codex_Ent_Conchas_03_A_Name;")]
    #[strum(to_string = "Concha Labiata - Teal")]
    ConchaLabiataTeal,
    #[serde(rename = "$Codex_Ent_Conchas_03_G_Name;")]
    #[strum(to_string = "Concha Labiata - Turquoise")]
    ConchaLabiataTurquoise,
    #[serde(rename = "$Codex_Ent_Conchas_03_K_Name;")]
    #[strum(to_string = "Concha Labiata - Red")]
    ConchaLabiataRed,
    #[serde(rename = "$Codex_Ent_Clypeus_02_K_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Grey")]
    ClypeusMargaritusGrey,
    #[serde(rename = "$Codex_Ent_Clypeus_01_G_Name;")]
    #[strum(to_string = "Clypeus Lacrimam - Amethyst")]
    ClypeusLacrimamAmethyst,
    #[serde(rename = "$Codex_Ent_Clypeus_03_G_Name;")]
    #[strum(to_string = "Clypeus Speculumi - Amethyst")]
    ClypeusSpeculumiAmethyst,
    #[serde(rename = "$Codex_Ent_Clypeus_03_F_Name;")]
    #[strum(to_string = "Clypeus Speculumi - Mauve")]
    ClypeusSpeculumiMauve,
    #[serde(rename = "$Codex_Ent_Clypeus_02_L_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Teal")]
    ClypeusMargaritusTeal,
    #[serde(rename = "$Codex_Ent_Clypeus_02_G_Name;")]
    #[strum(to_string = "Clypeus Margaritus - Amethyst")]
    ClypeusMargaritusAmethyst,
    #[serde(rename = "$Codex_Ent_Electricae_02_Tellurium_Name;")]
    #[strum(to_string = "Electricae Radialem - Magenta")]
    ElectricaeRadialemMagenta,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_K_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Emerald")]
    FonticuluaDigitosEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_01_M_Name;")]
    #[strum(to_string = "Fonticulua Segmentatus - Amethyst")]
    FonticuluaSegmentatusAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_K_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Emerald")]
    FonticuluaCampestrisEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_03_G_Name;")]
    #[strum(to_string = "Fonticulua Upupam - Teal")]
    FonticuluaUpupamTeal,
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Cadmium_Name;")]
    #[strum(to_string = "Fumerola Carbosis - Orange")]
    FumerolaCarbosisOrange,
    #[serde(rename = "$Codex_Ent_Fumerolas_02_Tin_Name;")]
    #[strum(to_string = "Fumerola Extremus - Peach")]
    FumerolaExtremusPeach,
    #[serde(rename = "$Codex_Ent_Fumarole_SulphurDioxideMagma_Name;")]
    #[strum(to_string = "Sulphur Dioxide Fumarole")]
    SulphurDioxideFumarole,
    #[serde(rename = "$Codex_Ent_Fungoids_04_Tin_Name;")]
    #[strum(to_string = "Fungoida Gelata - Red")]
    FungoidaGelataRed,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Cadmium_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Blue")]
    FungoidaStabitisBlue,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Technetium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Lime")]
    FungoidaSetisisLime,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Mercury_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Green")]
    FungoidaStabitisGreen,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Tin_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Orange")]
    FungoidaStabitisOrange,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Tungsten_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Peach")]
    FungoidaStabitisPeach,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Ruthenium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Gold")]
    FungoidaSetisisGold,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Tellurium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Yellow")]
    FungoidaSetisisYellow,
    #[serde(rename = "$Codex_Ent_Fungoids_02_Molybdenum_Name;")]
    #[strum(to_string = "Fungoida Stabitis - Magenta")]
    FungoidaStabitisMagenta,
    #[serde(rename = "$Codex_Ent_Fungoids_01_Yttrium_Name;")]
    #[strum(to_string = "Fungoida Setisis - Orange")]
    FungoidaSetisisOrange,
    #[serde(rename = "$Codex_Ent_Gas_Vents_SulphurDioxideMagma_Name;")]
    #[strum(to_string = "Sulphur Dioxide Gas Vent")]
    SulphurDioxideGasVent,
    #[serde(rename = "$Codex_Ent_Ingensradices_Unicus_Name;")]
    #[strum(to_string = "Radicoida Unica")]
    RadicoidaUnica,
    #[serde(rename = "$Codex_Ent_Osseus_02_Tin_Name;")]
    #[strum(to_string = "Osseus Discus - Blue")]
    OsseusDiscusBlue,
    #[serde(rename = "$Codex_Ent_Osseus_01_F_Name;")]
    #[strum(to_string = "Osseus Fractus - Turquoise")]
    OsseusFractusTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_01_G_Name;")]
    #[strum(to_string = "Osseus Fractus - Grey")]
    OsseusFractusGrey,
    #[serde(rename = "$Codex_Ent_Osseus_03_G_Name;")]
    #[strum(to_string = "Osseus Spiralis - Grey")]
    OsseusSpiralisGrey,
    #[serde(rename = "$Codex_Ent_Osseus_03_K_Name;")]
    #[strum(to_string = "Osseus Spiralis - Indigo")]
    OsseusSpiralisIndigo,
    #[serde(rename = "$Codex_Ent_Osseus_03_F_Name;")]
    #[strum(to_string = "Osseus Spiralis - Turquoise")]
    OsseusSpiralisTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_06_F_Name;")]
    #[strum(to_string = "Osseus Pellebantus - Turquoise")]
    OsseusPellebantusTurquoise,
    #[serde(rename = "$Codex_Ent_Osseus_02_Tungsten_Name;")]
    #[strum(to_string = "Osseus Discus - Red")]
    OsseusDiscusRed,
    #[serde(rename = "$Codex_Ent_Osseus_02_Mercury_Name;")]
    #[strum(to_string = "Osseus Discus - Lime")]
    OsseusDiscusLime,
    #[serde(rename = "$Codex_Ent_Osseus_02_Molybdenum_Name;")]
    #[strum(to_string = "Osseus Discus - Peach")]
    OsseusDiscusPeach,
    #[serde(rename = "$Codex_Ent_Osseus_03_A_Name;")]
    #[strum(to_string = "Osseus Spiralis - Lime")]
    OsseusSpiralisLime,
    #[serde(rename = "$Codex_Ent_Osseus_06_G_Name;")]
    #[strum(to_string = "Osseus Pellebantus - Grey")]
    OsseusPellebantusGrey,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_G_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Teal")]
    FonticuluaCampestrisTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_M_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Amethyst")]
    FonticuluaCampestrisAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_02_T_Name;")]
    #[strum(to_string = "Fonticulua Campestris - Orange")]
    FonticuluaCampestrisOrange,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_K_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Emerald")]
    FonticuluaLapidaEmerald,
    #[serde(rename = "$Codex_Ent_Fonticulus_04_M_Name;")]
    #[strum(to_string = "Fonticulua Lapida - Amethyst")]
    FonticuluaLapidaAmethyst,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_G_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Teal")]
    FonticuluaDigitosTeal,
    #[serde(rename = "$Codex_Ent_Fonticulus_06_M_Name;")]
    #[strum(to_string = "Fonticulua Digitos - Amethyst")]
    FonticuluaDigitosAmethyst,
    #[serde(rename = "$Codex_Ent_Shrubs_01_G_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Emerald")]
    FrutexaFlabellumEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_03_M_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Grey")]
    FrutexaMetallicumGrey,
    #[serde(rename = "$Codex_Ent_Shrubs_01_L_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Teal")]
    FrutexaFlabellumTeal,
    #[serde(rename = "$Codex_Ent_Shrubs_02_G_Name;")]
    #[strum(to_string = "Frutexa Acus - Emerald")]
    FrutexaAcusEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_03_F_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Green")]
    FrutexaMetallicumGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_03_L_Name;")]
    #[strum(to_string = "Frutexa Metallicum - Teal")]
    FrutexaMetallicumTeal,
    #[serde(rename = "$Codex_Ent_Shrubs_04_G_Name;")]
    #[strum(to_string = "Frutexa Flammasis - Emerald")]
    FrutexaFlammasisEmerald,
    #[serde(rename = "$Codex_Ent_Shrubs_01_F_Name;")]
    #[strum(to_string = "Frutexa Flabellum - Green")]
    FrutexaFlabellumGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_02_F_Name;")]
    #[strum(to_string = "Frutexa Acus - Green")]
    FrutexaAcusGreen,
    #[serde(rename = "$Codex_Ent_Shrubs_06_G_Name;")]
    #[strum(to_string = "Frutexa Sponsae - Emerald")]
    FrutexaSponsaeEmerald,
    #[serde(rename = "$Codex_Ent_SphereEFGH_01_Name;")]
    #[strum(to_string = "Rubeum Bioluminescent Anemone")]
    RubeumBioluminescentAnemone,
    #[serde(rename = "$Codex_Ent_Fumerolas_01_Tungsten_Name;")]
    #[strum(to_string = "Fumerola Carbosis - Yellow")]
    FumerolaCarbosisYellow,
    #[serde(rename = "$Codex_Ent_Stratum_01_F_Name;")]
    #[strum(to_string = "Stratum Excutitus - Emerald")]
    StratumExcutitusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_01_K_Name;")]
    #[strum(to_string = "Stratum Excutitus - Lime")]
    StratumExcutitusLime,
    #[serde(rename = "$Codex_Ent_Stratum_02_F_Name;")]
    #[strum(to_string = "Stratum Paleas - Emerald")]
    StratumPaleasEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_02_K_Name;")]
    #[strum(to_string = "Stratum Paleas - Lime")]
    StratumPaleasLime,
    #[serde(rename = "$Codex_Ent_Stratum_02_L_Name;")]
    #[strum(to_string = "Stratum Paleas - Turquoise")]
    StratumPaleasTurquoise,
    #[serde(rename = "$Codex_Ent_Stratum_02_M_Name;")]
    #[strum(to_string = "Stratum Paleas - Green")]
    StratumPaleasGreen,
    #[serde(rename = "$Codex_Ent_Stratum_06_F_Name;")]
    #[strum(to_string = "Stratum Cucumisis - Emerald")]
    StratumCucumisisEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_06_K_Name;")]
    #[strum(to_string = "Stratum Cucumisis - Lime")]
    StratumCucumisisLime,
    #[serde(rename = "$Codex_Ent_Stratum_07_F_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Emerald")]
    StratumTectonicasEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_07_K_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Lime")]
    StratumTectonicasLime,
    #[serde(rename = "$Codex_Ent_Stratum_07_L_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Turquoise")]
    StratumTectonicasTurquoise,
    #[serde(rename = "$Codex_Ent_Stratum_03_F_Name;")]
    #[strum(to_string = "Stratum Laminamus - Emerald")]
    StratumLaminamusEmerald,
    #[serde(rename = "$Codex_Ent_Stratum_07_M_Name;")]
    #[strum(to_string = "Stratum Tectonicas - Green")]
    StratumTectonicasGreen,
    #[serde(rename = "$Codex_Ent_Stratum_05_F_Name;")]
    #[strum(to_string = "Stratum Limaxus - Emerald")]
    StratumLimaxusEmerald,
    #[serde(rename = "$Codex_Ent_Tubus_01_A_Name;")]
    #[strum(to_string = "Tubus Conifer - Indigo")]
    TubusConiferIndigo,
    #[serde(rename = "$Codex_Ent_Tubus_02_K_Name;")]
    #[strum(to_string = "Tubus Sororibus - Maroon")]
    TubusSororibusMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_04_L_Name;")]
    #[strum(to_string = "Tubus Rosarium - Turquoise")]
    TubusRosariumTurquoise,
    #[serde(rename = "$Codex_Ent_Tubus_05_F_Name;")]
    #[strum(to_string = "Tubus Compagibus - Grey")]
    TubusCompagibusGrey,
    #[serde(rename = "$Codex_Ent_Tubus_05_G_Name;")]
    #[strum(to_string = "Tubus Compagibus - Red")]
    TubusCompagibusRed,
    #[serde(rename = "$Codex_Ent_Tubus_05_K_Name;")]
    #[strum(to_string = "Tubus Compagibus - Maroon")]
    TubusCompagibusMaroon,
    #[serde(rename = "$Codex_Ent_Tubus_05_A_Name;")]
    #[strum(to_string = "Tubus Compagibus - Indigo")]
    TubusCompagibusIndigo,
    #[serde(rename = "$Codex_Ent_Tussocks_06_K_Name;")]
    #[strum(to_string = "Tussock Pennatis - Green")]
    TussockPennatisGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_14_K_Name;")]
    #[strum(to_string = "Tussock Virgam - Green")]
    TussockVirgamGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_14_L_Name;")]
    #[strum(to_string = "Tussock Virgam - Sage")]
    TussockVirgamSage,
    #[serde(rename = "$Codex_Ent_Tussocks_02_F_Name;")]
    #[strum(to_string = "Tussock Ventusa - Yellow")]
    TussockVentusaYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_03_G_Name;")]
    #[strum(to_string = "Tussock Ignis - Lime")]
    TussockIgnisLime,
    #[serde(rename = "$Codex_Ent_Tussocks_04_G_Name;")]
    #[strum(to_string = "Tussock Cultro - Lime")]
    TussockCultroLime,
    #[serde(rename = "$Codex_Ent_Tussocks_04_K_Name;")]
    #[strum(to_string = "Tussock Cultro - Green")]
    TussockCultroGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_04_L_Name;")]
    #[strum(to_string = "Tussock Cultro - Sage")]
    TussockCultroSage,
    #[serde(rename = "$Codex_Ent_Tussocks_05_G_Name;")]
    #[strum(to_string = "Tussock Catena - Lime")]
    TussockCatenaLime,
    #[serde(rename = "$Codex_Ent_Tussocks_05_L_Name;")]
    #[strum(to_string = "Tussock Catena - Sage")]
    TussockCatenaSage,
    #[serde(rename = "$Codex_Ent_Tussocks_06_L_Name;")]
    #[strum(to_string = "Tussock Pennatis - Sage")]
    TussockPennatisSage,
    #[serde(rename = "$Codex_Ent_Tussocks_08_F_Name;")]
    #[strum(to_string = "Tussock Albata - Yellow")]
    TussockAlbataYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_08_G_Name;")]
    #[strum(to_string = "Tussock Albata - Lime")]
    TussockAlbataLime,
    #[serde(rename = "$Codex_Ent_Tussocks_08_K_Name;")]
    #[strum(to_string = "Tussock Albata - Green")]
    TussockAlbataGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_09_K_Name;")]
    #[strum(to_string = "Tussock Propagito - Green")]
    TussockPropagitoGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_10_K_Name;")]
    #[strum(to_string = "Tussock Divisa - Green")]
    TussockDivisaGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_10_M_Name;")]
    #[strum(to_string = "Tussock Divisa - Emerald")]
    TussockDivisaEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_11_G_Name;")]
    #[strum(to_string = "Tussock Caputus - Lime")]
    TussockCaputusLime,
    #[serde(rename = "$Codex_Ent_Tussocks_12_G_Name;")]
    #[strum(to_string = "Tussock Triticum - Lime")]
    TussockTriticumLime,
    #[serde(rename = "$Codex_Ent_Tussocks_14_G_Name;")]
    #[strum(to_string = "Tussock Virgam - Lime")]
    TussockVirgamLime,
    #[serde(rename = "$Codex_Ent_Tussocks_12_F_Name;")]
    #[strum(to_string = "Tussock Triticum - Yellow")]
    TussockTriticumYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_04_F_Name;")]
    #[strum(to_string = "Tussock Cultro - Yellow")]
    TussockCultroYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_11_F_Name;")]
    #[strum(to_string = "Tussock Caputus - Yellow")]
    TussockCaputusYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_03_M_Name;")]
    #[strum(to_string = "Tussock Ignis - Emerald")]
    TussockIgnisEmerald,
    #[serde(rename = "$Codex_Ent_Tussocks_11_K_Name;")]
    #[strum(to_string = "Tussock Caputus - Green")]
    TussockCaputusGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_03_K_Name;")]
    #[strum(to_string = "Tussock Ignis - Green")]
    TussockIgnisGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_07_K_Name;")]
    #[strum(to_string = "Tussock Serrati - Green")]
    TussockSerratiGreen,
    #[serde(rename = "$Codex_Ent_Tussocks_02_G_Name;")]
    #[strum(to_string = "Tussock Ventusa - Lime")]
    TussockVentusaLime,
    #[serde(rename = "$Codex_Ent_Tussocks_03_F_Name;")]
    #[strum(to_string = "Tussock Ignis - Yellow")]
    TussockIgnisYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_09_F_Name;")]
    #[strum(to_string = "Tussock Propagito - Yellow")]
    TussockPropagitoYellow,
    #[serde(rename = "$Codex_Ent_Tussocks_09_G_Name;")]
    #[strum(to_string = "Tussock Propagito - Lime")]
    TussockPropagitoLime,
    #[serde(rename = "$Codex_Ent_Tussocks_15_M_Name;")]
    #[strum(to_string = "Tussock Capillum - Emerald")]
    TussockCapillumEmerald,
    //
    // Thargoid / Guadian
    //
    #[serde(rename = "$Codex_Ent_Guardian_Beacons_Name;")]
    #[strum(to_string = "Guardian Beacon")]
    GuardianBeacon,
    #[serde(rename = "$Codex_Ent_Guardian_Data_Logs_Name;")]
    #[strum(to_string = "Guardian Codex")]
    GuardianCodex,
    #[serde(rename = "$Codex_Ent_Guardian_Terminal_Name;")]
    #[strum(to_string = "Guardian Data Terminal")]
    GuardianDataTerminal,
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_Spikes_Name;")]
    #[strum(to_string = "Thargoid Barnacle Barbs")]
    ThargoidBarnacleBarbs,
    #[serde(rename = "$Codex_Ent_Thargoid_Barnacle_01_Name;")]
    #[strum(to_string = "Common Thargoid Barnacle")]
    CommonThargoidBarnacle,
    #[serde(rename = "$Codex_Ent_Basilisk_Name;")]
    #[strum(to_string = "Thargoid Interceptor Basilisk")]
    ThargoidInterceptorBasilisk,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, Copy, Display, Hash, Eq, PartialEq, Ord, PartialOrd,
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
    Serialize, Deserialize, Clone, Debug, Copy, Display, Eq, Hash, PartialEq, Ord, PartialOrd,
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
    Serialize, Deserialize, Clone, Debug, Copy, Display, Eq, PartialEq, Hash, Ord, PartialOrd,
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
    #[serde(rename = "$Codex_RegionName_13;")]
    #[strum(to_string = "Newton's Vault")]
    NewtonsVault,
    #[serde(rename = "$Codex_RegionName_14;")]
    #[strum(to_string = "The Conduit")]
    TheConduit,
    #[serde(rename = "$Codex_RegionName_18;")]
    #[strum(to_string = "Inner Orion Spur")]
    InnerOrionSpur,
    #[serde(rename = "$Codex_RegionName_19;")]
    #[strum(to_string = "Hawking's Gap")]
    HawkingsGap,
    #[serde(rename = "$Codex_RegionName_24;")]
    #[strum(to_string = "Formorian Frontier")]
    FormorianFrontier,
    #[serde(rename = "$Codex_RegionName_26;")]
    #[strum(to_string = "Outer Scutum-Centaurus Arm")]
    OuterScutumCentaurusArm,
    #[serde(rename = "$Codex_RegionName_31;")]
    #[strum(to_string = "The Formidine Rift")]
    TheFormidineRift,
    #[serde(rename = "$Codex_RegionName_33;")]
    #[strum(to_string = "Elysian Shore")]
    ElysianShore,
    #[serde(rename = "$Codex_RegionName_34;")]
    #[strum(to_string = "Sanguineous Rim")]
    SanguineousRim,
    #[serde(rename = "$Codex_RegionName_35;")]
    #[strum(to_string = "Outer Orion Spur")]
    OuterOrionSpur,
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

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2022-08-26T19:09:04Z", "event":"CodexEntry", "EntryID":1100502, "Name":"$Codex_Ent_G_TypeGiant_Name;", 
    "Name_Localised":"G Type Giant", "SubCategory":"$Codex_SubCategory_Stars;", "SubCategory_Localised":"Stars", 
    "Category":"$Codex_Category_StellarBodies;", "Category_Localised":"Astronomical Bodies", "Region":"$Codex_RegionName_18;", 
    "Region_Localised":"Inner Orion Spur", "System":"HIP 759", "SystemAddress":10460563811, "IsNewEntry":true })]
#[testcase({ "timestamp":"2026-01-26T15:45:23Z", "event":"CodexEntry", "EntryID":1101001, "Name":"$Codex_Ent_TTS_Type_Name;", 
    "Name_Localised":"T Tauri Star", "SubCategory":"$Codex_SubCategory_Stars;", "SubCategory_Localised":"Stars", 
    "Category":"$Codex_Category_StellarBodies;", "Category_Localised":"Astronomical Bodies", "Region":"$Codex_RegionName_10;", 
    "Region_Localised":"Norma Expanse", "System":"Skauduae QM-T c17-4", "SystemAddress":1185175214746, "BodyID":0, "IsNewEntry":true })]
pub struct EDLogCodexEntry {
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    pub name: CodexNames,
    #[serde(rename = "Name_Localised")]
    pub name_localised: EDString,
    pub sub_category: CodexSubCategory,
    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: EDString,
    pub category: CodexCategory,
    #[serde(rename = "Category_Localised")]
    pub category_localised: EDString,
    pub region: CodexRegion,
    #[serde(rename = "Region_Localised")]
    pub region_localised: EDString,
    pub system: EDString,
    pub system_address: u64,
    #[serde(flatten)]
    pub body_information: Option<CodexBodyInformation>,
    pub is_new_entry: Option<bool>,
    pub voucher_amount: Option<u32>,
}
