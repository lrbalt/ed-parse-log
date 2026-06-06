use crate::{
    EDString,
    common_types::{CarrierDockingAccess, Credits, StationType},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MarketItemType {
    #[serde(alias = "$alexandrite_name;")]
    Alexandrite,
    #[serde(alias = "$aluminium_name;")]
    Aluminium,
    #[serde(alias = "$silver_name;")]
    Silver,
    #[serde(alias = "$tritium_name;")]
    Tritium,
    #[serde(alias = "$platinum_name;")]
    Platinum,
    #[serde(alias = "$titanium_name;")]
    Titanium,
    #[serde(alias = "$thallium_name;")]
    Thallium,
    #[serde(alias = "$tantalum_name;")]
    Tantalum,
    #[serde(alias = "Cobalt", alias = "$cobalt_name;")]
    Cobalt,
    #[serde(alias = "$steel_name;")]
    Steel,
    #[serde(alias = "$lanthanum_name;")]
    Lanthanum,
    #[serde(alias = "$palladium_name;")]
    Palladium,
    #[serde(alias = "$osmium_name;")]
    Osmium,
    #[serde(alias = "Gold", alias = "$gold_name;")]
    Gold,
    #[serde(alias = "Painite", alias = "$painite_name;")]
    Painite,
    #[serde(alias = "$beryllium_name;")]
    Beryllium,
    #[serde(alias = "$bauxite_name;")]
    Bauxite,
    #[serde(alias = "$gallite_name;")]
    Gallite,
    #[serde(alias = "$samarium_name;")]
    Samarium,
    #[serde(alias = "$lepidolite_name;")]
    Lepidolite,
    #[serde(alias = "$rutile_name;")]
    Rutile,
    #[serde(alias = "Uraninite", alias = "$uraninite_name;")]
    Uraninite,
    #[serde(alias = "$moissanite_name;")]
    Moissanite,
    #[serde(alias = "$lowtemperaturediamond_name;")]
    LowTemperatureDiamond,
    #[serde(alias = "$taaffeite_name;")]
    Taaffeite,
    #[serde(alias = "$jadeite_name;")]
    Jadeite,
    #[serde(alias = "$gallium_name;")]
    Gallium,
    #[serde(alias = "$uranium_name;")]
    Uranium,
    #[serde(alias = "$indite_name;")]
    Indite,
    #[serde(alias = "$coltan_name;")]
    Coltan,
    #[serde(alias = "$praseodymium_name;")]
    Praseodymium,
    #[serde(alias = "$copper_name;")]
    Copper,
    #[serde(alias = "$water_name;")]
    Water,
    #[serde(alias = "$indium_name;")]
    Indium,
    #[serde(alias = "$bromellite_name;")]
    Bromellite,
    #[serde(alias = "$bertrandite_name;")]
    Bertrandite,
    #[serde(alias = "$opal_name;")]
    Opal,
    #[serde(alias = "$grandidierite_name;")]
    GrandIdierite,
    #[serde(alias = "$serendibite_name;")]
    Serendibite,
    #[serde(alias = "$haematite_name;")]
    Haematite,
    #[serde(alias = "$monazite_name;")]
    Monazite,
    #[serde(alias = "$thorium_name;")]
    Thorium,
    #[serde(alias = "$lithium_name;")]
    Lithium,
    #[serde(alias = "$benitoite_name;")]
    Benitoite,
    #[serde(alias = "$musgravite_name;")]
    Musgravite,

    #[serde(alias = "SyntheticFabrics", alias = "$syntheticfabrics_name;")]
    SyntheticFabrics,
    #[serde(alias = "$syntheticreagents_name;")]
    SyntheticReagents,
    #[serde(alias = "$syntheticmeat_name;")]
    SyntheticMeat,
    #[serde(alias = "$superconductors_name;")]
    SuperConductors,
    #[serde(alias = "$advancedcatalysers_name;")]
    AdvancedCatalysers,
    #[serde(alias = "$reinforcedmountingplate_name;")]
    ReinforcedMountingPlate,
    #[serde(alias = "$marinesupplies_name;")]
    MarineSupplies,
    #[serde(alias = "Clothing", alias = "$clothing_name;")]
    Clothing,
    #[serde(alias = "ConsumerTechnology", alias = "$consumertechnology_name;")]
    ConsumerTechnology,
    #[serde(alias = "$domesticappliances_name;")]
    DomesticAppliances,
    #[serde(alias = "$magneticemittercoil_name;")]
    MagneticEmitterCoil,
    #[serde(alias = "$powertransferconduits_name;")]
    PowerTransferConduits,
    #[serde(alias = "$explosives_name;")]
    Explosives,
    #[serde(alias = "$powergenerators_name;")]
    PowerGenerators,
    #[serde(alias = "$commercialsamples_name;")]
    ComercialSamples,
    #[serde(alias = "USSCargoTechnicalBlueprints")]
    UssCargoTechnicalBlueprints,
    #[serde(alias = "USSCargoTradeData")]
    UssCargoTradeData,
    #[serde(alias = "USSCargoPrototypeTech")]
    UssCargoPrototypeTech,
    #[serde(alias = "USSCargoBlackBox")]
    UssCargoBlackBox,
    #[serde(alias = "USSCargoMilitaryPlans")]
    UssCargoMilitaryPlans,
    #[serde(alias = "USSCargoAreaArtwork", alias = "$usscargorareartwork_name;")]
    UssCargoAreaArtwork,
    #[serde(alias = "EncriptedDataStorage", alias = "$encripteddatastorage_name;")]
    EncriptedDataStorage,
    #[serde(alias = "$animalmeat_name;")]
    AnimalMeat,
    #[serde(alias = "$animalmonitors_name;")]
    AnimalMonitors,
    #[serde(alias = "$coffee_name;")]
    Coffee,
    #[serde(alias = "Grain", alias = "$grain_name;")]
    Grain,
    #[serde(alias = "$fish_name;")]
    Fish,
    #[serde(alias = "Algae", alias = "$algae_name;")]
    Algae,
    #[serde(alias = "$fruitandvegetables_name;")]
    FruitAndVegetables,
    #[serde(alias = "FoodCartridges", alias = "$foodcartridges_name;")]
    FoodCartridges,
    #[serde(alias = "Liquor", alias = "$liquor_name;")]
    Liquor,
    #[serde(alias = "$beer_name;")]
    Beer,
    #[serde(alias = "Wine", alias = "$wine_name;")]
    Wine,
    BasicNarcotics,
    #[serde(alias = "$geologicalequipment_name;")]
    GeologicalEquipment,
    #[serde(alias = "$biowaste_name;")]
    Biowaste,
    #[serde(alias = "$chemicalwaste_name;")]
    ChemicalWaste,
    #[serde(alias = "$landmines_name;")]
    Landmines,
    #[serde(alias = "$nerveagents_name;")]
    NerveAgents,
    #[serde(alias = "$lithiumhydroxide_name;")]
    LithiumHydroxide,
    #[serde(alias = "Tobacco", alias = "$tobacco_name;")]
    Tobacco,
    #[serde(alias = "$tea_name;")]
    Tea,
    #[serde(alias = "$progenitorcells_name;")]
    ProgenitorCells,
    #[serde(alias = "$hydrogenperoxide_name;")]
    HydrogenPeroxide,
    #[serde(alias = "$personalgifts_name;")]
    PersonalGifts,
    #[serde(alias = "$personalweapons_name;")]
    PersonalWeapons,
    #[serde(alias = "PersonalEffects")]
    PersonalEffects,
    #[serde(alias = "$militarygradefabrics_name;")]
    MilitarygradeFabrics,
    #[serde(alias = "$battleweapons_name;")]
    BattleWeapons,
    #[serde(alias = "$agronomictreatment_name;")]
    AgronomicTreatment,
    #[serde(alias = "$agriculturalmedicines_name;")]
    AgriculturalMedicines,
    #[serde(alias = "$liquidoxygen_name;")]
    LiquidOxygen,
    #[serde(alias = "$methanolmonohydratecrystals_name;")]
    MethanolMonoHydrateCrystals,
    #[serde(alias = "$autofabricators_name;")]
    AutoFabricators,
    #[serde(alias = "$resonatingseparators_name;")]
    ResonatingSeparators,
    #[serde(alias = "$powerconverter_name;")]
    PowerConverter,
    #[serde(alias = "$microcontrollers_name;")]
    MicroControllers,
    #[serde(alias = "$hazardousenvironmentsuits_name;")]
    HazardousEnvironmentSuits,
    #[serde(alias = "$computercomponents_name;")]
    ComputerComponents,
    #[serde(alias = "$modularterminals_name;")]
    ModularTerminals,
    #[serde(alias = "Pesticides", alias = "$pesticides_name;")]
    Pesticides,
    #[serde(alias = "$radiationbaffle_name;")]
    RadiationBaffle,
    #[serde(alias = "$neofabricinsulation_name;")]
    NeofabricInsulation,
    #[serde(alias = "$waterpurifiers_name;")]
    WaterPurifiers,
    #[serde(alias = "$performanceenhancers_name;")]
    PerformanceEnhancers,
    #[serde(alias = "$terrainenrichmentsystems_name;")]
    TerrainenrichmentSystems,
    #[serde(alias = "$heliostaticfurnaces_name;")]
    HelioStaticFurnaces,
    #[serde(alias = "$semiconductors_name;")]
    SemiConductors,
    #[serde(alias = "Polymers", alias = "$polymers_name;")]
    Polymers,
    #[serde(alias = "$insulatingmembrane_name;")]
    InsulatingMembrane,
    #[serde(alias = "$cmmcomposite_name;")]
    CMMComposite,
    #[serde(alias = "$medicaldiagnosticequipment_name;")]
    MedicalDiagnosticEquipment,
    #[serde(alias = "$hydrogenfuel_name;")]
    HydrogenFuel,
    #[serde(alias = "$ceramiccomposites_name;")]
    CeramicComposites,
    #[serde(alias = "$survivalequipment_name;")]
    SurvivalEquipment,
    #[serde(alias = "$buildingfabricators_name;")]
    BuildingFabricators,
    #[serde(alias = "$evacuationshelter_name;")]
    EvacuationShelter,
    #[serde(alias = "$emergencypowercells_name;")]
    EmergencyPowercells,
    #[serde(alias = "$surfacestabilisers_name;")]
    SurfaceStabilisers,
    #[serde(alias = "$structuralregulators_name;")]
    StructuralRegulators,
    #[serde(alias = "$cropharvesters_name;")]
    CropHarvesters,
    #[serde(alias = "$diagnosticsensor_name;")]
    DiagnosticSensor,
    #[serde(alias = "$hostage_name;")]
    Hostage,
    #[serde(alias = "Scrap", alias = "$scrap_name;")]
    Scrap,
    #[serde(alias = "$coolinghoses_name;")]
    CoolingHoses,
    #[serde(alias = "$reactivearmour_name;")]
    ReactiveArmour,
    #[serde(alias = "$conductivefabrics_name;")]
    ConductiveFabrics,
    #[serde(alias = "$advancedmedicines_name;")]
    AdvancedMedicines,
    #[serde(alias = "$combatstabilisers_name;")]
    CombatStabilisers,
    #[serde(alias = "$articulationmotors_name;")]
    ArticulationMotors,
    #[serde(alias = "$thermalcoolingunits_name;")]
    ThermalCoolingUnits,
    #[serde(alias = "$mineralextractors_name;")]
    MineralExtractors,
    #[serde(alias = "$geologicalsamples_name;")]
    GeologicalSamples,
    #[serde(alias = "$scientificsamples_name;")]
    ScientificSamples,
    #[serde(alias = "ComercialSamples", alias = "$comercialsamples_name;")]
    CommercialSamples,
    #[serde(alias = "$bootlegliquor_name;")]
    BootLegLiquor,
    #[serde(alias = "$leather_name;")]
    Leather,
    #[serde(alias = "$robotics_name;")]
    Robotics,
    #[serde(alias = "$naturalfabrics_name;")]
    NaturalFabrics,
    #[serde(alias = "$scientificresearch_name;")]
    ScientificResearch,
    #[serde(alias = "$restrictedintel_name;")]
    RestrictedIntel,
    #[serde(alias = "$militaryintelligence_name;")]
    MilitaryIntelligence,
    #[serde(alias = "$aquaponicsystems_name;")]
    AquaponicSystems,
    #[serde(alias = "$mineraloil_name;")]
    MineralOil,
    #[serde(alias = "$basicmedicines_name;")]
    BasicMedicines,
    #[serde(alias = "$njangarisaddles_name;")]
    NjangariSaddles,
    #[serde(alias = "$volkhabbeedrones_name;")]
    VolkhaBeeDrones,
    #[serde(alias = "$atmosphericextractors_name;")]
    AtmosphericExtractors,
    #[serde(alias = "$nonlethalweapons_name;")]
    NonLethalWeapons,
    #[serde(alias = "$rajukrustoves_name;")]
    Rajukrustoves,
    #[serde(alias = "$livehecateseaworms_name;")]
    LiveHecateSeaworms,
    #[serde(alias = "$cetirabbits_name;")]
    Cetirabbits,
    #[serde(alias = "$aerialedenapple_name;")]
    AerialEdenApple,
    #[serde(alias = "$alacarakmoskinart_name;")]
    AlacarakmoSkinArt,
    #[serde(alias = "$rockforthfertiliser_name;")]
    RockforthFertiliser,
    #[serde(alias = "$harmasilversearum_name;")]
    HarmasilverAurum,

    Drones,
    #[serde(alias = "DamagedEscapePod")]
    DamagedEscapePod,
    #[serde(alias = "$unocuppiedescapepod_name;")]
    UnoccupiedEscapePod,

    #[serde(alias = "WreckageComponents")]
    WreckageComponents,
    AislingMediaMaterials,
    AislingPromotionalMaterials,
    RepublicanFieldSupplies,
    RepublicanGarisonSupplies,
    ImperialSlaves,

    #[serde(alias = "$kamitracigars_name;")]
    KamitraCigars,
    #[serde(alias = "$lavianbrandy_name;")]
    LavianBrandy,
    #[serde(alias = "$konggaale_name;")]
    KonggaAle,
    #[serde(alias = "$soontillrelics_name;")]
    SoontillRelics,
    #[serde(alias = "$vegaslimweed_name;")]
    VegaSlimWeed,
    #[serde(alias = "$eraninpearlwhisky_name;")]
    EraninPearlWhisky,
    #[serde(alias = "$alieneggs_name;")]
    AlienEggs,
    #[serde(alias = "$anynacoffee_name;")]
    AnynaCoffee,
    #[serde(alias = "$kinagoinstruments_name;")]
    KinagoInstruments,
    #[serde(alias = "$onionhead_name;")]
    OnionHead,
    #[serde(alias = "$belalansrayleather_name;")]
    BelalansRayLeather,
    #[serde(alias = "$eshuumbrellas_name;")]
    EshuUmbrellas,
    #[serde(alias = "$hip10175bushmeat_name;")]
    Hip10175BushMeat,
    #[serde(alias = "$eleuthermals_name;")]
    Eleuthermals,
    #[serde(alias = "$bioreducinglichen_name;")]
    BioreducingLichen,
    #[serde(alias = "$baltahsinevacuumkrill_name;")]
    BaltahsineVacuumKrill,
    #[serde(alias = "$tiolcewaste2pasteunits_name;")]
    TiolceWaste2PasteUnits,
    #[serde(alias = "$ngunamodernantiques_name;")]
    NgunaModernAntiques,
    #[serde(alias = "$giantirukamasnails_name;")]
    GiantIrukamaSnails,
    #[serde(alias = "$utgaroarmillenialeggs_name;")]
    UtgaroArmilleniaLeggs,
    #[serde(alias = "$neritusberries_name;")]
    NeritusBerries,
    #[serde(alias = "$ochoengchillies_name;")]
    OchoengChillies,
    #[serde(alias = "$wuthielokufroth_name;")]
    WuthielokuFroth,
    #[serde(alias = "$cd75catcoffee_name;")]
    CD75CatCoffee,
    #[serde(alias = "$hip41181squid_name;")]
    Hip41181Squid,
    #[serde(alias = "$hr7221wheat_name;")]
    HR7221Wheat,
    #[serde(alias = "$jarouarice_name;")]
    JarouaRice,
    #[serde(alias = "$karetiicouture_name;")]
    KaretiiCouture,
    #[serde(alias = "$albinoquechuamammoth_name;")]
    AlbinoQuechuamammoth,
    #[serde(alias = "$chieridanimarinepaste_name;")]
    ChieridaniMarinePaste,
    #[serde(alias = "$deuringastruffles_name;")]
    DeuringaStruffles,
    #[serde(alias = "$gomanyauponcoffee_name;")]
    GomanyaUponCoffee,
    #[serde(alias = "$helvetitjpearls_name;")]
    HelvetitjPearls,
    #[serde(alias = "$coquimspongiformvictuals_name;")]
    CoquimSpongiformVictuals,
    #[serde(alias = "$karsukilocusts_name;")]
    KarsukiLocusts,
    #[serde(alias = "$kachiriginleaches_name;")]
    KachiriginLeaches,
    #[serde(alias = "$esusekucaviar_name;")]
    EsusekuCaviar,
    #[serde(alias = "$witchhaulkobebeef_name;")]
    WitchHaulKobeBeef,
    #[serde(alias = "$damnacarapaces_name;")]
    DamnaCarapaces,
    #[serde(alias = "$yasokondileaf_name;")]
    YasokondiLeaf,
    #[serde(alias = "$bakedgreebles_name;")]
    BakedGreebles,
    #[serde(alias = "$sanumameat_name;")]
    SanumaMeat,
    #[serde(alias = "$giantverrix_name;")]
    GiantVerrix,
    #[serde(alias = "$aroucaconventualsweets_name;")]
    AroucaConventualSweets,
    #[serde(alias = "$pantaaprayersticks_name;")]
    PantaapPrayerSticks,
    #[serde(alias = "$uzumokulowgwings_name;")]
    UzumokuLowgWings,
    #[serde(alias = "$medbstarlube_name;")]
    MedbStarLube,
    #[serde(alias = "$bastsnakegin_name;")]
    BastSnakeGin,
    #[serde(alias = "$wheemetewheatcakes_name;")]
    WheemeteWheatCakes,
    #[serde(alias = "$ngadandarifireopals_name;")]
    NgadandariFireOpals,
    #[serde(alias = "$rusanioldsmokey_name;")]
    RusaniOldsMokey,
    #[serde(alias = "$wulpahyperboresystems_name;")]
    Wulpahyperboresystems,
    #[serde(alias = "$jaradharrepuzzlebox_name;")]
    JaradharRepuzzleBox,
    #[serde(alias = "$xihecompanions_name;")]
    XiheCompanions,
    #[serde(alias = "$azcancriformula42_name;")]
    AZCancriFormula42,
    #[serde(alias = "$geawendancedust_name;")]
    GeawendanceDust,
    #[serde(alias = "$ltthypersweet_name;")]
    LTTHyperSweet,
    #[serde(alias = "$jotunmookah_name;")]
    JotunMookah,
    #[serde(alias = "$ethgrezeteabuds_name;")]
    EthgrezeTeaBuds,
    #[serde(alias = "$ophiuchiexinoartefacts_name;")]
    OphiuchiexinoArtefacts,
    #[serde(alias = "$saxonwine_name;")]
    SaxonWine,
    #[serde(alias = "$cetiaepyornisegg_name;")]
    CetiaepyornisEgg,
    #[serde(alias = "$fujintea_name;")]
    FujinTea,
    #[serde(alias = "$mulachigiantfungus_name;")]
    MulachiGiantFungus,
    #[serde(alias = "$zeesszeantglue_name;")]
    ZeesszeAntGlue,
    #[serde(alias = "$tanmarktranquiltea_name;")]
    TanmarkTranquilTea,
    #[serde(alias = "$mukusubiichitinos_name;")]
    Mukusubiichitinos,
    #[serde(alias = "$burnhambiledistillate_name;")]
    BurnhamBileDistillate,
    #[serde(alias = "$hiporganophosphates_name;")]
    HipOrganophosphates,
    #[serde(alias = "$tiegfriessynthsilk_name;")]
    TiegfriesSynthSilk,
    #[serde(alias = "$altairianskin_name;")]
    AltairianSkin,
    #[serde(alias = "$rapabaosnakeskins_name;")]
    RapabaosSnakeSkins,
    #[serde(alias = "$deltaphoenicispalms_name;")]
    DeltaPhoenicisPalms,
    #[serde(alias = "$watersofshintara_name;")]
    WatersOfShintara,
    #[serde(alias = "$mechucoshightea_name;")]
    MechucosHighTea,
    #[serde(alias = "$centaurimegagin_name;")]
    CentauriMegagin,
    #[serde(alias = "$mokojingbeastfeast_name;")]
    MokojingBeastFeast,
    #[serde(alias = "$haidneblackbrew_name;")]
    HaidneBlackBrew,
    #[serde(alias = "$lftvoidextractcoffee_name;")]
    LFTVoidExtractCoffee,
    #[serde(alias = "$honestypills_name;")]
    HonestyPills,
    #[serde(alias = "$gerasiangueuzebeer_name;")]
    GerasiangueuzeBeer,
    #[serde(alias = "$toxandjivirocide_name;")]
    ToxandJivirocide,
    #[serde(alias = "$korrokungpellets_name;")]
    KorrokungPellets,
    #[serde(alias = "$cherbonesbloodcrystals_name;")]
    CherbonesBloodCrystals,
    #[serde(alias = "$thrutiscream_name;")]
    ThrutisCream,
    #[serde(alias = "$indibourbon_name;")]
    IndiBourbon,
    #[serde(alias = "$taurichimes_name;")]
    TauriChimes,
    #[serde(alias = "$ceremonialheiketea_name;")]
    CeremonialHeikeTea,
    #[serde(alias = "$vanayequirhinofur_name;")]
    VanayeQurikhinoFur,
    #[serde(alias = "$vherculisbodyrub_name;")]
    VherculisBodyRub,
    #[serde(alias = "$havasupaidreamcatcher_name;")]
    HavasupaiDreamCatcher,
    #[serde(alias = "$chateaudeaegaeon_name;")]
    ChateaudeAegaeon,
    #[serde(alias = "$anduligafireworks_name;")]
    AnduligaFireworks,
    #[serde(alias = "$noneuclidianexotanks_name;")]
    NoneuclidianExotanks,
    #[serde(alias = "$bankiamphibiousleather_name;")]
    BankiaMpibiousLeather,
    #[serde(alias = "$momusbogspaniel_name;")]
    MomusBogSpaniel,
    #[serde(alias = "$pyrophyllite_name;")]
    Pyrophyllite,
    #[serde(alias = "$transgeniconionhead_name;")]
    TransgenicOnionHead,
    #[serde(alias = "$sothiscrystallinegold_name;")]
    SothisCrystallineGold,
    #[serde(alias = "$antiquejewellery_name;")]
    AntiqueJewellery,
    #[serde(alias = "$tacticaldata_name;")]
    TacticalData,
    #[serde(alias = "$smallexplorationdatacash_name;")]
    SmallExplorationDataCash,
    #[serde(alias = "$largeexplorationdatacash_name;")]
    LargeExplorationDataCash,
    #[serde(alias = "$advert1_name;")]
    UltraCompactProcessorPrototypes,
    #[serde(alias = "$assaultplans_name;")]
    AssaultPlans,
    #[serde(alias = "$jaquesquinentianstill_name;")]
    JaquesQuinentianStill,
    #[serde(alias = "$leestianeviljuice_name;")]
    LeestianEvilJuice,
    #[serde(alias = "$diplomaticbag_name;")]
    DiplomaticBag,
    #[serde(alias = "$vidavantianlace_name;")]
    VidavantianLace,
    #[serde(alias = "$chameleoncloth_name;")]
    ChameleonCloth,
    #[serde(alias = "$timecapsule_name;")]
    TimeCapsule,
    #[serde(alias = "$disomacorn_name;")]
    DisomaCorn,
    #[serde(alias = "$genebank_name;")]
    GeneBank,
    #[serde(alias = "$mutomimager_name;")]
    MutomiMager,
    #[serde(alias = "$bismuth_name;")]
    Bismuth,
    #[serde(alias = "$crystallinespheres_name;")]
    CrystallineSpheres,
    #[serde(alias = "$alyabodilysoap_name;")]
    AlyaBodilySoap,
    #[serde(alias = "$earthrelics_name;")]
    EarthRelics,
    #[serde(alias = "$uszaiantreegrub_name;")]
    UszaianTreegrub,
    #[serde(alias = "$thehuttonmug_name;")]
    TheHuttonMug,
    #[serde(alias = "$cryolite_name;")]
    Cryolite,
    #[serde(alias = "$orrerianviciousbrew_name;")]
    OrrerianViciousBrew,
    #[serde(alias = "$skimercomponents_name;")]
    SkimerComponents,
    #[serde(alias = "$goslarite_name;")]
    Goslarite,
    #[serde(alias = "$bluemilk_name;")]
    BlueMilk,
    #[serde(alias = "$preciousgems_name;")]
    PreciousGems,
    #[serde(alias = "$encryptedcorrespondence_name;")]
    EncryptedCorrespondence,
    #[serde(alias = "$sap8corecontainer_name;")]
    Sap8CoreContainer,
    #[serde(alias = "$shanscharisorchid_name;")]
    ShanschariOrchid,
    #[serde(alias = "$methaneclathrate_name;")]
    MethaneClathrate,
    #[serde(alias = "$hnshockmount_name;")]
    HnshockMount,
    #[serde(alias = "$mysteriousidol_name;")]
    MysteriousIdol,
    #[serde(alias = "$fossilremnants_name;")]
    FossilRemnants,
    #[serde(alias = "$exhaustmanifold_name;")]
    ExhaustManifold,
    #[serde(alias = "$onionheada_name;")]
    OnionheadAlphaStrain,
    #[serde(alias = "$onionheadb_name;")]
    OnionheadBetaStrain,
    #[serde(alias = "$onionheadc_name;")]
    OnionheadGammaStrain,
    #[serde(alias = "$unstabledatacore_name;")]
    UnstableDataCore,
    #[serde(alias = "$spacepioneerrelics_name;")]
    SpacePioneerRelics,
    #[serde(alias = "$datacore_name;")]
    DataCore,
    #[serde(alias = "$iondistributor_name;")]
    IonDistributor,
    #[serde(alias = "$heatsinkinterlink_name;")]
    HeatSinkInterlink,
    #[serde(alias = "$telemetrysuite_name;")]
    TelemetrySuite,
    #[serde(alias = "$nanobreakers_name;")]
    Nanobreakers,
    #[serde(alias = "$nanomedicines_name;")]
    Nanomedicines,
    #[serde(alias = "$galactictravelguide_name;")]
    GalacticTravelGuide,
    #[serde(alias = "$powergridassembly_name;")]
    PowerGridAssembly,
    #[serde(alias = "$buckyballbeermats_name;")]
    BuckyBallBeerMats,
    #[serde(alias = "$coralsap_name;")]
    CoralSap,
    #[serde(alias = "$classifiedexperimentalequipment_name;")]
    ClassifiedExperimentalEquipment,
    #[serde(alias = "$apavietii_name;")]
    ApaVietii,
    #[serde(alias = "$platinumaloy_name;")]
    PlatinumAloy,
    #[serde(alias = "$rhodplumsite_name;")]
    Rhodplumsite,

    #[serde(alias = "OccupiedCryoPod")]
    OccupiedCryoPod,

    #[serde(alias = "$ancientcasket_name;")]
    AncientCasket,
    #[serde(alias = "AncientRelic", alias = "$ancientrelic_name;")]
    AncientRelic,
    #[serde(alias = "$ancientrelictg_name;")]
    UnclassifiedRelic,
    #[serde(alias = "$ancienttablet_name;")]
    AncientTablet,
    #[serde(alias = "$ancienttotem_name;")]
    AncientTotem,
    #[serde(alias = "$ancienturn_name;")]
    AncientUrn,
    #[serde(alias = "$ancientorb_name;")]
    AncientOrb,
    #[serde(alias = "$ancientkey_name;")]
    AncientKey,

    #[serde(
        alias = "ThargoidTitanDriveComponent",
        alias = "$thargoidtitandrivecomponent_name;"
    )]
    ThargoidTitanDriveComponent,
    #[serde(
        alias = "ThargoidBoneFragments",
        alias = "$thargoidbonefragments_name;"
    )]
    ThargoidBoneFragments,
    #[serde(
        alias = "ThargoidGeneratorTissueSample",
        alias = "$thargoidgeneratortissuesample_name;"
    )]
    ThargoidGeneratorTissueSample,
    #[serde(alias = "ThargoidCystSpecimen", alias = "$thargoidcystspecimen_name;")]
    ThargoidCystSpecimen,
    #[serde(alias = "ThargoidPod", alias = "$thargoidpod_name;")]
    ThargoidPod,
    #[serde(
        alias = "unknownartifact",
        alias = "$unknownartifact_name;",
        alias = "UnknownArtifact"
    )]
    ThargoidSensor,
    #[serde(alias = "ThargoidOrganSample", alias = "$thargoidorgansample_name;")]
    ThargoidOrganSample,
    #[serde(alias = "$thargoidscouttissuesample_name;")]
    ThargoidScoutTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType1",
        alias = "thargoidtissuesampletype1",
        alias = "$thargoidtissuesampletype1_name;"
    )]
    ThargoidCyclopsTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType2",
        alias = "thargoidtissuesampletype2",
        alias = "$thargoidtissuesampletype2_name;"
    )]
    ThargoidBasiliskTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType3",
        alias = "thargoidtissuesampletype3",
        alias = "$thargoidtissuesampletype3_name;"
    )]
    ThargoidMedusaTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType4",
        alias = "thargoidtissuesampletype4",
        alias = "$thargoidtissuesampletype4_name;"
    )]
    ThargoidHydraTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType5",
        alias = "thargoidtissuesampletype5",
        alias = "$thargoidtissuesampletype5_name;"
    )]
    ThargoidOrthrusTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType6",
        alias = "thargoidtissuesampletype6",
        alias = "$thargoidtissuesampletype6_name;"
    )]
    ThargoidGlaiveTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType7",
        alias = "thargoidtissuesampletype7",
        alias = "$thargoidtissuesampletype7_name;"
    )]
    ThargoidScytheTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType9a",
        alias = "thargoidtissuesampletype9a",
        alias = "$thargoidtissuesampletype9a_name;"
    )]
    TitanDeepTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType9b",
        alias = "thargoidtissuesampletype9b",
        alias = "$thargoidtissuesampletype9b_name;"
    )]
    TitanTissueSample,
    #[serde(
        alias = "ThargoidTissueSampleType9c",
        alias = "thargoidtissuesampletype9c",
        alias = "$thargoidtissuesampletype9c_name;"
    )]
    TitanPartialTissueSample,
    #[serde(alias = "$thargoidtissuesampletype10a_name;")]
    TitanMawDeepTissueSample,
    #[serde(alias = "$thargoidtissuesampletype10b_name;")]
    TitanMawTissueSample,
    #[serde(alias = "$thargoidtissuesampletype10c_name;")]
    TitanMawPartialTissueSample,
    #[serde(alias = "ThargoidHeart", alias = "$thargoidheart_name;")]
    ThargoidHeart,
    #[serde(
        alias = "Thargoid Resin",
        alias = "$unknownresin_name;",
        alias = "UnknownResin",
        alias = "unknownresin"
    )]
    ThargoidResin,
    #[serde(alias = "$unknownbiologicalmatter_name;")]
    ThargoidBiologicalMatter,
    #[serde(alias = "$unknowntechnologysamples_name;")]
    ThargoidTechnologySamples,
    #[serde(alias = "$unknownartifact2_name;")]
    ThargoidProbe,
    #[serde(alias = "$unknownartifact3_name;")]
    ThargoidLink,
    #[serde(alias = "MetaAlloys", alias = "$metaalloys_name;")]
    MetaAlloys,
    #[serde(
        alias = "$unknownsack_name;",
        alias = "unknownsack",
        alias = "UnknownSack"
    )]
    ProtectiveMembraneScrap,
    #[serde(alias = "$unknownmineral_name;")]
    ImpureSpireMineral,
    #[serde(alias = "$unknownrefinedmineral_name;")]
    SemiRefinedSpireMineral,
    #[serde(alias = "$m_tissuesample_fluid_name;")]
    MolluscFluid,
    #[serde(alias = "$m_tissuesample_soft_name;")]
    MolluscSoftTissue,
    #[serde(alias = "$m_tissuesample_nerves_name;")]
    MolluscBrainTissue,
    #[serde(alias = "$s_tissuesample_cells_name;")]
    PodCoreTissue,
    #[serde(alias = "$s_tissuesample_surface_name;")]
    PodDeadTissue,
    #[serde(alias = "$s_tissuesample_core_name;")]
    PodSurfaceTissue,
    #[serde(alias = "$p_particulatesample_name;")]
    AnomalyParticles,
    #[serde(alias = "$duradrives_name;")]
    Duradrives,
    #[serde(alias = "$s9_tissuesample_shell_name;")]
    PodTissue,
    #[serde(alias = "$m3_tissuesample_membrane_name;")]
    MolluscMembrane,
    #[serde(alias = "$m3_tissuesample_mycelium_name;")]
    MolluscMycelium,
    #[serde(alias = "$m3_tissuesample_spores_name;")]
    MolluscSpores,
    #[serde(alias = "$s6_tissuesample_mesoglea_name;")]
    PodMesoglea,
    #[serde(alias = "$s6_tissuesample_cells_name;")]
    PodOuterTissue,
    #[serde(alias = "$s6_tissuesample_coenosarc_name;")]
    PodShellTissue,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarketBuy {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    #[serde(rename = "Type")]
    pub buy_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub buy_type_localised: Option<EDString>,
    pub count: u64,
    pub buy_price: Credits,
    pub total_cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2017-10-17T03:29:58Z", "event":"MarketSell", "Type":"biowaste", "Count":1, "SellPrice":10, "TotalSale":10, "AvgPricePaid":0 })]
pub struct EDLogMarketSell {
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
    #[serde(rename = "Type")]
    pub sell_type: MarketItemType,
    #[serde(rename = "Type_Localised")]
    pub sell_type_localised: Option<EDString>,
    pub count: u64,
    pub sell_price: Credits,
    pub total_sale: Credits,
    pub avg_price_paid: Credits,
    pub illegal_goods: Option<bool>,
    pub stolen_goods: Option<bool>,
    pub black_market: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MicroResourceType {
    Data,
    Item,
    Component,
    Consumable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct MicroResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    category: MicroResourceType,
    count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogBuyMicroResources {
    pub total_count: Option<u64>,
    #[serde(flatten)]
    pub micro_resource: Option<MicroResource>,
    pub micro_resources: Option<Vec<MicroResource>>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2025-06-28T19:16:15Z", "event":"BuyTradeData", "System":"Quator", "Cost":100 })]
pub struct EDLogBuyTradeData {
    system: EDString,
    cost: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogTradeMicroResources {
    offered: Vec<MicroResource>,
    total_count: u64,
    received: EDString,
    #[serde(rename = "Received_Localised")]
    received_localised: Option<EDString>,
    count: u64,
    category: MicroResourceType,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase({ "timestamp":"2026-01-21T20:24:20Z", "event":"SellMicroResources", "TotalCount":44, 
    "MicroResources":[ 
        { "Name":"compactlibrary", "Name_Localised":"Compact Library", "Category":"Item", "Count":1 }, 
        { "Name":"insight", "Category":"Item", "Count":1 } ], 
    "Price":479000, "MarketID":3228823296 })]
pub struct EDLogSellMicroResources {
    pub total_count: u64,
    pub micro_resources: Vec<MicroResource>,
    pub price: Credits,
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogDeliverPowerMicroResources {
    total_count: u64,
    micro_resources: Vec<MicroResource>,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum MarketItemCategory {
    #[serde(alias = "$MARKET_category_chemicals;")]
    Chemicals,
    #[serde(alias = "$MARKET_category_consumer_items;")]
    ConsumerItems,
    #[serde(alias = "$MARKET_category_drugs;")]
    Drugs,
    #[serde(alias = "$MARKET_category_foods;")]
    Foods,
    #[serde(alias = "$MARKET_category_industrial_materials;")]
    IndustrialMaterials,
    #[serde(alias = "$MARKET_category_machinery;")]
    Machinery,
    #[serde(alias = "$MARKET_category_metals;")]
    Metals,
    #[serde(alias = "$MARKET_category_medicines;")]
    Medicines,
    #[serde(alias = "$MARKET_category_minerals;")]
    Minerals,
    #[serde(alias = "$MARKET_category_salvage;")]
    Salvage,
    #[serde(alias = "$MARKET_category_technology;")]
    Technology,
    #[serde(alias = "$MARKET_category_textiles;")]
    Textiles,
    #[serde(alias = "$MARKET_category_waste;")]
    Waste,
    #[serde(alias = "$MARKET_category_weapons;")]
    Weapons,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
#[testcase_struct({ "id":128793127, "Name":"$thargoidheart_name;", "Name_Localised":"Thargoid Heart", "Category":"$MARKET_category_salvage;", "Category_Localised":"Salvage", "BuyPrice":106696, "SellPrice":105639, "MeanPrice":140275, "StockBracket":0, "DemandBracket":0, "Stock":0, "Demand":0, "Consumer":false, "Producer":false, "Rare":false })]
pub struct MarketItem {
    #[serde(rename = "id")]
    id: u64,
    #[serde(rename = "Name")]
    pub market_item_name: MarketItemType,
    #[serde(rename = "Name_Localised")]
    pub market_item_name_localised: Option<EDString>,
    pub category: MarketItemCategory,
    #[serde(rename = "Category_Localised")]
    pub category_localised: Option<EDString>,
    buy_price: Credits,
    sell_price: Credits,
    mean_price: Credits,
    stock_bracket: u64,
    demand_bracket: u64,
    stock: u64,
    demand: u64,
    consumer: bool,
    producer: bool,
    rare: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogMarket {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: EDString,
    pub station_type: StationType,
    pub carrier_docking_access: Option<CarrierDockingAccess>,
    pub star_system: EDString,
    pub items: Option<Vec<MarketItem>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogCargoDepot {
    #[serde(rename = "MissionID")]
    mission_id: u64,
    update_type: EDString,        // TODO: enum
    cargo_type: Option<EDString>, // TODO: enum
    #[serde(rename = "CargoType_Localised")]
    cargo_type_localised: Option<EDString>,
    count: Option<u64>,
    #[serde(rename = "StartMarketID")]
    start_market_id: u64,
    #[serde(rename = "EndMarketID")]
    end_market_id: u64,
    items_collected: u64,
    items_delivered: u64,
    total_items_to_deliver: u64,
    progress: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct RequiredResource {
    pub name: EDString,
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<EDString>,
    pub required_amount: u64,
    pub provided_amount: u64,
    pub payment: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct ContributedResource {
    name: EDString,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<EDString>,
    amount: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationConstructionDepot {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub construction_progress: f64,
    pub construction_complete: bool,
    pub construction_failed: bool,
    pub resources_required: Vec<RequiredResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogColonisationContribution {
    #[serde(rename = "MarketID")]
    market_id: u64,
    contributions: Vec<ContributedResource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct SoldBioData {
    pub genus: EDString,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: EDString,
    pub species: EDString,
    #[serde(rename = "Species_Localised")]
    pub species_localised: EDString,
    pub variant: Option<EDString>,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<EDString>,
    pub value: Credits,
    pub bonus: Credits,
}

#[derive(Serialize, Deserialize, Clone, Debug, Extractable)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct EDLogSellOrganicData {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub bio_data: Vec<SoldBioData>,
}
