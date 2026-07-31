use crate::{
    EDString,
    common_types::{CarrierDockingAccess, Credits, StationType},
};
use ed_parse_log_files_macros::{Extractable, testcase, testcase_struct};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
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

    #[serde(
        alias = "$lowtemperaturediamond_name;",
        alias = "lowtemperaturediamond"
    )]
    #[strum(to_string = "Low Temp. Diamonds")]
    LowTempDiamonds,
    #[serde(alias = "$opal_name;", alias = "opal")]
    #[strum(to_string = "Void Opal")]
    VoidOpal,
    #[serde(alias = "$basicnarcotics_name;", alias = "basicnarcotics")]
    #[strum(to_string = "Narcotics")]
    Narcotics,
    #[serde(alias = "$personalweapons_name;")]
    #[strum(to_string = "Personal Weapons")]
    PersonalWeapons,
    #[serde(alias = "$battleweapons_name;")]
    #[strum(to_string = "Battle Weapons")]
    BattleWeapons,
    #[serde(alias = "$combatstabilisers_name;")]
    #[strum(to_string = "Combat Stabilisers")]
    CombatStabilisers,
    #[serde(alias = "$borasetanipathogenetics_name;")]
    #[strum(to_string = "Borasetani Pathogenetics")]
    BorasetaniPathogenetics,
    #[serde(alias = "$hip118311swarm_name;")]
    #[strum(to_string = "HIP 118311 Swarm")]
    HIP118311Swarm,

    #[serde(alias = "$semiconductors_name;")]
    #[strum(to_string = "Semiconductors")]
    Semiconductors,
    #[serde(alias = "$superconductors_name;")]
    #[strum(to_string = "Superconductors")]
    Superconductors,
    #[serde(alias = "$hydrogenfuel_name;")]
    #[strum(to_string = "Hydrogen Fuel")]
    HydrogenFuel,
    #[serde(alias = "$mineraloil_name;")]
    #[strum(to_string = "Mineral Oil")]
    MineralOil,
    #[serde(alias = "$agriculturalmedicines_name;")]
    #[strum(to_string = "Agri-Medicines")]
    AgriMedicines,
    #[serde(alias = "$performanceenhancers_name;")]
    #[strum(to_string = "Performance Enhancers")]
    PerformanceEnhancers,
    #[serde(alias = "$basicmedicines_name;")]
    #[strum(to_string = "Basic Medicines")]
    BasicMedicines,
    #[serde(alias = "$powergenerators_name;")]
    #[strum(to_string = "Power Generators")]
    PowerGenerators,
    #[serde(alias = "$waterpurifiers_name;")]
    #[strum(to_string = "Water Purifiers")]
    WaterPurifiers,
    #[serde(alias = "$heliostaticfurnaces_name;", alias = "heliostaticfurnaces")]
    #[strum(to_string = "Microbial Furnaces")]
    MicrobialFurnaces,
    #[serde(alias = "$mineralextractors_name;")]
    #[strum(to_string = "Mineral Extractors")]
    MineralExtractors,
    #[serde(alias = "$cropharvesters_name;")]
    #[strum(to_string = "Crop Harvesters")]
    CropHarvesters,
    #[serde(alias = "$marinesupplies_name;", alias = "marinesupplies")]
    #[strum(to_string = "Marine Equipment")]
    MarineEquipment,
    #[serde(alias = "$computercomponents_name;")]
    #[strum(to_string = "Computer Components")]
    ComputerComponents,
    #[serde(alias = "$hazardousenvironmentsuits_name;")]
    #[strum(to_string = "H.E. Suits")]
    HazardousEnvironmentSuits,
    #[serde(alias = "$autofabricators_name;")]
    #[strum(to_string = "Auto-Fabricators")]
    AutoFabricators,
    #[serde(alias = "$animalmonitors_name;")]
    #[strum(to_string = "Animal Monitors")]
    AnimalMonitors,
    #[serde(alias = "$aquaponicsystems_name;")]
    #[strum(to_string = "Aquaponic Systems")]
    AquaponicSystems,
    #[serde(alias = "$advancedcatalysers_name;")]
    #[strum(to_string = "Advanced Catalysers")]
    AdvancedCatalysers,
    #[serde(
        alias = "$terrainenrichmentsystems_name;",
        alias = "terrainenrichmentsystems"
    )]
    #[strum(to_string = "Land Enrichment Systems")]
    LandEnrichmentSystems,
    #[serde(alias = "$reactivearmour_name;")]
    #[strum(to_string = "Reactive Armour")]
    ReactiveArmour,
    #[serde(alias = "$nonlethalweapons_name;")]
    #[strum(to_string = "Non-Lethal Weapons")]
    NonLethalWeapons,
    #[serde(alias = "$domesticappliances_name;")]
    #[strum(to_string = "Domestic Appliances")]
    DomesticAppliances,
    #[serde(alias = "$consumertechnology_name;", alias = "ConsumerTechnology")]
    #[strum(to_string = "Consumer Technology")]
    ConsumerTechnology,
    #[serde(alias = "$chemicalwaste_name;")]
    #[strum(to_string = "Chemical Waste")]
    ChemicalWaste,
    #[serde(alias = "$progenitorcells_name;")]
    #[strum(to_string = "Progenitor Cells")]
    ProgenitorCells,
    #[serde(alias = "$resonatingseparators_name;")]
    #[strum(to_string = "Resonating Separators")]
    ResonatingSeparators,
    #[serde(alias = "$bioreducinglichen_name;")]
    #[strum(to_string = "Bioreducing Lichen")]
    BioreducingLichen,
    #[serde(alias = "$atmosphericextractors_name;")]
    #[strum(to_string = "Atmospheric Processors")]
    AtmosphericProcessors,
    #[serde(alias = "$eraninpearlwhisky_name;")]
    #[strum(to_string = "Eranin Pearl Whisky")]
    EraninPearlWhisky,
    #[serde(alias = "$lavianbrandy_name;")]
    #[strum(to_string = "Lavian Brandy")]
    LavianBrandy,
    #[serde(alias = "$usscargorareartwork_name;")]
    #[strum(to_string = "Rare Artwork")]
    RareArtwork,
    #[serde(alias = "$hip10175bushmeat_name;")]
    #[strum(to_string = "HIP 10175 Bush Meat")]
    HIP10175BushMeat,
    #[serde(alias = "$albinoquechuamammoth_name;")]
    #[strum(to_string = "Albino Quechua Mammoth Meat")]
    AlbinoQuechuaMammothMeat,
    #[serde(alias = "$utgaroarmillenialeggs_name;")]
    #[strum(to_string = "Utgaroar Millennial Eggs")]
    UtgaroarMillennialEggs,
    #[serde(alias = "$witchhaulkobebeef_name;")]
    #[strum(to_string = "Witchhaul Kobe Beef")]
    WitchhaulKobeBeef,
    #[serde(alias = "$karsukilocusts_name;")]
    #[strum(to_string = "Karsuki Locusts")]
    KarsukiLocusts,
    #[serde(alias = "$giantirukamasnails_name;")]
    #[strum(to_string = "Giant Irukama Snails")]
    GiantIrukamaSnails,
    #[serde(alias = "$baltahsinevacuumkrill_name;")]
    #[strum(to_string = "Baltah'sine Vacuum Krill")]
    BaltahsineVacuumKrill,
    #[serde(alias = "$cetirabbits_name;")]
    #[strum(to_string = "Ceti Rabbits")]
    CetiRabbits,
    #[serde(alias = "$kachiriginleaches_name;")]
    #[strum(to_string = "Kachirigin Filter Leeches")]
    KachiriginFilterLeeches,
    #[serde(alias = "$lyraeweed_name;")]
    #[strum(to_string = "Lyrae Weed")]
    LyraeWeed,
    #[serde(alias = "$onionhead_name;")]
    #[strum(to_string = "Onionhead")]
    Onionhead,
    #[serde(alias = "$tarachtorspice_name;")]
    #[strum(to_string = "Tarach Spice")]
    TarachSpice,
    #[serde(alias = "$wolf1301fesh_name;")]
    #[strum(to_string = "Wolf Fesh")]
    WolfFesh,
    #[serde(alias = "$konggaale_name;")]
    #[strum(to_string = "Kongga Ale")]
    KonggaAle,
    #[serde(alias = "$wuthielokufroth_name;")]
    #[strum(to_string = "Wuthielo Ku Froth")]
    WuthieloKuFroth,
    #[serde(alias = "$alacarakmoskinart_name;")]
    #[strum(to_string = "Alacarakmo Skin Art")]
    AlacarakmoSkinArt,
    #[serde(alias = "$eleuthermals_name;")]
    #[strum(to_string = "Eleu Thermals")]
    EleuThermals,
    #[serde(alias = "$eshuumbrellas_name;")]
    #[strum(to_string = "Eshu Umbrellas")]
    EshuUmbrellas,
    #[serde(alias = "$karetiicouture_name;")]
    #[strum(to_string = "Karetii Couture")]
    KaretiiCouture,
    #[serde(alias = "$njangarisaddles_name;")]
    #[strum(to_string = "Njangari Saddles")]
    NjangariSaddles,
    #[serde(alias = "$anynacoffee_name;")]
    #[strum(to_string = "Any Na Coffee")]
    AnyNaCoffee,
    #[serde(alias = "$cd75catcoffee_name;")]
    #[strum(to_string = "CD-75 Kitten Brand Coffee")]
    CD75KittenBrandCoffee,
    #[serde(alias = "$gomanyauponcoffee_name;")]
    #[strum(to_string = "Goman Yaupon Coffee")]
    GomanYauponCoffee,
    #[serde(alias = "$volkhabbeedrones_name;")]
    #[strum(to_string = "Volkhab Bee Drones")]
    VolkhabBeeDrones,
    #[serde(alias = "$kinagoinstruments_name;")]
    #[strum(to_string = "Kinago Violins")]
    KinagoViolins,
    #[serde(alias = "$ngunamodernantiques_name;")]
    #[strum(to_string = "Nguna Modern Antiques")]
    NgunaModernAntiques,
    #[serde(alias = "$rajukrustoves_name;")]
    #[strum(to_string = "Rajukru Multi-Stoves")]
    RajukruMultiStoves,
    #[serde(alias = "$tiolcewaste2pasteunits_name;")]
    #[strum(to_string = "Tiolce Waste2Paste Units")]
    TiolceWaste2PasteUnits,
    #[serde(alias = "$chieridanimarinepaste_name;")]
    #[strum(to_string = "Chi Eridani Marine Paste")]
    ChiEridaniMarinePaste,
    #[serde(alias = "$esusekucaviar_name;")]
    #[strum(to_string = "Esuseku Caviar")]
    EsusekuCaviar,
    #[serde(alias = "$livehecateseaworms_name;")]
    #[strum(to_string = "Live Hecate Sea Worms")]
    LiveHecateSeaWorms,
    #[serde(alias = "$helvetitjpearls_name;")]
    #[strum(to_string = "Helvetitj Pearls")]
    HelvetitjPearls,
    #[serde(alias = "$hip41181squid_name;")]
    #[strum(to_string = "HIP Proto-Squid")]
    HIPProtoSquid,
    #[serde(alias = "$coquimspongiformvictuals_name;")]
    #[strum(to_string = "Coquim Spongiform Victuals")]
    CoquimSpongiformVictuals,
    #[serde(alias = "$aerialedenapple_name;")]
    #[strum(to_string = "Eden Apples Of Aerial")]
    EdenApplesOfAerial,
    #[serde(alias = "$neritusberries_name;")]
    #[strum(to_string = "Neritus Berries")]
    NeritusBerries,
    #[serde(alias = "$ochoengchillies_name;")]
    #[strum(to_string = "Ochoeng Chillies")]
    OchoengChillies,
    #[serde(alias = "$deuringastruffles_name;")]
    #[strum(to_string = "Deuringas Truffles")]
    DeuringasTruffles,
    #[serde(alias = "$hr7221wheat_name;")]
    #[strum(to_string = "HR 7221 Wheat")]
    HR7221Wheat,
    #[serde(alias = "$jarouarice_name;")]
    #[strum(to_string = "Jaroua Rice")]
    JarouaRice,
    #[serde(alias = "$belalansrayleather_name;")]
    #[strum(to_string = "Belalans Ray Leather")]
    BelalansRayLeather,
    #[serde(alias = "$damnacarapaces_name;")]
    #[strum(to_string = "Damna Carapaces")]
    DamnaCarapaces,
    #[serde(alias = "$rapabaosnakeskins_name;")]
    #[strum(to_string = "Rapa Bao Snake Skins")]
    RapaBaoSnakeSkins,
    #[serde(alias = "$vanayequirhinofur_name;")]
    #[strum(to_string = "Vanayequi Ceratomorpha Fur")]
    VanayequiCeratomorphaFur,
    #[serde(alias = "$bastsnakegin_name;")]
    #[strum(to_string = "Bast Snake Gin")]
    BastSnakeGin,
    #[serde(alias = "$thrutiscream_name;")]
    #[strum(to_string = "Thrutis Cream")]
    ThrutisCream,
    #[serde(alias = "$wulpahyperboresystems_name;")]
    #[strum(to_string = "Wulpa Hyperbore Systems")]
    WulpaHyperboreSystems,
    #[serde(alias = "$holvaduellingblades_name;")]
    #[strum(to_string = "Holva Duelling Blades")]
    HolvaDuellingBlades,
    #[serde(alias = "$deltaphoenicispalms_name;")]
    #[strum(to_string = "Delta Phoenicis Palms")]
    DeltaPhoenicisPalms,
    #[serde(alias = "$toxandjivirocide_name;")]
    #[strum(to_string = "Toxandji Virocide")]
    ToxandjiVirocide,
    #[serde(alias = "$xihecompanions_name;", alias = "xihecompanions")]
    #[strum(to_string = "Xihe Biomorphic Companions")]
    XiheBiomorphicCompanions,
    #[serde(alias = "$sanumameat_name;")]
    #[strum(to_string = "Sanuma Decorative Meat")]
    SanumaDecorativeMeat,
    #[serde(alias = "$ethgrezeteabuds_name;")]
    #[strum(to_string = "Ethgreze Tea Buds")]
    EthgrezeTeaBuds,
    #[serde(alias = "$ceremonialheiketea_name;")]
    #[strum(to_string = "Ceremonial Heike Tea")]
    CeremonialHeikeTea,
    #[serde(alias = "$tanmarktranquiltea_name;")]
    #[strum(to_string = "Tanmark Tranquil Tea")]
    TanmarkTranquilTea,
    #[serde(alias = "$azcancriformula42_name;")]
    #[strum(to_string = "AZ Cancri Formula 42")]
    AZCancriFormula42,
    #[serde(alias = "$kamitracigars_name;")]
    #[strum(to_string = "Kamitra Cigars")]
    KamitraCigars,
    #[serde(alias = "$rusanioldsmokey_name;")]
    #[strum(to_string = "Rusani Old Smokey")]
    RusaniOldSmokey,
    #[serde(alias = "$yasokondileaf_name;")]
    #[strum(to_string = "Yaso Kondi Leaf")]
    YasoKondiLeaf,
    #[serde(alias = "$chateaudeaegaeon_name;")]
    #[strum(to_string = "Chateau De Aegaeon")]
    ChateauDeAegaeon,
    #[serde(alias = "$watersofshintara_name;")]
    #[strum(to_string = "The Waters Of Shintara")]
    TheWatersOfShintara,
    #[serde(alias = "$ophiuchiexinoartefacts_name;")]
    #[strum(to_string = "Ophiuch Exino Artefacts")]
    OphiuchExinoArtefacts,
    #[serde(alias = "$bakedgreebles_name;")]
    #[strum(to_string = "Baked Greebles")]
    BakedGreebles,
    #[serde(alias = "$cetiaepyornisegg_name;")]
    #[strum(to_string = "Aepyornis Egg")]
    AepyornisEgg,
    #[serde(alias = "$saxonwine_name;")]
    #[strum(to_string = "Saxon Wine")]
    SaxonWine,
    #[serde(alias = "$centaurimegagin_name;")]
    #[strum(to_string = "Centauri Mega Gin")]
    CentauriMegaGin,
    #[serde(alias = "$anduligafireworks_name;")]
    #[strum(to_string = "Anduliga Fire Works")]
    AnduligaFireWorks,
    #[serde(alias = "$bankiamphibiousleather_name;")]
    #[strum(to_string = "Banki Amphibious Leather")]
    BankiAmphibiousLeather,
    #[serde(alias = "$cherbonesbloodcrystals_name;")]
    #[strum(to_string = "Cherbones Blood Crystals")]
    CherbonesBloodCrystals,
    #[serde(alias = "$motronaexperiencejelly_name;")]
    #[strum(to_string = "Motrona Experience Jelly")]
    MotronaExperienceJelly,
    #[serde(alias = "$geawendancedust_name;")]
    #[strum(to_string = "Geawen Dance Dust")]
    GeawenDanceDust,
    #[serde(alias = "$gerasiangueuzebeer_name;")]
    #[strum(to_string = "Gerasian Gueuze Beer")]
    GerasianGueuzeBeer,
    #[serde(alias = "$haidneblackbrew_name;")]
    #[strum(to_string = "Haiden Black Brew")]
    HaidenBlackBrew,
    #[serde(alias = "$havasupaidreamcatcher_name;")]
    #[strum(to_string = "Havasupai Dream Catcher")]
    HavasupaiDreamCatcher,
    #[serde(alias = "$burnhambiledistillate_name;")]
    #[strum(to_string = "Burnham Bile Distillate")]
    BurnhamBileDistillate,
    #[serde(alias = "$hiporganophosphates_name;")]
    #[strum(to_string = "Hip Organophosphates")]
    HipOrganophosphates,
    #[serde(alias = "$jaradharrepuzzlebox_name;")]
    #[strum(to_string = "Jaradharre Puzzle Box")]
    JaradharrePuzzleBox,
    #[serde(alias = "$korrokungpellets_name;")]
    #[strum(to_string = "Koro Kung Pellets")]
    KoroKungPellets,
    #[serde(alias = "$lftvoidextractcoffee_name;")]
    #[strum(to_string = "Void Extract Coffee")]
    VoidExtractCoffee,
    #[serde(alias = "$honestypills_name;")]
    #[strum(to_string = "Honesty Pills")]
    HonestyPills,
    #[serde(alias = "$noneuclidianexotanks_name;")]
    #[strum(to_string = "Non Euclidian Exotanks")]
    NonEuclidianExotanks,
    #[serde(alias = "$ltthypersweet_name;")]
    #[strum(to_string = "LTT Hyper Sweet")]
    LTTHyperSweet,
    #[serde(alias = "$mechucoshightea_name;")]
    #[strum(to_string = "Mechucos High Tea")]
    MechucosHighTea,
    #[serde(alias = "$medbstarlube_name;")]
    #[strum(to_string = "Medb Starlube")]
    MedbStarlube,
    #[serde(alias = "$mokojingbeastfeast_name;")]
    #[strum(to_string = "Mokojing Beast Feast")]
    MokojingBeastFeast,
    #[serde(alias = "$mukusubiichitinos_name;")]
    #[strum(to_string = "Mukusubii Chitin-os")]
    MukusubiiChitinos,
    #[serde(alias = "$mulachigiantfungus_name;")]
    #[strum(to_string = "Mulachi Giant Fungus")]
    MulachiGiantFungus,
    #[serde(alias = "$ngadandarifireopals_name;")]
    #[strum(to_string = "Ngadandari Fire Opals")]
    NgadandariFireOpals,
    #[serde(alias = "$tiegfriessynthsilk_name;")]
    #[strum(to_string = "Tiegfries Synth Silk")]
    TiegfriesSynthSilk,
    #[serde(alias = "$uzumokulowgwings_name;")]
    #[strum(to_string = "Uzumoku Low-G Wings")]
    UzumokuLowGWings,
    #[serde(alias = "$vherculisbodyrub_name;")]
    #[strum(to_string = "V Herculis Body Rub")]
    VHerculisBodyRub,
    #[serde(alias = "$wheemetewheatcakes_name;")]
    #[strum(to_string = "Wheemete Wheat Cakes")]
    WheemeteWheatCakes,
    #[serde(alias = "$vegaslimweed_name;")]
    #[strum(to_string = "Vega Slimweed")]
    VegaSlimweed,
    #[serde(alias = "$altairianskin_name;")]
    #[strum(to_string = "Altairian Skin")]
    AltairianSkin,
    #[serde(alias = "$jotunmookah_name;")]
    #[strum(to_string = "Jotun Mookah")]
    JotunMookah,
    #[serde(alias = "$giantverrix_name;")]
    #[strum(to_string = "Giant Verrix")]
    GiantVerrix,
    #[serde(alias = "$indibourbon_name;")]
    #[strum(to_string = "Indi Bourbon")]
    IndiBourbon,
    #[serde(alias = "$aroucaconventualsweets_name;")]
    #[strum(to_string = "Arouca Conventual Sweets")]
    AroucaConventualSweets,
    #[serde(alias = "$taurichimes_name;")]
    #[strum(to_string = "Tauri Chimes")]
    TauriChimes,
    #[serde(alias = "$zeesszeantglue_name;")]
    #[strum(to_string = "Zeessze Ant Grub Glue")]
    ZeesszeAntGrubGlue,
    #[serde(alias = "$pantaaprayersticks_name;")]
    #[strum(to_string = "Pantaa Prayer Sticks")]
    PantaaPrayerSticks,
    #[serde(alias = "$fujintea_name;")]
    #[strum(to_string = "Fujin Tea")]
    FujinTea,
    #[serde(alias = "$chameleoncloth_name;")]
    #[strum(to_string = "Chameleon Cloth")]
    ChameleonCloth,
    #[serde(alias = "$orrerianviciousbrew_name;")]
    #[strum(to_string = "Orrerian Vicious Brew")]
    OrrerianViciousBrew,
    #[serde(alias = "$uszaiantreegrub_name;")]
    #[strum(to_string = "Uszaian Tree Grub")]
    UszaianTreeGrub,
    #[serde(alias = "$momusbogspaniel_name;")]
    #[strum(to_string = "Momus Bog Spaniel")]
    MomusBogSpaniel,
    #[serde(alias = "$disomacorn_name;")]
    #[strum(to_string = "Diso Ma Corn")]
    DisoMaCorn,
    #[serde(alias = "$leestianeviljuice_name;")]
    #[strum(to_string = "Leestian Evil Juice")]
    LeestianEvilJuice,
    #[serde(alias = "$bluemilk_name;")]
    #[strum(to_string = "Azure Milk")]
    AzureMilk,
    #[serde(alias = "$alieneggs_name;", alias = "alieneggs")]
    #[strum(to_string = "Leathery Eggs")]
    LeatheryEggs,
    #[serde(alias = "$alyabodilysoap_name;")]
    #[strum(to_string = "Alya Body Soap")]
    AlyaBodySoap,
    #[serde(alias = "$vidavantianlace_name;")]
    #[strum(to_string = "Vidavantian Lace")]
    VidavantianLace,
    #[serde(alias = "$imperialslaves_name;")]
    #[strum(to_string = "Imperial Slaves")]
    ImperialSlaves,
    #[serde(alias = "$MARKET_category_slaves;")]
    #[strum(to_string = "Slavery")]
    Slavery,
    #[serde(alias = "$transgeniconionhead_name;")]
    #[strum(to_string = "Lucan Onionhead")]
    LucanOnionhead,
    #[serde(alias = "$jaquesquinentianstill_name;")]
    #[strum(to_string = "Jaques Quinentian Still")]
    JaquesQuinentianStill,
    #[serde(alias = "$soontillrelics_name;")]
    #[strum(to_string = "Soontill Relics")]
    SoontillRelics,
    #[serde(
        alias = "$unknownartifact_name;",
        alias = "unknownartifact",
        alias = "UnknownArtifact"
    )]
    #[strum(to_string = "Thargoid Sensor")]
    ThargoidSensor,
    #[serde(alias = "$advert1_name;")]
    #[strum(to_string = "Ultra-Compact Processor Prototypes")]
    UltraCompactProcessorPrototypes,
    #[serde(alias = "$sap8corecontainer_name;")]
    #[strum(to_string = "SAP 8 Core Container")]
    SAP8CoreContainer,
    #[serde(alias = "$thehuttonmug_name;")]
    #[strum(to_string = "The Hutton Mug")]
    TheHuttonMug,
    #[serde(alias = "$sothiscrystallinegold_name;")]
    #[strum(to_string = "Sothis Crystalline Gold")]
    SothisCrystallineGold,
    #[serde(
        alias = "$encripteddatastorage_name;",
        alias = "encripteddatastorage",
        alias = "EncriptedDataStorage"
    )]
    #[strum(to_string = "Encrypted Data Storage")]
    EncryptedDataStorage,
    #[serde(alias = "$comercialsamples_name;", alias = "ComercialSamples")]
    #[strum(to_string = "Commercial Samples")]
    CommercialSamples,
    #[serde(alias = "$tacticaldata_name;")]
    #[strum(to_string = "Tactical Data")]
    TacticalData,
    #[serde(alias = "$assaultplans_name;")]
    #[strum(to_string = "Assault Plans")]
    AssaultPlans,
    #[serde(alias = "$encryptedcorrespondence_name;")]
    #[strum(to_string = "Encrypted Correspondence")]
    EncryptedCorrespondence,
    #[serde(alias = "$diplomaticbag_name;")]
    #[strum(to_string = "Diplomatic Bag")]
    DiplomaticBag,
    #[serde(alias = "$scientificresearch_name;")]
    #[strum(to_string = "Scientific Research")]
    ScientificResearch,
    #[serde(alias = "$scientificsamples_name;")]
    #[strum(to_string = "Scientific Samples")]
    ScientificSamples,
    #[serde(alias = "$largeexplorationdatacash_name;")]
    #[strum(to_string = "Large Survey Data Cache")]
    LargeSurveyDataCache,
    #[serde(alias = "$smallexplorationdatacash_name;")]
    #[strum(to_string = "Small Survey Data Cache")]
    SmallSurveyDataCache,
    #[serde(alias = "$antiquejewellery_name;")]
    #[strum(to_string = "Antique Jewellery")]
    AntiqueJewellery,
    #[serde(alias = "$preciousgems_name;")]
    #[strum(to_string = "Precious Gems")]
    PreciousGems,
    #[serde(alias = "$earthrelics_name;")]
    #[strum(to_string = "Earth Relics")]
    EarthRelics,
    #[serde(alias = "$genebank_name;")]
    #[strum(to_string = "Gene Bank")]
    GeneBank,
    #[serde(alias = "$timecapsule_name;")]
    #[strum(to_string = "Time Capsule")]
    TimeCapsule,
    #[serde(alias = "$ceramiccomposites_name;")]
    #[strum(to_string = "Ceramic Composites")]
    CeramicComposites,
    #[serde(alias = "$syntheticreagents_name;")]
    #[strum(to_string = "Synthetic Reagents")]
    SyntheticReagents,
    #[serde(alias = "$nerveagents_name;")]
    #[strum(to_string = "Nerve Agents")]
    NerveAgents,
    #[serde(alias = "$surfacestabilisers_name;")]
    #[strum(to_string = "Surface Stabilisers")]
    SurfaceStabilisers,
    #[serde(alias = "$bootlegliquor_name;")]
    #[strum(to_string = "Bootleg Liquor")]
    BootlegLiquor,
    #[serde(alias = "$geologicalequipment_name;")]
    #[strum(to_string = "Geological Equipment")]
    GeologicalEquipment,
    #[serde(alias = "$thermalcoolingunits_name;")]
    #[strum(to_string = "Thermal Cooling Units")]
    ThermalCoolingUnits,
    #[serde(alias = "$buildingfabricators_name;")]
    #[strum(to_string = "Building Fabricators")]
    BuildingFabricators,
    #[serde(alias = "$mutomimager_name;")]
    #[strum(to_string = "Muon Imager")]
    MuonImager,
    #[serde(alias = "$structuralregulators_name;")]
    #[strum(to_string = "Structural Regulators")]
    StructuralRegulators,
    #[serde(alias = "$skimercomponents_name;", alias = "skimercomponents")]
    #[strum(to_string = "Skimmer Components")]
    SkimmerComponents,
    #[serde(alias = "$evacuationshelter_name;")]
    #[strum(to_string = "Evacuation Shelter")]
    EvacuationShelter,
    #[serde(alias = "$geologicalsamples_name;")]
    #[strum(to_string = "Geological Samples")]
    GeologicalSamples,
    #[serde(alias = "$masterchefs_name;")]
    #[strum(to_string = "Master Chefs")]
    MasterChefs,
    #[serde(alias = "$personalgifts_name;", alias = "personalgifts")]
    #[strum(to_string = "Festive Gifts")]
    FestiveGifts,
    #[serde(alias = "$crystallinespheres_name;")]
    #[strum(to_string = "Crystalline Spheres")]
    CrystallineSpheres,
    #[serde(alias = "$metaalloys_name;", alias = "MetaAlloys")]
    #[strum(to_string = "Meta-Alloys")]
    MetaAlloys,
    #[serde(alias = "$unstabledatacore_name;")]
    #[strum(to_string = "Unstable Data Core")]
    UnstableDataCore,
    #[serde(alias = "$onionheada_name;")]
    #[strum(to_string = "Onionhead Alpha Strain")]
    OnionheadAlphaStrain,
    #[serde(alias = "$onionheadb_name;")]
    #[strum(to_string = "Onionhead Beta Strain")]
    OnionheadBetaStrain,
    #[serde(alias = "$hydrogenperoxide_name;")]
    #[strum(to_string = "Hydrogen Peroxide")]
    HydrogenPeroxide,
    #[serde(alias = "$liquidoxygen_name;")]
    #[strum(to_string = "Liquid oxygen")]
    Liquidoxygen,
    #[serde(alias = "$methanolmonohydratecrystals_name;")]
    #[strum(to_string = "Methanol Monohydrate Crystals")]
    MethanolMonohydrateCrystals,
    #[serde(alias = "$lithiumhydroxide_name;")]
    #[strum(to_string = "Lithium Hydroxide")]
    LithiumHydroxide,
    #[serde(alias = "$methaneclathrate_name;")]
    #[strum(to_string = "Methane Clathrate")]
    MethaneClathrate,
    #[serde(alias = "$insulatingmembrane_name;")]
    #[strum(to_string = "Insulating Membrane")]
    InsulatingMembrane,
    #[serde(alias = "$cmmcomposite_name;")]
    #[strum(to_string = "CMM Composite")]
    CMMComposite,
    #[serde(alias = "$coolinghoses_name;", alias = "coolinghoses")]
    #[strum(to_string = "Micro-weave Cooling Hoses")]
    MicroweaveCoolingHoses,
    #[serde(alias = "$neofabricinsulation_name;")]
    #[strum(to_string = "Neofabric Insulation")]
    NeofabricInsulation,
    #[serde(alias = "$articulationmotors_name;")]
    #[strum(to_string = "Articulation Motors")]
    ArticulationMotors,
    #[serde(alias = "$hnshockmount_name;")]
    #[strum(to_string = "HN Shock Mount")]
    HNShockMount,
    #[serde(alias = "$emergencypowercells_name;")]
    #[strum(to_string = "Emergency Power Cells")]
    EmergencyPowerCells,
    #[serde(alias = "$powerconverter_name;")]
    #[strum(to_string = "Power Converter")]
    PowerConverter,
    #[serde(alias = "$powergridassembly_name;")]
    #[strum(to_string = "Energy Grid Assembly")]
    EnergyGridAssembly,
    #[serde(
        alias = "$powertransferconduits_name;",
        alias = "powertransferconduits"
    )]
    #[strum(to_string = "Power Transfer Bus")]
    PowerTransferBus,
    #[serde(alias = "$radiationbaffle_name;")]
    #[strum(to_string = "Radiation Baffle")]
    RadiationBaffle,
    #[serde(alias = "$exhaustmanifold_name;")]
    #[strum(to_string = "Exhaust Manifold")]
    ExhaustManifold,
    #[serde(alias = "$reinforcedmountingplate_name;")]
    #[strum(to_string = "Reinforced Mounting Plate")]
    ReinforcedMountingPlate,
    #[serde(alias = "$heatsinkinterlink_name;")]
    #[strum(to_string = "Heatsink Interlink")]
    HeatsinkInterlink,
    #[serde(alias = "$magneticemittercoil_name;")]
    #[strum(to_string = "Magnetic Emitter Coil")]
    MagneticEmitterCoil,
    #[serde(alias = "$modularterminals_name;")]
    #[strum(to_string = "Modular Terminals")]
    ModularTerminals,
    #[serde(alias = "$telemetrysuite_name;")]
    #[strum(to_string = "Telemetry Suite")]
    TelemetrySuite,
    #[serde(alias = "$microcontrollers_name;")]
    #[strum(to_string = "Micro Controllers")]
    MicroControllers,
    #[serde(alias = "$iondistributor_name;")]
    #[strum(to_string = "Ion Distributor")]
    IonDistributor,
    #[serde(alias = "$diagnosticsensor_name;", alias = "diagnosticsensor")]
    #[strum(to_string = "Hardware Diagnostic Sensor")]
    HardwareDiagnosticSensor,
    #[serde(alias = "$unknownartifact2_name;")]
    #[strum(to_string = "Thargoid Probe")]
    ThargoidProbe,
    #[serde(alias = "$conductivefabrics_name;")]
    #[strum(to_string = "Conductive Fabrics")]
    ConductiveFabrics,
    #[serde(alias = "$militarygradefabrics_name;")]
    #[strum(to_string = "Military Grade Fabrics")]
    MilitaryGradeFabrics,
    #[serde(alias = "$advancedmedicines_name;")]
    #[strum(to_string = "Advanced Medicines")]
    AdvancedMedicines,
    #[serde(alias = "$medicaldiagnosticequipment_name;")]
    #[strum(to_string = "Medical Diagnostic Equipment")]
    MedicalDiagnosticEquipment,
    #[serde(alias = "$survivalequipment_name;")]
    #[strum(to_string = "Survival Equipment")]
    SurvivalEquipment,
    #[serde(alias = "$datacore_name;")]
    #[strum(to_string = "Data Core")]
    DataCore,
    #[serde(alias = "$galactictravelguide_name;")]
    #[strum(to_string = "Galactic Travel Guide")]
    GalacticTravelGuide,
    #[serde(alias = "$mysteriousidol_name;")]
    #[strum(to_string = "Mysterious Idol")]
    MysteriousIdol,
    #[serde(alias = "$spacepioneerrelics_name;")]
    #[strum(to_string = "Space Pioneer Relics")]
    SpacePioneerRelics,
    #[serde(alias = "$fossilremnants_name;")]
    #[strum(to_string = "Fossil Remnants")]
    FossilRemnants,
    #[serde(alias = "$animaleffigies_name;")]
    #[strum(to_string = "Crom Silver Fesh")]
    CromSilverFesh,
    #[serde(
        alias = "$ancientrelic_name;",
        alias = "ancientrelic",
        alias = "AncientRelic"
    )]
    #[strum(to_string = "Guardian Relic")]
    GuardianRelic,
    #[serde(alias = "$ancientorb_name;", alias = "ancientorb")]
    #[strum(to_string = "Guardian Orb")]
    GuardianOrb,
    #[serde(alias = "$ancientcasket_name;", alias = "ancientcasket")]
    #[strum(to_string = "Guardian Casket")]
    GuardianCasket,
    #[serde(alias = "$ancienttablet_name;", alias = "ancienttablet")]
    #[strum(to_string = "Guardian Tablet")]
    GuardianTablet,
    #[serde(alias = "$ancienturn_name;", alias = "ancienturn")]
    #[strum(to_string = "Guardian Urn")]
    GuardianUrn,
    #[serde(alias = "$ancienttotem_name;", alias = "ancienttotem")]
    #[strum(to_string = "Guardian Totem")]
    GuardianTotem,
    #[serde(alias = "$shanscharisorchid_name;")]
    #[strum(to_string = "Shan's Charis Orchid")]
    ShansCharisOrchid,
    #[serde(
        alias = "$unknownresin_name;",
        alias = "unknownresin",
        alias = "UnknownResin"
    )]
    #[strum(to_string = "Thargoid Resin")]
    ThargoidResin,
    #[serde(alias = "$unknownbiologicalmatter_name;")]
    #[strum(to_string = "Thargoid Biological Matter")]
    ThargoidBiologicalMatter,
    #[serde(alias = "$unknowntechnologysamples_name;")]
    #[strum(to_string = "Thargoid Technology Samples")]
    ThargoidTechnologySamples,
    #[serde(alias = "$unknownartifact3_name;")]
    #[strum(to_string = "Thargoid Link")]
    ThargoidLink,
    #[serde(alias = "$buckyballbeermats_name;")]
    #[strum(to_string = "Buckyball Beer Mats")]
    BuckyballBeerMats,
    #[serde(alias = "$harmasilversearum_name;")]
    #[strum(to_string = "Harma Silver Sea Rum")]
    HarmaSilverSeaRum,
    #[serde(alias = "$platinumaloy_name;")]
    #[strum(to_string = "Platinum Alloy")]
    PlatinumAlloy,
    #[serde(
        alias = "$thargoidheart_name;",
        alias = "thargoidheart",
        alias = "ThargoidHeart"
    )]
    #[strum(to_string = "Thargoid Heart")]
    ThargoidHeart,
    #[serde(
        alias = "$thargoidtissuesampletype1_name;",
        alias = "thargoidtissuesampletype1",
        alias = "ThargoidTissueSampleType1"
    )]
    #[strum(to_string = "Thargoid Cyclops Tissue Sample")]
    ThargoidCyclopsTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype2_name;",
        alias = "thargoidtissuesampletype2",
        alias = "ThargoidTissueSampleType2"
    )]
    #[strum(to_string = "Thargoid Basilisk Tissue Sample")]
    ThargoidBasiliskTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype3_name;",
        alias = "thargoidtissuesampletype3",
        alias = "ThargoidTissueSampleType3"
    )]
    #[strum(to_string = "Thargoid Medusa Tissue Sample")]
    ThargoidMedusaTissueSample,
    #[serde(alias = "$thargoidscouttissuesample_name;")]
    #[strum(to_string = "Thargoid Scout Tissue Sample")]
    ThargoidScoutTissueSample,
    #[serde(alias = "$ancientkey_name;")]
    #[strum(to_string = "Ancient Key")]
    AncientKey,
    #[serde(
        alias = "$thargoidtissuesampletype4_name;",
        alias = "thargoidtissuesampletype4",
        alias = "ThargoidTissueSampleType4"
    )]
    #[strum(to_string = "Thargoid Hydra Tissue Sample")]
    ThargoidHydraTissueSample,
    #[serde(alias = "$m_tissuesample_fluid_name;")]
    #[strum(to_string = "Mollusc Fluid")]
    MolluscFluid,
    #[serde(alias = "$m_tissuesample_soft_name;")]
    #[strum(to_string = "Mollusc Soft Tissue")]
    MolluscSoftTissue,
    #[serde(alias = "$m_tissuesample_nerves_name;")]
    #[strum(to_string = "Mollusc Brain Tissue")]
    MolluscBrainTissue,
    #[serde(alias = "$s_tissuesample_cells_name;")]
    #[strum(to_string = "Pod Core Tissue")]
    PodCoreTissue,
    #[serde(alias = "$s_tissuesample_surface_name;")]
    #[strum(to_string = "Pod Dead Tissue")]
    PodDeadTissue,
    #[serde(alias = "$s_tissuesample_core_name;")]
    #[strum(to_string = "Pod Surface Tissue")]
    PodSurfaceTissue,
    #[serde(alias = "$p_particulatesample_name;")]
    #[strum(to_string = "Anomaly Particles")]
    AnomalyParticles,
    #[serde(alias = "$s9_tissuesample_shell_name;")]
    #[strum(to_string = "Pod Tissue")]
    PodTissue,
    #[serde(alias = "$m3_tissuesample_membrane_name;")]
    #[strum(to_string = "Mollusc Membrane")]
    MolluscMembrane,
    #[serde(alias = "$m3_tissuesample_mycelium_name;")]
    #[strum(to_string = "Mollusc Mycelium")]
    MolluscMycelium,
    #[serde(alias = "$m3_tissuesample_spores_name;")]
    #[strum(to_string = "Mollusc Spores")]
    MolluscSpores,
    #[serde(alias = "$s6_tissuesample_mesoglea_name;")]
    #[strum(to_string = "Pod Mesoglea")]
    PodMesoglea,
    #[serde(alias = "$s6_tissuesample_cells_name;")]
    #[strum(to_string = "Pod Outer Tissue")]
    PodOuterTissue,
    #[serde(alias = "$s6_tissuesample_coenosarc_name;")]
    #[strum(to_string = "Pod Shell Tissue")]
    PodShellTissue,
    #[serde(alias = "$grandidierite_name;")]
    #[strum(to_string = "Grandidierite")]
    Grandidierite,
    #[serde(alias = "$rockforthfertiliser_name;")]
    #[strum(to_string = "Rockforth Fertiliser")]
    RockforthFertiliser,
    #[serde(alias = "$agronomictreatment_name;")]
    #[strum(to_string = "Agronomic Treatment")]
    AgronomicTreatment,
    #[serde(alias = "$apavietii_name;")]
    #[strum(to_string = "Apa Vietii")]
    ApaVietii,
    #[serde(alias = "$classifiedexperimentalequipment_name;")]
    #[strum(to_string = "Classified Experimental Equipment")]
    ClassifiedExperimentalEquipment,
    #[serde(alias = "$ancientrelictg_name;")]
    #[strum(to_string = "Unclassified Relic")]
    UnclassifiedRelic,
    #[serde(
        alias = "$thargoidtissuesampletype5_name;",
        alias = "thargoidtissuesampletype5",
        alias = "ThargoidTissueSampleType5"
    )]
    #[strum(to_string = "Thargoid Orthrus Tissue Sample")]
    ThargoidOrthrusTissueSample,
    #[serde(
        alias = "$thargoidgeneratortissuesample_name;",
        alias = "thargoidgeneratortissuesample",
        alias = "ThargoidGeneratorTissueSample"
    )]
    #[strum(to_string = "Caustic Tissue Sample")]
    CausticTissueSample,
    #[serde(alias = "$unocuppiedescapepod_name;")]
    #[strum(to_string = "Unoccupied Escape Pod")]
    UnoccupiedEscapePod,
    #[serde(
        alias = "$thargoidtissuesampletype6_name;",
        alias = "thargoidtissuesampletype6",
        alias = "ThargoidTissueSampleType6"
    )]
    #[strum(to_string = "Thargoid Glaive Tissue Sample")]
    ThargoidGlaiveTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype7_name;",
        alias = "thargoidtissuesampletype7",
        alias = "ThargoidTissueSampleType7"
    )]
    #[strum(to_string = "Thargoid Scythe Tissue Sample")]
    ThargoidScytheTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype9a_name;",
        alias = "thargoidtissuesampletype9a",
        alias = "ThargoidTissueSampleType9a"
    )]
    #[strum(to_string = "Titan Deep Tissue Sample")]
    TitanDeepTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype9b_name;",
        alias = "thargoidtissuesampletype9b",
        alias = "ThargoidTissueSampleType9b"
    )]
    #[strum(to_string = "Titan Tissue Sample")]
    TitanTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype9c_name;",
        alias = "thargoidtissuesampletype9c",
        alias = "ThargoidTissueSampleType9c"
    )]
    #[strum(to_string = "Titan Partial Tissue Sample")]
    TitanPartialTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype10a_name;",
        alias = "thargoidtissuesampletype10a",
        alias = "ThargoidTissueSampleType10a"
    )]
    #[strum(to_string = "Titan Maw Deep Tissue Sample")]
    TitanMawDeepTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype10b_name;",
        alias = "thargoidtissuesampletype10b",
        alias = "ThargoidTissueSampleType10b"
    )]
    #[strum(to_string = "Titan Maw Tissue Sample")]
    TitanMawTissueSample,
    #[serde(
        alias = "$thargoidtissuesampletype10c_name;",
        alias = "thargoidtissuesampletype10c",
        alias = "ThargoidTissueSampleType10c"
    )]
    #[strum(to_string = "Titan Maw Partial Tissue Sample")]
    TitanMawPartialTissueSample,
    #[serde(
        alias = "$unknownsack_name;",
        alias = "unknownsack",
        alias = "UnknownSack"
    )]
    #[strum(to_string = "Protective Membrane Scrap")]
    ProtectiveMembraneScrap,
    #[serde(alias = "$coralsap_name;")]
    #[strum(to_string = "Coral Sap")]
    CoralSap,
    #[serde(alias = "$unknownmineral_name;")]
    #[strum(to_string = "Impure Spire Mineral")]
    ImpureSpireMineral,
    #[serde(alias = "$unknownrefinedmineral_name;")]
    #[strum(to_string = "Semi-Refined Spire Mineral")]
    SemiRefinedSpireMineral,
    #[serde(
        alias = "$thargoidtitandrivecomponent_name;",
        alias = "thargoidtitandrivecomponent",
        alias = "ThargoidTitanDriveComponent"
    )]
    #[strum(to_string = "Titan Drive Component")]
    TitanDriveComponent,
    #[serde(
        alias = "$thargoidcystspecimen_name;",
        alias = "thargoidcystspecimen",
        alias = "ThargoidCystSpecimen"
    )]
    #[strum(to_string = "Cyst Specimen")]
    CystSpecimen,
    #[serde(
        alias = "$thargoidbonefragments_name;",
        alias = "thargoidbonefragments",
        alias = "ThargoidBoneFragments"
    )]
    #[strum(to_string = "Bone Fragments")]
    BoneFragments,
    #[serde(
        alias = "$thargoidorgansample_name;",
        alias = "thargoidorgansample",
        alias = "ThargoidOrganSample"
    )]
    #[strum(to_string = "Organ Sample")]
    OrganSample,
    #[serde(alias = "$curatedcommodity_name;", alias = "curatedcommodity")]
    #[strum(to_string = "Curated Commodity Package")]
    CuratedCommodityPackage,
    #[serde(alias = "$aganipperush_name;")]
    #[strum(to_string = "Aganippe Rush")]
    AganippeRush,
    #[serde(alias = "$terramaterbloodbores_name;")]
    #[strum(to_string = "Terra Mater Blood Bores")]
    TerraMaterBloodBores,
    #[serde(alias = "$kamorinhistoricweapons_name;")]
    #[strum(to_string = "Kamorin Historic Weapons")]
    KamorinHistoricWeapons,
    #[serde(alias = "$gilyasignatureweapons_name;")]
    #[strum(to_string = "Gilya Signature Weapons")]
    GilyaSignatureWeapons,
    #[serde(alias = "$pavoniseargrubs_name;")]
    #[strum(to_string = "Pavonis Ear Grubs")]
    PavonisEarGrubs,
    #[serde(alias = "$onionheadc_name;")]
    #[strum(to_string = "Onionhead Gamma Strain")]
    OnionheadGammaStrain,
    #[serde(alias = "$fruitandvegetables_name;")]
    #[strum(to_string = "Fruit and Vegetables")]
    FruitandVegetables,
    #[serde(alias = "$animalmeat_name;")]
    #[strum(to_string = "Animal Meat")]
    AnimalMeat,
    #[serde(alias = "$foodcartridges_name;", alias = "FoodCartridges")]
    #[strum(to_string = "Food Cartridges")]
    FoodCartridges,
    #[serde(alias = "$syntheticmeat_name;")]
    #[strum(to_string = "Synthetic Meat")]
    SyntheticMeat,
    #[serde(alias = "$naturalfabrics_name;")]
    #[strum(to_string = "Natural Fabrics")]
    NaturalFabrics,
    #[serde(alias = "$syntheticfabrics_name;", alias = "SyntheticFabrics")]
    #[strum(to_string = "Synthetic Fabrics")]
    SyntheticFabrics,
    #[serde(alias = "Clothing", alias = "$clothing_name;")]
    Clothing,
    #[serde(alias = "$explosives_name;")]
    Explosives,
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
    #[serde(alias = "$coffee_name;")]
    Coffee,
    #[serde(alias = "Grain", alias = "$grain_name;")]
    Grain,
    #[serde(alias = "$fish_name;")]
    Fish,
    #[serde(alias = "Algae", alias = "$algae_name;")]
    Algae,
    #[serde(alias = "Liquor", alias = "$liquor_name;")]
    Liquor,
    #[serde(alias = "$beer_name;")]
    Beer,
    #[serde(alias = "Wine", alias = "$wine_name;")]
    Wine,
    #[serde(alias = "$biowaste_name;")]
    Biowaste,
    #[serde(alias = "$landmines_name;")]
    Landmines,
    #[serde(alias = "Tobacco", alias = "$tobacco_name;")]
    Tobacco,
    #[serde(alias = "$tea_name;")]
    Tea,
    #[serde(alias = "PersonalEffects")]
    PersonalEffects,
    #[serde(alias = "Pesticides", alias = "$pesticides_name;")]
    Pesticides,
    #[serde(alias = "Polymers", alias = "$polymers_name;")]
    Polymers,
    #[serde(alias = "$hostage_name;")]
    Hostage,
    #[serde(alias = "Scrap", alias = "$scrap_name;")]
    Scrap,
    #[serde(alias = "$leather_name;")]
    Leather,
    #[serde(alias = "$robotics_name;")]
    Robotics,
    #[serde(alias = "$restrictedintel_name;")]
    RestrictedIntel,
    #[serde(alias = "$militaryintelligence_name;")]
    MilitaryIntelligence,
    #[serde(alias = "$alliancetradeagreements_name;")]
    AllianceTradeAgreements,

    #[strum(to_string = "Limpet")]
    #[serde(alias = "Drones")]
    Drones,
    #[serde(alias = "DamagedEscapePod")]
    DamagedEscapePod,

    #[serde(alias = "WreckageComponents")]
    WreckageComponents,
    AislingMediaMaterials,
    AislingPromotionalMaterials,
    RepublicanFieldSupplies,
    RepublicanGarisonSupplies,

    #[serde(alias = "$pyrophyllite_name;")]
    Pyrophyllite,
    #[serde(alias = "$bismuth_name;")]
    Bismuth,
    #[serde(alias = "$cryolite_name;")]
    Cryolite,
    #[serde(alias = "$goslarite_name;")]
    Goslarite,
    #[serde(alias = "$nanobreakers_name;")]
    Nanobreakers,
    #[serde(alias = "$nanomedicines_name;")]
    Nanomedicines,
    #[serde(alias = "$rhodplumsite_name;")]
    Rhodplumsite,
    #[serde(alias = "$slaves_name;")]
    Slaves,

    #[serde(alias = "OccupiedCryoPod")]
    OccupiedCryoPod,

    #[serde(alias = "ThargoidPod", alias = "$thargoidpod_name;")]
    ThargoidPod,
    #[serde(alias = "$duradrives_name;")]
    Duradrives,
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

