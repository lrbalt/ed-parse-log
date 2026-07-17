use crate::{EDString, ship_type::ShipType};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::Display;

pub mod serde_ship_module {
    use crate::{
        ship_module::{
            HardpointConnection, HardpointSize, ShipArmourGrade, ShipModule, ShipModuleClass,
            ShipModuleCoreInternal, ShipModuleExternal, ShipModuleHardpoint,
            ShipModuleOptionalInternal, ShipModuleSize,
            ShipModuleUtilityMount::{
                self, CausticSinkLauncher, HeatsinkLauncher, PlasmaPointDefence,
            },
        },
        ship_type::ShipType,
    };
    use serde::{
        Deserialize, Deserializer, Serializer,
        de::{
            Error, IntoDeserializer,
            value::{Error as ValueError, StrDeserializer},
        },
    };

    fn deserialize_enum_from_str<'de, T, E>(s: &str, ctx: &str) -> Result<T, E>
    where
        T: Deserialize<'de>,
        E: Error,
    {
        let deserializer: StrDeserializer<ValueError> = s.into_deserializer();
        T::deserialize(deserializer).map_err(|e| E::custom(format!("Error parsing {ctx}: {e}")))
    }

    fn core_internal<'de, D: Deserializer<'de>>(
        name: &str,
    ) -> Result<ShipModuleCoreInternal, D::Error> {
        deserialize_enum_from_str::<ShipModuleCoreInternal, D::Error>(name, "Core Internal Name")
    }

    fn optional_internal<'de, D: Deserializer<'de>>(
        name: &str,
    ) -> Result<ShipModuleOptionalInternal, D::Error> {
        deserialize_enum_from_str::<ShipModuleOptionalInternal, D::Error>(
            name,
            "Optional Internal Name",
        )
    }

    fn utility_mount<'de, D: Deserializer<'de>>(
        name: &str,
    ) -> Result<ShipModuleUtilityMount, D::Error> {
        deserialize_enum_from_str::<ShipModuleUtilityMount, D::Error>(name, "Utility Mount Name")
    }

    fn hardpoint<'de, D: Deserializer<'de>>(name: &str) -> Result<ShipModuleHardpoint, D::Error> {
        deserialize_enum_from_str::<ShipModuleHardpoint, D::Error>(name, "Hardpoint Name")
    }

    fn external<'de, D: Deserializer<'de>>(name: &str) -> Result<ShipModuleExternal, D::Error> {
        deserialize_enum_from_str::<ShipModuleExternal, D::Error>(name, "External Name")
    }

    fn ship_type<'de, D: Deserializer<'de>>(name: &str) -> Result<ShipType, D::Error> {
        deserialize_enum_from_str::<ShipType, D::Error>(name, "External Name")
    }

    fn ship_armour_grade<'de, D: Deserializer<'de>>(
        name: &str,
    ) -> Result<ShipArmourGrade, D::Error> {
        deserialize_enum_from_str::<ShipArmourGrade, D::Error>(name, "External Name")
    }

    fn module_size<'de, D: Deserializer<'de>>(name: &str) -> Result<ShipModuleSize, D::Error> {
        deserialize_enum_from_str::<ShipModuleSize, D::Error>(name, "Module size")
    }

    fn module_class<'de, D: Deserializer<'de>>(name: &str) -> Result<ShipModuleClass, D::Error> {
        deserialize_enum_from_str::<ShipModuleClass, D::Error>(name, "Module Class")
    }

    fn hardpoint_conn<'de, D: Deserializer<'de>>(
        name: &str,
    ) -> Result<HardpointConnection, D::Error> {
        deserialize_enum_from_str::<HardpointConnection, D::Error>(name, "Hardpoint Connection")
    }