#[derive(Serialize, Deserialize, Clone, Debug, Display)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub enum MarketItemCategory {
    #[serde(alias = "$MARKET_category_chemicals;")]
    Chemicals,
    #[serde(alias = "$MARKET_category_consumer_items;")]
    #[strum(to_string = "Consumer items")]
    Consumeritems,
    #[serde(alias = "$MARKET_category_drugs;")]
    #[strum(to_string = "Legal drugs")]
    Legaldrugs,
    #[serde(alias = "$MARKET_category_foods;")]
    Foods,
    #[serde(alias = "$MARKET_category_industrial_materials;")]
    #[strum(to_string = "Industrial materials")]
    Industrialmaterials,
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
    #[serde(alias = "$MARKET_category_slaves;")]
    #[strum(to_string = "Slavery")]
    Slavery,
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
pub struct EDLogMarketID {}

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

#[test]
// read these market data and check if they parse.
fn test_market_examples() {
    let json = [
        include_str!("../testdata/Market - rare goods.json"),
        include_str!("../testdata/Market - robardin-rock.json"),
    ];

    let lines: Result<Vec<crate::log_line::EDLogLine>, _> =
        json.into_iter().map(serde_json::from_str).collect();

    if let Ok(lines) = lines {
        for line in lines {
            assert!(matches!(
                line.event(),
                crate::log_line::EDLogEvent::Market(_)
            ));
        }
    } else {
        use chrono::{DateTime, Utc};

        // create untyped struct to parse market data using strings

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase", deny_unknown_fields)]
        #[allow(unused)]
        pub struct SimpleEDLogMarket {
            #[serde(rename = "MarketID")]
            pub market_id: u64,
            pub station_name: String,
            pub station_type: StationType,
            pub carrier_docking_access: Option<CarrierDockingAccess>,
            pub star_system: String,
            pub items: Option<Vec<SimpleMarketItem>>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "PascalCase", deny_unknown_fields)]
        #[allow(unused)]
        pub struct SimpleMarketItem {
            #[serde(rename = "id")]
            id: u64,
            #[serde(rename = "Name")]
            pub market_item_name: String,
            #[serde(rename = "Name_Localised")]
            pub market_item_name_localised: Option<String>,
            pub category: String,
            #[serde(rename = "Category_Localised")]
            pub category_localised: Option<String>,
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

        #[derive(Deserialize, Debug)]
        #[allow(unused)]
        pub struct SimpleEDLogLine {
            timestamp: DateTime<Utc>,
            #[serde(flatten)]
            event: SimpleEDLogEvent,
        }

        #[derive(Deserialize, Debug)]
        #[serde(tag = "event", deny_unknown_fields)]
        pub enum SimpleEDLogEvent {
            Market(SimpleEDLogMarket),
        }

        // parse market data into simple structs

        let lines: Vec<SimpleEDLogLine> = json
            .into_iter()
            .map(|j| serde_json::from_str(j).expect("Simple variant of datastructure should parse"))
            .collect();

        let mut err = false;
        for line in lines {
            let SimpleEDLogEvent::Market(data) = line.event;

            // check all items in the simple market data and try to parse into market_item_type
            for item in data.items.expect("testdata should contain market_items") {
                let name: Result<MarketItemType, _> =
                    serde_json::from_str(&format!("\"{}\"", item.market_item_name));
                let name_loc = item.market_item_name_localised.clone();

                if name.is_err()
                    || (item.market_item_name_localised.is_some()
                        && name.unwrap().to_string() != name_loc.unwrap())
                {
                    err = true;

                    let pcname = item
                        .market_item_name_localised
                        .as_ref()
                        .map(|s| s.split(" ").collect::<Vec<_>>().join(""));

                    println!(
                        "#[serde(alias = \"{}\")]#[strum(to_string = \"{}\")]{},",
                        item.market_item_name,
                        item.market_item_name_localised
                            .as_ref()
                            .unwrap_or(&item.market_item_name),
                        pcname.unwrap_or(item.market_item_name.clone())
                    );
                }

                let cat: Result<MarketItemCategory, _> =
                    serde_json::from_str(&format!("\"{}\"", item.category));
                let cat_loc = item.category_localised.clone();

                let pcname = item
                    .category_localised
                    .as_ref()
                    .map(|s| s.split(" ").collect::<Vec<_>>().join(""));

                if cat.is_err()
                    || (item.category_localised.is_some()
                        && cat.unwrap().to_string() != cat_loc.unwrap())
                {
                    err = true;
                    println!(
                        "#[serde(alias = \"{}\")]#[strum(to_string=\"{}\")]{},",
                        item.category,
                        item.category_localised.as_ref().unwrap_or(&item.category),
                        pcname.unwrap_or(item.category.clone())
                    );
                }
            }
        }

        assert!(!err);
    }
}