    fn hardpoint_size<'de, D: Deserializer<'de>>(name: &str) -> Result<HardpointSize, D::Error> {
        deserialize_enum_from_str::<HardpointSize, D::Error>(name, "Hardpoint size")
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ShipModule, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let mut parts = s.split('_').collect::<Vec<_>>();

        if parts.is_empty() {
            return Err(Error::custom(format!("Did not find a prefix on '{s:?}'")));
        }

        if parts[0].starts_with('$') {
            // remove the $ prefix
            parts[0] = &parts[0][1..];

            // remove the trailing ;
            let last = parts.len() - 1;
            parts[last] = parts[last].trim_end_matches(';');

            // remove _name; part
            if parts[last] == "name" {
                parts.pop();
            }
        }

        match parts[0] {
            "Null" => Ok(ShipModule::Null),
            "int" => {
                // handle modules that do not have pattern $int_name_class_size_name;
                match parts[1] {
                    "dockingcomputer" => {
                        // handle $int_dockingcomputer_advanced_name;
                        let module = if parts.len() > 2 && parts[2] == "advanced" {
                            ShipModuleOptionalInternal::DockingComputerAdvanced
                        } else {
                            ShipModuleOptionalInternal::DockingComputer
                        };
                        return Ok(ShipModule::OptionalInternal(
                            module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "planetapproachsuite" => {
                        let module = if parts.len() > 2 && parts[2] == "advanced" {
                            ShipModuleOptionalInternal::PlanetApproachSuiteAdvanced
                        } else {
                            ShipModuleOptionalInternal::PlanetApproachSuite
                        };
                        return Ok(ShipModule::OptionalInternal(
                            module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "colonisation" | "codexscanner" | "stellarbodydiscoveryscanner" => {
                        return Ok(ShipModule::CoreInternal(
                            core_internal::<D>(parts[1])?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "supercruiseassist" => {
                        return Ok(ShipModule::OptionalInternal(
                            optional_internal::<D>(parts[1])?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "mkii" if parts.len() > 4 => {
                        // int_mkii_passengercabin_size2_class1_name
                        return Ok(ShipModule::OptionalInternal(
                            ShipModuleOptionalInternal::PassengerCabinMkII,
                            module_size::<D>(parts[3])?,
                            module_class::<D>(parts[4])?,
                        ));
                    }
                    "mkiiagileboost" if parts.len() > 4 => {
                        // handle "$int_mkiiagileboost_engine_size5_class5_name;"
                        return Ok(ShipModule::CoreInternal(
                            ShipModuleCoreInternal::EngineMkIIAgileBoost,
                            module_size::<D>(parts[3])?,
                            module_class::<D>(parts[4])?,
                        ));
                    }
                    "dronecontrol" | "multidronecontrol" if parts.len() > 4 => {
                        // handle "$int_dronecontrol_fueltransfer_size1_class5_name;"
                        let name = parts[1].to_string() + "_" + parts[2];
                        return Ok(ShipModule::OptionalInternal(
                            optional_internal::<D>(name.as_str())?,
                            module_size::<D>(parts[3])?,
                            module_class::<D>(parts[4])?,
                        ));
                    }
                    "dronecontrol" if parts.len() > 2 => {
                        // int_dronecontrol_unkvesselresearch
                        let name = parts[1].to_string() + "_" + parts[2];
                        return Ok(ShipModule::OptionalInternal(
                            optional_internal::<D>(name.as_str())?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "guardianfsdbooster" => {
                        if parts.len() > 2 {
                            // int_guardianfsdbooster_size5
                            return Ok(ShipModule::OptionalInternal(
                                optional_internal::<D>(parts[1])?,
                                module_size::<D>(parts[2])?,
                                ShipModuleClass::None,
                            ));
                        }
                    }
                    "guardianpowerplant" => {
                        // int_guardianpowerplant_size2
                        if parts.len() > 2 {
                            // int_guardianfsdbooster_size5
                            // int_guardianpowerplant_size2
                            return Ok(ShipModule::CoreInternal(
                                core_internal::<D>(parts[1])?,
                                module_size::<D>(parts[2])?,
                                ShipModuleClass::None,
                            ));
                        }
                    }
                    "detailedsurfacescanner" if parts.len() > 2 => {
                        // $int_detailedsurfacescanner_tiny_name;
                        //
                        // ignore tiny for now

                        return Ok(ShipModule::OptionalInternal(
                            optional_internal::<D>(parts[1])?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "hyperdrive" if parts.len() > 4 => {
                        // int_hyperdrive_size5_class5
                        if parts[2] == "overcharge" {
                            // int_hyperdrive_overcharge_size5_class5
                            return Ok(ShipModule::CoreInternal(
                                ShipModuleCoreInternal::HyperdriveOvercharge,
                                module_size::<D>(parts[3])?,
                                module_class::<D>(parts[4])?,
                            ));
                        } else {
                            return Ok(ShipModule::CoreInternal(
                                ShipModuleCoreInternal::Hyperdrive,
                                module_size::<D>(parts[2])?,
                                module_class::<D>(parts[3])?,
                            ));
                        }
                    }
                    _ => { /* continue */ }
                }

                // handle pattern int_name_class_size

                if parts.len() < 4 {
                    return Err(Error::custom(format!(
                        "Error parsing {s}: expected four parts"
                    )));
                }

                let module_size: ShipModuleSize = module_size::<D>(parts[2])?;
                let module_class: ShipModuleClass = module_class::<D>(parts[3])?;

                if let Ok(mut internal_module) = core_internal::<D>(parts[1]) {
                    if parts.len() > 4 {
                        match internal_module {
                            // handle $int_engine_size7_class5_gravityoptimised_mkii_name;
                            ShipModuleCoreInternal::Engine if parts[4] == "gravityoptimised" => {
                                internal_module =
                                    ShipModuleCoreInternal::EngineGravityOptimisedMkII;
                            }
                            _ => { /* continue  */ }
                        }
                    }
                    return Ok(ShipModule::CoreInternal(
                        internal_module,
                        module_size,
                        module_class,
                    ));
                }

                if let Ok(mut internal_module) = optional_internal::<D>(parts[1]) {
                    // handle prismatic and bi-weave shields
                    if parts.len() > 4 {
                        match internal_module {
                            ShipModuleOptionalInternal::ShieldGenerator if parts[4] == "strong" => {
                                internal_module =
                                    ShipModuleOptionalInternal::PrismaticShieldGenerator;
                            }
                            ShipModuleOptionalInternal::ShieldGenerator if parts[4] == "fast" => {
                                internal_module =
                                    ShipModuleOptionalInternal::BiWeaveShieldGenerator;
                            }
                            _ => { /* continue */ }
                        }
                    }
                    return Ok(ShipModule::OptionalInternal(
                        internal_module,
                        module_size,
                        module_class,
                    ));
                }

                Err(Error::custom(format!(
                    "Found unknown internal module '{}'",
                    parts[1]
                )))
            }
            "hpt" => {
                // matches patters other than $hpt_name_connection_size_name; first
                match parts[1] {
                    "mining" | "guardian" if parts.len() > 4 => {
                        // handle $hpt_mining_subsurfdispmisle_fixed_medium_name;
                        // or $hpt_guardian_plasmalauncher_fixed_medium_name;
                        let name = parts[1].to_string() + "_" + parts[2];
                        return Ok(ShipModule::Hardpoint(
                            hardpoint::<D>(name.as_str())?,
                            hardpoint_conn::<D>(parts[3])?,
                            hardpoint_size::<D>(parts[4])?,
                        ));
                    }
                    "xenoscanner" | "xenoscannermk2" if parts.len() > 3 => {
                        // hpt_xenoscanner_basic_tiny | hpt_xenoscannermk2_basic_tiny
                        // hpt_xenoscanner_advanced_tiny_name
                        //
                        // for now we ignore the tiny-part
                        return Ok(ShipModule::UtilityMount(
                            utility_mount::<D>((parts[1].to_string() + "_" + parts[2]).as_str())?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "electroniccountermeasure" | "chafflauncher" if parts.len() > 2 => {
                        // hpt_electroniccountermeasure_tiny
                        //
                        // for now we ignore the tiny-part
                        return Ok(ShipModule::UtilityMount(
                            utility_mount::<D>(parts[1])?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "shipdatalinkscanner" => {
                        // hpt_shipdatalinkscanner
                        return Ok(ShipModule::UtilityMount(
                            utility_mount::<D>(parts[1])?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "antiunknownshutdown" if parts.len() > 2 => {
                        // hpt_antiunknownshutdown_tiny_v2
                        // hpt_antiunknownshutdown_tiny
                        //
                        // for now we ignore the tiny-part

                        let name = if parts.len() == 4 && parts[3] == "v2" {
                            parts[1].to_string() + parts[3]
                        } else {
                            parts[1].to_string()
                        };

                        return Ok(ShipModule::UtilityMount(
                            utility_mount::<D>(name.as_str())?,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "heatsinklauncher" => {
                        // hpt_heatsinklauncher_turret_tiny, ignore turret_tiny
                        return Ok(ShipModule::UtilityMount(
                            HeatsinkLauncher,
                            ShipModuleSize::Size0,
                            ShipModuleClass::Class1,
                        ));
                    }
                    "plasmapointdefence" => {
                        // hpt_heatsinklauncher_turret_tiny, ignore turret_tiny
                        return Ok(ShipModule::UtilityMount(
                            PlasmaPointDefence,
                            ShipModuleSize::Size0,
                            ShipModuleClass::Class1,
                        ));
                    }
                    "causticsinklauncher" => {
                        // hpt_heatsinklauncher_turret_tiny, ignore turret_tiny
                        return Ok(ShipModule::UtilityMount(
                            CausticSinkLauncher,
                            ShipModuleSize::Size0,
                            ShipModuleClass::Class1,
                        ));
                    }
                    _ => { /* continue */ }
                }

                // optional internal with hpt prefix
                if let Ok(module) = utility_mount::<D>(parts[1])
                    && matches!(
                        module,
                        ShipModuleUtilityMount::ShieldBooster
                            | ShipModuleUtilityMount::CloudScanner
                            | ShipModuleUtilityMount::MRAScanner
                            | ShipModuleUtilityMount::CrimeScanner
                            | ShipModuleUtilityMount::CargoScanner
                    )
                    && parts.len() > 3
                {
                    // hpt_cloudscanner_size0_class4 or hpt_mrascanner_size0_class5
                    let module_size: ShipModuleSize = module_size::<D>(parts[2])?;
                    let module_class: ShipModuleClass = module_class::<D>(parts[3])?;
                    return Ok(ShipModule::UtilityMount(module, module_size, module_class));
                }

                if parts.len() < 4 {
                    return Err(Error::custom(format!(
                        "Error parsing {s}: expected four parts"
                    )));
                }

                // $hpt_name_connection_size_name;

                let mut module = hardpoint::<D>(parts[1])?;
                let module_conn: HardpointConnection = hardpoint_conn::<D>(parts[2])?;
                let module_size: HardpointSize = hardpoint_size::<D>(parts[3])?;

                if parts.len() > 4 {
                    match module {
                        // check for range in $hpt_slugshot_fixed_large_range_name;
                        ShipModuleHardpoint::Slugshot if parts[4] == "range" => {
                            module = ShipModuleHardpoint::SlugshotRange;
                        }
                        // check for burst in $hpt_railgun_fixed_medium_burst_name;
                        ShipModuleHardpoint::RailGun if parts[4] == "burst" => {
                            module = ShipModuleHardpoint::TheHammer;
                        }
                        // check for scatter in $hpt_pulselaserburst_fixed_small_scatter_name;
                        ShipModuleHardpoint::PulseLaserBurst if parts[4] == "scatter" => {
                            module = ShipModuleHardpoint::Cytoscrambler;
                        }
                        // check for v2 in $hpt_atmulticannon_turret_medium_v2_name;
                        ShipModuleHardpoint::ATMultiCannon if parts[4] == "v2" => {
                            module = ShipModuleHardpoint::ATMultiCannonV2;
                        }
                        // check for heat in hpt_beamlaser_fixed_small_heat
                        ShipModuleHardpoint::BeamLaser if parts[4] == "heat" => {
                            module = ShipModuleHardpoint::Retributor;
                        }
                        // check for lasso in hpt_dumbfiremissilerack_fixed_medium_lasso_name
                        ShipModuleHardpoint::DumbFireMissileRack if parts[4] == "lasso" => {
                            module = ShipModuleHardpoint::ContainmentMissile;
                        }
                        // check for advanced in hpt_plasmaaccelerator_fixed_large_advanced_name
                        ShipModuleHardpoint::PlasmaAccelerator if parts[4] == "advanced" => {
                            module = ShipModuleHardpoint::AdvandedPlasmaAccelerator;
                        }
                        _ => { /* continue */ }
                    }
                }

                Ok(ShipModule::Hardpoint(module, module_conn, module_size))
            }
            "ext" => {
                if parts.len() < 3 {
                    return Err(Error::custom(format!(
                        "Error parsing {s}: expected at least three parts"
                    )));
                }

                // $ext_drive_class3_cob_name;
                // $ext_drive_krait_light_name;
                // ext_drive_indfighter
                let module: ShipModuleExternal = external::<D>(parts[1])?;
                if let Ok(module_class) = module_class::<D>(parts[2]) {
                    let rest = if parts.len() > 3 {
                        parts[3..].join("_")
                    } else {
                        String::from("")
                    };
                    return Ok(ShipModule::External(module, module_class, rest.into()));
                }
                Ok(ShipModule::External(
                    module,
                    ShipModuleClass::None,
                    parts[2..].join("_").into(),
                ))
            }
            "modularcargobaydoor" | "$modularcargobaydoor" => Ok(ShipModule::ModularCargoBayDoor),
            "modularcargobaydoorfdl" | "$modularcargobaydoorfdl" => {
                Ok(ShipModule::ModularCargoBayDoorFDL)
            }
            "paintjob" => {
                if parts.len() < 3 {
                    return Err(Error::custom(format!(
                        "Error parsing {s}: expected at least three parts"
                    )));
                }
                // paintjob_asp_halloween01_01
                // paintjob_smallcombat01_nx_03_01
                let (i, ship_type) =
                    ship_type::<D>(parts[1])
                        .map(|s| (2, s))
                        .or_else(|_: D::Error| {
                            ship_type::<D>(parts[1..3].join("_").as_str()).map(|s| (3, s))
                        })?;
                let paintjob_name = parts[i..].join("_");
                Ok(ShipModule::Paintjob(ship_type, paintjob_name.into()))
            }
            "voicepack" if parts.len() > 1 => Ok(ShipModule::VoicePack(parts[1].into())),
            "decal" if parts.len() > 1 => Ok(ShipModule::Decal(parts[1..].join("_").into())),
            "weaponcustomisation" if parts.len() > 1 => {
                Ok(ShipModule::WeaponCustomisation(parts[1..].join("_").into()))
            }
            "enginecustomisation" if parts.len() > 1 => {
                Ok(ShipModule::EngineCustomisation(parts[1..].join("_").into()))
            }
            "nameplate" if parts.len() > 1 => {
                Ok(ShipModule::NamePlate(parts[1..].join("_").into()))
            }
            "bobble" if parts.len() > 1 => Ok(ShipModule::Bobble(parts[1..].join("_").into())),
            "string" if parts.len() > 1 => Ok(ShipModule::String(parts[1..].join("_").into())),
            unknown => {
                // check on cockpit module that follows $shipname_cockpit_name; pattern
                if let Some(index) = s.rfind("cockpit") {
                    // we are working on s, not on parts, so a possible $ is not removed
                    let name = s[0..index - 1].trim_matches('$');
                    return Ok(ShipModule::Cockpit(ship_type::<D>(name)?));
                }

                // check on ship_armour_gradeN pattern, also $ship_ext_armour_gradeN;
                // ship_name can have more than one part, i.e. explorer_nx
                if let Some(i) = parts.iter().position(|p| *p == "armour") {
                    let (ship_name, grade_name) = (parts[0..i].join("_"), parts[i + 1]);
                    let ship_type = ship_type::<D>(ship_name.as_str())?;
                    let grade = ship_armour_grade::<D>(grade_name)?;
                    return Ok(ShipModule::Armour(ship_type, grade));
                }

                if let Some(i) = parts
                    .iter()
                    .position(|p| p.starts_with("shipkit"))
                    .or_else(|| parts.iter().position(|p| p.starts_with("thargoidreward")))
                {
                    let ship_name = parts[0..i].join("_");
                    let shipkit_name = parts[i..].join("_");
                    let ship_type = ship_type::<D>(ship_name.as_str())?;
                    return Ok(ShipModule::ShipKit(ship_type, shipkit_name.into()));
                }

                Err(Error::custom(format!(
                    "Found unknown prefix '{unknown}' on module name"
                )))
            }
        }
    }

    pub fn serialize<S>(module: &ShipModule, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(module.to_string().as_str())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Display)]
#[serde(rename_all = "lowercase")]
pub enum ShipArmourGrade {
    #[strum(to_string = "Grade 1")]
    Grade1,
    #[strum(to_string = "Grade 2")]
    Grade2,
    #[strum(to_string = "Grade 3")]
    Grade3,
    #[strum(to_string = "Grade 4")]
    Grade4,
    #[strum(to_string = "Grade 5")]
    Grade5,
    Reactive,
}

#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq)]
pub enum ShipModuleSlot {
    Armour,
    Bobble01,
    Bobble02,
    Bobble03,
    Bobble04,
    Bobble05,
    Bobble06,
    Bobble07,
    Bobble08,
    Bobble09,
    Bobble10,
    Cargo01,
    Cargo02,
    CargoHatch,
    CodexScanner,
    ColonisationSuite,
    DataLinkScanner,
    Decal1,
    Decal2,
    Decal3,
    DiscoveryScanner,
    EngineColour,
    FighterBay01,
    FrameShiftDrive,
    FuelTank,
    HugeHardpoint1,
    HugeHardpoint2,
    LargeHardpoint1,
    LargeHardpoint2,
    LargeHardpoint3,
    LargeHardpoint4,
    LargeMiningHardpoint1,
    LargeMiningHardpoint2,
    LifeSupport,
    LimpetController01,
    MainEngines,
    MediumHardpoint1,
    MediumHardpoint2,
    MediumHardpoint3,
    MediumHardpoint4,
    MediumHardpoint5,
    MediumHardpoint6,
    MediumMiningHardpoint1,
    MediumMiningHardpoint2,
    Military01,
    Military02,
    Military03,
    PaintJob,
    Passenger01,
    Passenger02,
    Passenger03,
    PlanetaryApproachSuite,
    PowerDistributor,
    PowerPlant,
    Radar,
    ShipCockpit,
    ShipName0,
    ShipName1,
    ShipID0,
    ShipID1,
    ShipKitBumper,
    ShipKitSpoiler,
    ShipKitWings,
    ShipKitTail,
    SmallHardpoint1,
    SmallHardpoint2,
    SmallHardpoint3,
    SmallHardpoint4,
    SmallHardpoint5,
    SmallHardpoint6,
    SmallMiningHardpoint1,
    SmallMiningHardpoint2,
    StringLights,
    TinyHardpoint1,
    TinyHardpoint2,
    TinyHardpoint3,
    TinyHardpoint4,
    TinyHardpoint5,
    TinyHardpoint6,
    TinyHardpoint7,
    TinyHardpoint8,
    VesselVoice,
    WeaponColour,
    #[serde(rename = "Slot00_Size8")]
    Slot00Size8,
    #[serde(rename = "Slot01_Size2")]
    Slot01Size2,
    #[serde(rename = "Slot01_Size3")]
    Slot01Size3,
    #[serde(rename = "Slot01_Size4")]
    Slot01Size4,
    #[serde(rename = "Slot01_Size5")]
    Slot01Size5,
    #[serde(rename = "Slot01_Size6")]
    Slot01Size6,
    #[serde(rename = "Slot01_Size7")]
    Slot01Size7,
    #[serde(rename = "Slot01_Size8")]
    Slot01Size8,
    #[serde(rename = "Slot02_Size2")]
    Slot02Size2,
    #[serde(rename = "Slot02_Size3")]
    Slot02Size3,
    #[serde(rename = "Slot02_Size4")]
    Slot02Size4,
    #[serde(rename = "Slot02_Size5")]
    Slot02Size5,
    #[serde(rename = "Slot02_Size6")]
    Slot02Size6,
    #[serde(rename = "Slot02_Size7")]
    Slot02Size7,
    #[serde(rename = "Slot02_Size8")]
    Slot02Size8,
    #[serde(rename = "Slot03_Size1")]
    Slot03Size1,
    #[serde(rename = "Slot03_Size2")]
    Slot03Size2,
    #[serde(rename = "Slot03_Size3")]
    Slot03Size3,
    #[serde(rename = "Slot03_Size4")]
    Slot03Size4,
    #[serde(rename = "Slot03_Size5")]
    Slot03Size5,
    #[serde(rename = "Slot03_Size6")]
    Slot03Size6,
    #[serde(rename = "Slot03_Size7")]
    Slot03Size7,
    #[serde(rename = "Slot04_Size1")]
    Slot04Size1,
    #[serde(rename = "Slot04_Size2")]
    Slot04Size2,
    #[serde(rename = "Slot04_Size3")]
    Slot04Size3,
    #[serde(rename = "Slot04_Size4")]
    Slot04Size4,
    #[serde(rename = "Slot04_Size5")]
    Slot04Size5,
    #[serde(rename = "Slot04_Size6")]
    Slot04Size6,
    #[serde(rename = "Slot05_Size1")]
    Slot05Size1,
    #[serde(rename = "Slot05_Size2")]
    Slot05Size2,
    #[serde(rename = "Slot05_Size3")]
    Slot05Size3,
    #[serde(rename = "Slot05_Size4")]
    Slot05Size4,
    #[serde(rename = "Slot05_Size5")]
    Slot05Size5,
    #[serde(rename = "Slot05_Size6")]
    Slot05Size6,
    #[serde(rename = "Slot06_Size1")]
    Slot06Size1,
    #[serde(rename = "Slot06_Size2")]
    Slot06Size2,
    #[serde(rename = "Slot06_Size3")]
    Slot06Size3,
    #[serde(rename = "Slot06_Size4")]
    Slot06Size4,
    #[serde(rename = "Slot06_Size5")]
    Slot06Size5,
    #[serde(rename = "Slot06_Size6")]
    Slot06Size6,
    #[serde(rename = "Slot07_Size1")]
    Slot07Size1,
    #[serde(rename = "Slot07_Size2")]
    Slot07Size2,
    #[serde(rename = "Slot07_Size3")]
    Slot07Size3,
    #[serde(rename = "Slot07_Size4")]
    Slot07Size4,
    #[serde(rename = "Slot07_Size5")]
    Slot07Size5,
    #[serde(rename = "Slot08_Size1")]
    Slot08Size1,
    #[serde(rename = "Slot08_Size2")]
    Slot08Size2,
    #[serde(rename = "Slot08_Size3")]
    Slot08Size3,
    #[serde(rename = "Slot08_Size4")]
    Slot08Size4,
    #[serde(rename = "Slot09_Size1")]
    Slot09Size1,
    #[serde(rename = "Slot09_Size2")]
    Slot09Size2,
    #[serde(rename = "Slot09_Size3")]
    Slot09Size3,
    #[serde(rename = "Slot09_Size4")]
    Slot09Size4,
    #[serde(rename = "Slot10_Size1")]
    Slot10Size1,
    #[serde(rename = "Slot10_Size2")]
    Slot10Size2,
    #[serde(rename = "Slot10_Size3")]
    Slot10Size3,
    #[serde(rename = "Slot10_Size4")]
    Slot10Size4,
    #[serde(rename = "Slot11_Size1")]
    Slot11Size1,
    #[serde(rename = "Slot11_Size2")]
    Slot11Size2,
    #[serde(rename = "Slot11_Size3")]
    Slot11Size3,
    #[serde(rename = "Slot12_Size1")]
    Slot12Size1,
    #[serde(rename = "Slot13_Size1")]
    Slot13Size1,
    #[serde(rename = "Slot13_Size2")]
    Slot13Size2,
    #[serde(rename = "Slot14_Size1")]
    Slot14Size1,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleSize {
    None,
    #[strum(to_string = "0")]
    Size0,
    #[strum(to_string = "1")]
    Size1,
    #[strum(to_string = "2")]
    Size2,
    #[strum(to_string = "3")]
    Size3,
    #[strum(to_string = "4")]
    Size4,
    #[strum(to_string = "5")]
    Size5,
    #[strum(to_string = "6")]
    Size6,
    #[strum(to_string = "7")]
    Size7,
    #[strum(to_string = "8")]
    Size8,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleClass {
    None,
    #[strum(to_string = "F")]
    Class0,
    #[strum(to_string = "E")]
    Class1,
    #[strum(to_string = "D")]
    Class2,
    #[strum(to_string = "C")]
    Class3,
    #[strum(to_string = "B")]
    Class4,
    #[strum(to_string = "A")]
    Class5,
    #[strum(to_string = "Class 6")]
    Class6,
    #[strum(to_string = "Class 7")]
    Class7,
}

impl ShipModuleClass {
    pub fn passenger_cabin_abbr(&self) -> Option<&str> {
        match self {
            ShipModuleClass::Class1 => Some("EC"),
            ShipModuleClass::Class2 => Some("BC"),
            ShipModuleClass::Class3 => Some("FC"),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleCoreInternal {
    #[strum(to_string = "Discovery Scanner")]
    StellarBodyDiscoveryScanner,
    #[strum(to_string = "Colonisation Suite")]
    Colonisation,
    #[strum(to_string = "Codex Scanner")]
    CodexScanner,
    #[strum(to_string = "Thrusters")]
    Engine,
    #[strum(to_string = "Thrusters")]
    EngineGravityOptimisedMkII,
    #[strum(to_string = "Thrusters")]
    EngineMkIIAgileBoost,
    #[strum(to_string = "FSD")]
    Hyperdrive,
    #[strum(to_string = "FSD (SCO)")]
    HyperdriveOvercharge,
    #[strum(to_string = "FSD (SCO)")]
    FSDSCOOverchargeBoosterMkII,
    #[strum(to_string = "Guardian Power Plant")]
    GuardianPowerPlant,
    #[strum(to_string = "Life Support")]
    LifeSupport,
    #[strum(to_string = "Power Distributor")]
    PowerDistributor,
    #[strum(to_string = "Power Plant")]
    PowerPlant,
    #[strum(to_string = "Sensors")]
    Sensors,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleOptionalInternal {
    #[strum(to_string = "Bi-Weave Shield")]
    BiWeaveShieldGenerator,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    BuggyBay,
    #[strum(to_string = "Cargo Rack")]
    CargoRack,
    #[strum(to_string = "Anti-Corrosion Cargo Rack")]
    CorrosionProofCargoRack,
    #[strum(to_string = "Surface Scanner")]
    DetailedSurfaceScanner,
    #[strum(to_string = "Docking Computer")]
    DockingComputer,
    #[strum(to_string = "Docking Computer")]
    DockingComputerAdvanced,
    #[strum(to_string = "Fuel Transfer")]
    #[serde(alias = "dronecontrol_fueltransfer")]
    DroneControlFuelTransfer,
    #[strum(to_string = "Collector")]
    #[serde(alias = "dronecontrol_collection")]
    DroneControlCollection,
    #[strum(to_string = "Decontamination")]
    #[serde(alias = "dronecontrol_decontamination")]
    DroneControlDecontamination,
    #[strum(to_string = "Prospector")]
    #[serde(alias = "dronecontrol_prospector")]
    DroneControlProspector,
    #[strum(to_string = "Recon")]
    #[serde(alias = "dronecontrol_recon")]
    DroneControlRecon,
    #[strum(to_string = "Repair")]
    #[serde(alias = "dronecontrol_repair")]
    DroneControlRepair,
    #[strum(to_string = "Research")]
    #[serde(alias = "dronecontrol_unkvesselresearch")]
    DroneControlResearch,
    #[strum(to_string = "Hatch Breaker")]
    #[serde(alias = "dronecontrol_resourcesiphon")]
    DroneControlResourceSiphon,
    #[strum(to_string = "Experimental Weapon Stabiliser")]
    ExpModuleStabiliser,
    #[strum(to_string = "Vessel Hangar")]
    FighterBay,
    #[strum(to_string = "Vessel Hangar Mk II")]
    FighterBayMk2,
    #[strum(to_string = "FSD Interdictor")]
    FSDInterdictor,
    #[strum(to_string = "Fuel Scoop")]
    FuelScoop,
    #[strum(to_string = "Fuel Tank")]
    FuelTank,
    #[strum(to_string = "Guardian FSD Booster")]
    GuardianFSDBooster,
    #[strum(to_string = "Guardian Hull Reinforcement")]
    GuardianHullReinforcement,
    #[strum(to_string = "Guardian Module Reinforcement")]
    GuardianModuleReinforcement,
    #[strum(to_string = "Guardian Shield Reinforcement")]
    GuardianShieldReinforcement,
    #[strum(to_string = "Hull Reinforcement")]
    HullReinforcement,
    #[strum(to_string = "Mk II Cargo Rack")]
    LargeCargoRack,
    #[strum(to_string = "Module Reinforcement")]
    ModuleReinforcement,
    #[strum(to_string = "Rescue Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_rescue")]
    MultiDroneControlRescue,
    #[strum(to_string = "Mining Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_mining")]
    MultiDroneControlMining,
    #[strum(to_string = "Mk II Mining Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_miningv2")]
    MultiDroneControlMiningV2,
    #[strum(to_string = "Operations Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_operations")]
    MultiDroneControlOperations,
    #[strum(to_string = "Universal Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_universal")]
    MultiDroneControlUniversal,
    #[strum(to_string = "Xeno Multi-Limpet Controller")]
    #[serde(alias = "multidronecontrol_xeno")]
    MultiDroneControlXeno,
    #[strum(to_string = "Passenger Cabin")]
    PassengerCabin,
    #[strum(to_string = "Mk II Passenger Cabin")]
    PassengerCabinMkII,
    #[strum(to_string = "Adv. Planetary Approach Suite")]
    PlanetApproachSuiteAdvanced,
    #[strum(to_string = "Planetary Approach Suite")]
    PlanetApproachSuite,
    #[strum(to_string = "Prismatic Shield")]
    PrismaticShieldGenerator,
    #[strum(to_string = "Refinery")]
    Refinery,
    #[strum(to_string = "AFM Unit")]
    Repairer,
    #[strum(to_string = "Shield Cell Bank")]
    ShieldCellBank,
    #[strum(to_string = "Shield Generator")]
    ShieldGenerator,
    #[strum(to_string = "Supercruise Assist")]
    SupercruiseAssist,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleUtilityMount {
    #[strum(to_string = "Caustic Sink Launcher")]
    CausticSinkLauncher,
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(alias = "antiunknownshutdown")]
    AntiUnknownShutdown,
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(alias = "antiunknownshutdown_v2")]
    AntiUnknownShutdownV2,
    #[strum(to_string = "Manifest Scanner")]
    CargoScanner,
    #[strum(to_string = "Chaff")]
    ChaffLauncher,
    #[strum(to_string = "Wake Scanner")]
    CloudScanner,
    #[strum(to_string = "K-Warrant Scanner")]
    CrimeScanner,
    #[strum(to_string = "ECM")]
    ElectronicCounterMeasure,
    #[strum(to_string = "Heatsink")]
    HeatsinkLauncher,
    #[strum(to_string = "Pulse Wave")]
    MRAScanner,
    #[strum(to_string = "Point Defence Turret")]
    PlasmaPointDefence,
    #[strum(to_string = "Shield Booster")]
    ShieldBooster,
    #[strum(to_string = "DataLinkScanner")]
    ShipDataLinkScanner,
    #[strum(to_string = "P. Wave Xeno Scanner")]
    #[serde(alias = "xenoscanner_advanced")]
    XenoScannerAdvanced,
    #[strum(to_string = "Xeno Scanner")]
    #[serde(alias = "xenoscanner_basic")]
    XenoScannerBasic,
    #[strum(to_string = "Enhanced Xeno Scanner")]
    #[serde(alias = "xenoscannermk2_basic")]
    XenoScannerMkII,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleHardpoint {
    #[strum(to_string = "Advanced Accelerator")]
    AdvandedPlasmaAccelerator,
    #[strum(to_string = "Torpedo Pylon")]
    #[serde(alias = "advancedtorppylon")]
    AdvancedTorpedoPylon,
    #[strum(to_string = "Enhanced AX Missile Rack")]
    ATDumbFireMissile,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    ATMultiCannon,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    ATMultiCannonV2,
    #[strum(to_string = "Nanite Torpedo Pylon")]
    ATVentDisruptorPylon,
    #[strum(to_string = "AX Multi-Cannon")]
    AXMultiCannon,
    #[strum(to_string = "Seeker Missile Rack")]
    BasicMissileRack,
    #[strum(to_string = "Beam Laser")]
    BeamLaser,
    #[strum(to_string = "Cannon")]
    Cannon,
    #[strum(to_string = "Containment Missile")]
    ContainmentMissile,
    #[strum(to_string = "Cytoscrambler")]
    Cytoscrambler,
    #[strum(to_string = "Pack-Hound")]
    DrunkMissileRack,
    #[strum(to_string = "Missile Rack")]
    DumbFireMissileRack,
    #[strum(to_string = "Enhanced AX Multi-Cannon")]
    EnhancedAXMultiCannon,
    #[strum(to_string = "Remote Flak")]
    FlakMortar,
    #[strum(to_string = "Remote Flechette")]
    FlechetteLauncher,
    #[strum(to_string = "Gauss Cannon")]
    GaussCannon,
    #[strum(to_string = "Plasma Charger")]
    #[serde(alias = "guardian_plasmalauncher")]
    GuardianPlasmaLauncher,
    #[strum(to_string = "Shard Cannon")]
    #[serde(alias = "guardian_shardcannon")]
    GuardianShardCannon,
    #[strum(to_string = "Gauss Cannon")]
    #[serde(alias = "guardian_gausscannon")]
    GuardianGaussCannon,
    #[strum(to_string = "Mine Launcher")]
    MineLauncher,
    #[strum(to_string = "Mining Volley Repeater")]
    MiningToolV2,
    #[strum(to_string = "Mining Laser")]
    MiningLaser,
    #[strum(to_string = "Mk II Plasma Shock Accelerator")]
    MkIIPlasmaShockAutoCannon,
    #[strum(to_string = "Multi-Cannon")]
    MultiCannon,
    #[strum(to_string = "Plasma Acc")]
    PlasmaAccelerator,
    #[strum(to_string = "Pulse Laser")]
    PulseLaser,
    #[strum(to_string = "Burst Laser")]
    PulseLaserBurst,
    #[strum(to_string = "Retributor")]
    Retributor,
    #[strum(to_string = "Rail Gun")]
    RailGun,
    #[strum(to_string = "Remote Flak")]
    RemoteFlak,
    #[strum(to_string = "Shard Cannon")]
    ShardCannon,
    #[strum(to_string = "Frag Cannon")]
    Slugshot,
    #[strum(to_string = "The Pacifier")]
    SlugshotRange,
    #[strum(to_string = "The Hammer")]
    TheHammer,
    #[strum(to_string = "Displacement Missile")]
    #[serde(alias = "mining_subsurfdispmisle")]
    MiningSubsurfaceDisplacementMissile,
    #[strum(to_string = "Seismic Charge")]
    #[serde(alias = "mining_seismchrgwarhd")]
    MiningSeismicChargeWarhead,
    #[strum(to_string = "Abrasion Blaster")]
    #[serde(alias = "mining_abrblstr")]
    MiningAbrationBlaster,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HardpointConnection {
    Fixed,
    Gimbal,
    Turret,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HardpointSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleExternal {
    Drive,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ShipModule {
    Armour(ShipType, ShipArmourGrade),
    Bobble(EDString), // bobble name
    Cockpit(ShipType),
    Decal(EDString),                                         // decal name
    EngineCustomisation(EDString),                           // customisation name
    External(ShipModuleExternal, ShipModuleClass, EDString), // details of module, e.g. cob or d_xl
    Hardpoint(ShipModuleHardpoint, HardpointConnection, HardpointSize),
    CoreInternal(ShipModuleCoreInternal, ShipModuleSize, ShipModuleClass),
    OptionalInternal(ShipModuleOptionalInternal, ShipModuleSize, ShipModuleClass),
    UtilityMount(ShipModuleUtilityMount, ShipModuleSize, ShipModuleClass),
    ModularCargoBayDoor,
    ModularCargoBayDoorFDL,
    NamePlate(EDString),           // name plate name
    Paintjob(ShipType, EDString),  // paintjob name
    ShipKit(ShipType, EDString),   // shipkit name
    String(EDString),              // string name
    VoicePack(EDString),           // voicepack name
    WeaponCustomisation(EDString), // customisation name
    Null,                          // when module is removed, the log shows string "Null"
}

impl Display for ShipModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShipModule::Armour(_, grade) => match grade {
                ShipArmourGrade::Grade1 => write!(f, "Lightweight Alloy"),
                ShipArmourGrade::Grade2 => write!(f, "Reinforced Alloy"),
                ShipArmourGrade::Grade3 => write!(f, "Military Grade Composite"),
                ShipArmourGrade::Grade4 => write!(f, "Armour Grade 4"),
                ShipArmourGrade::Grade5 => write!(f, "Armour Grade 5"),
                ShipArmourGrade::Reactive => write!(f, "Reactive Surface Composite"),
            },
            ShipModule::OptionalInternal(module, _size, class) => match module {
                ShipModuleOptionalInternal::PassengerCabin => {
                    write!(
                        f,
                        "{}Passenger Cabin",
                        class
                            .passenger_cabin_abbr()
                            .map(|a| a.to_owned() + " ")
                            .unwrap_or("".to_owned())
                    )
                }
                ShipModuleOptionalInternal::PassengerCabinMkII => {
                    write!(
                        f,
                        "Mk II {}Passenger Cabin",
                        class
                            .passenger_cabin_abbr()
                            .map(|a| a.to_owned() + " ")
                            .unwrap_or("".to_owned())
                    )
                }
                _ => write!(f, "{module}"),
            },
            ShipModule::CoreInternal(module, _size, _class) => write!(f, "{module}"),
            ShipModule::UtilityMount(module, _size, _class) => write!(f, "{module}"),
            ShipModule::Hardpoint(hpt, _conn, _size) => write!(f, "{hpt}"),
            ShipModule::Cockpit(_ship) => write!(f, "Cockpit"),
            ShipModule::ModularCargoBayDoor => write!(f, "Cargo Hatch"),
            ShipModule::ModularCargoBayDoorFDL => write!(f, "Cargo Hatch"),
            ShipModule::Paintjob(_, _) => write!(f, "Paintjob"),
            ShipModule::VoicePack(_) => write!(f, "Voicepack"),
            ShipModule::Decal(_) => write!(f, "Decal"),
            ShipModule::WeaponCustomisation(_) => write!(f, "Weapon Customisation"),
            ShipModule::EngineCustomisation(_) => write!(f, "Engine Customisation"),
            ShipModule::NamePlate(_) => write!(f, "Name Plate Customisation"),
            ShipModule::Bobble(_) => write!(f, "Bobble Customisation"),
            ShipModule::String(_) => write!(f, "String Customisation"),
            ShipModule::ShipKit(_, _) => write!(f, "Ship Kit"),
            ShipModule::External(_, _, _) => write!(f, "Drive"),
            ShipModule::Null => write!(f, "Null"),
        }
    }
}
