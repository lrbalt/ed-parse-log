use crate::{EDString, ship_type::ShipType};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::Display;

pub mod serde_ship_module {
    use crate::{
        ship_module::{
            HardpointConnection, HardpointSize, ShipArmourGrade, ShipModule, ShipModuleClass,
            ShipModuleExternal, ShipModuleHardpoint,
            ShipModuleInternal::{self, *},
            ShipModuleSize,
        },
        ship_type::ShipType,
    };
    use serde::{
        Deserialize, Deserializer, Serializer,
        de::{
            IntoDeserializer,
            value::{Error as ValueError, StrDeserializer},
        },
    };

    fn deserialize_enum_from_str<'de, T, E>(s: &str, ctx: &str) -> Result<T, E>
    where
        T: Deserialize<'de>,
        E: serde::de::Error,
    {
        let deserializer: StrDeserializer<ValueError> = s.into_deserializer();
        T::deserialize(deserializer).map_err(|e| E::custom(format!("Error parsing {ctx}: {e}")))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ShipModule, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let mut parts = s.split('_').collect::<Vec<_>>();

        if parts.is_empty() {
            return Err(serde::de::Error::custom(format!(
                "Did not find a prefix on '{:?}'",
                s
            )));
        }

        if parts[0].starts_with('$') {
            // remove the $ prefix
            parts[0] = &parts[0][1..];
            if parts[parts.len() - 1] == "name;" {
                // $module_name; pattern, remove the trailing name;
                parts.pop();
            }
            // some patterns have a prefix $ and a suffic ; but not _name; at the end
            if parts[parts.len() - 1].ends_with(';') {
                // remove the trailing ; from the last part
                let last = parts.len() - 1;
                parts[last] = parts[last].trim_end_matches(';');
            }
        }

        match parts[0] {
            "Null" => Ok(ShipModule::Null),
            "int" => {
                // handle modules that do not have pattern $int_name_class_size_name;
                match parts[1] {
                    "dockingcomputer" => {
                        // handle $int_dockingcomputer_advanced_name;
                        let module = if parts[2] == "advanced" {
                            ShipModuleInternal::DockingComputerAdvanced
                        } else {
                            ShipModuleInternal::DockingComputer
                        };
                        return Ok(ShipModule::Internal(
                            module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "planetapproachsuite" => {
                        let module = if parts.len() > 2 && parts[2] == "advanced" {
                            ShipModuleInternal::PlanetApproachSuiteAdvanced
                        } else {
                            ShipModuleInternal::PlanetApproachSuite
                        };
                        return Ok(ShipModule::Internal(
                            module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "colonisation"
                    | "supercruiseassist"
                    | "codexscanner"
                    | "stellarbodydiscoveryscanner" => {
                        let internal_module: ShipModuleInternal =
                            deserialize_enum_from_str::<_, D::Error>(parts[1], "ship module name")?;

                        return Ok(ShipModule::Internal(
                            internal_module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "mkii" => {
                        // int_mkii_passengercabin_size2_class1_name
                        let module = ShipModuleInternal::PassengerCabinMkII;
                        let module_size: ShipModuleSize =
                            deserialize_enum_from_str::<_, D::Error>(parts[3], "ship module size")?;

                        let module_class: ShipModuleClass = deserialize_enum_from_str::<_, D::Error>(
                            parts[4],
                            "ship module class",
                        )?;
                        return Ok(ShipModule::Internal(module, module_size, module_class));
                    }
                    "mkiiagileboost" => {
                        // handle "$int_mkiiagileboost_engine_size5_class5_name;"
                        let module = ShipModuleInternal::EngineMkIIAgileBoost;
                        let module_size: ShipModuleSize =
                            deserialize_enum_from_str::<_, D::Error>(parts[3], "ship module size")?;

                        let module_class: ShipModuleClass = deserialize_enum_from_str::<_, D::Error>(
                            parts[4],
                            "ship module class",
                        )?;
                        return Ok(ShipModule::Internal(module, module_size, module_class));
                    }
                    "dronecontrol" | "multidronecontrol" if parts.len() > 4 => {
                        // handle "$int_dronecontrol_fueltransfer_size1_class5_name;"
                        let module: ShipModuleInternal = deserialize_enum_from_str::<_, D::Error>(
                            (parts[1].to_string() + "_" + parts[2]).as_str(),
                            "ship module size",
                        )?;
                        let module_size: ShipModuleSize =
                            deserialize_enum_from_str::<_, D::Error>(parts[3], "ship module size")?;
                        let module_class: ShipModuleClass = deserialize_enum_from_str::<_, D::Error>(
                            parts[4],
                            "ship module class",
                        )?;
                        return Ok(ShipModule::Internal(module, module_size, module_class));
                    }
                    "dronecontrol" if parts.len() == 3 || parts.len() == 4 => {
                        // int_dronecontrol_unkvesselresearch
                        // $int_dronecontrol_unkvesselresearch_name;
                        let module: ShipModuleInternal = deserialize_enum_from_str::<_, D::Error>(
                            (parts[1].to_string() + "_" + parts[2]).as_str(),
                            "ship module size",
                        )?;
                        return Ok(ShipModule::Internal(
                            module,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                    "guardianfsdbooster" => {
                        // int_guardianfsdbooster_size5
                        let module: ShipModuleInternal =
                            deserialize_enum_from_str::<_, D::Error>(parts[1], "ship module name")?;
                        let module_size: ShipModuleSize =
                            deserialize_enum_from_str::<_, D::Error>(parts[2], "ship module size")?;
                        return Ok(ShipModule::Internal(
                            module,
                            module_size,
                            ShipModuleClass::None,
                        ));
                    }
                    "detailedsurfacescanner" => {
                        // $int_detailedsurfacescanner_tiny_name;
                        let module: ShipModuleInternal =
                            deserialize_enum_from_str::<_, D::Error>(parts[1], "ship module name")?;
                        let module_size: HardpointSize = deserialize_enum_from_str::<_, D::Error>(
                            parts[2],
                            "ship hardpoint size",
                        )?;
                        return Ok(ShipModule::OptionalInternalHptSized(module, module_size));
                    }
                    "hyperdrive" => {
                        // int_hyperdrive_size5_class5
                        if parts[2] == "overcharge" {
                            // int_hyperdrive_overcharge_size5_class5
                            let module_size: ShipModuleSize =
                                deserialize_enum_from_str::<_, D::Error>(
                                    parts[3],
                                    "ship module size",
                                )?;
                            let module_class: ShipModuleClass =
                                deserialize_enum_from_str::<_, D::Error>(
                                    parts[4],
                                    "ship module class",
                                )?;
                            return Ok(ShipModule::Internal(
                                HyperdriveOvercharge,
                                module_size,
                                module_class,
                            ));
                        } else {
                            let module_size: ShipModuleSize =
                                deserialize_enum_from_str::<_, D::Error>(
                                    parts[2],
                                    "ship module size",
                                )?;
                            let module_class: ShipModuleClass =
                                deserialize_enum_from_str::<_, D::Error>(
                                    parts[3],
                                    "ship module class",
                                )?;
                            return Ok(ShipModule::Internal(Hyperdrive, module_size, module_class));
                        }
                    }
                    _ => { /* continue */ }
                }

                // handle pattern $int_name_class_size_name;

                if parts.len() < 4 {
                    return Err(serde::de::Error::custom(format!(
                        "Error parsing {s}: expected four parts"
                    )));
                }

                let mut internal_module: ShipModuleInternal =
                    deserialize_enum_from_str::<_, D::Error>(parts[1], "ship module name")?;

                let module_size: ShipModuleSize =
                    deserialize_enum_from_str::<_, D::Error>(parts[2], "ship module size")?;

                let module_class: ShipModuleClass =
                    deserialize_enum_from_str::<_, D::Error>(parts[3], "ship module class")?;

                // handle $int_engine_size7_class5_gravityoptimised_mkii_name;
                // handle prismatic and bi-weave shields
                if parts.len() > 4 {
                    match internal_module {
                        ShipModuleInternal::Engine if parts[4] == "gravityoptimised" => {
                            internal_module = ShipModuleInternal::EngineGravityOptimisedMkII;
                        }
                        ShipModuleInternal::ShieldGenerator if parts[4] == "strong" => {
                            internal_module = ShipModuleInternal::PrismaticShieldGenerator;
                        }
                        ShipModuleInternal::ShieldGenerator if parts[4] == "fast" => {
                            internal_module = ShipModuleInternal::BiWeaveShieldGenerator;
                        }
                        _ => { /* continue */ }
                    }
                }

                Ok(ShipModule::Internal(
                    internal_module,
                    module_size,
                    module_class,
                ))
            }
            "hpt" => {
                // matches patters other than $hpt_name_connection_size_name; first
                match parts[1] {
                    "mining" | "guardian" if parts.len() > 4 => {
                        // handle $hpt_mining_subsurfdispmisle_fixed_medium_name;
                        // or $hpt_guardian_plasmalauncher_fixed_medium_name;
                        let module: ShipModuleHardpoint = deserialize_enum_from_str::<_, D::Error>(
                            (parts[1].to_string() + "_" + parts[2]).as_str(),
                            "ship module name",
                        )?;
                        let module_conn: HardpointConnection =
                            deserialize_enum_from_str::<_, D::Error>(
                                parts[3],
                                "ship hardpoint connection",
                            )?;
                        let module_size: HardpointSize = deserialize_enum_from_str::<_, D::Error>(
                            parts[4],
                            "ship hardpoint size",
                        )?;
                        return Ok(ShipModule::Hardpoint(module, module_conn, module_size));
                    }
                    "xenoscanner" | "xenoscannermk2" => {
                        // hpt_xenoscanner_basic_tiny | hpt_xenoscannermk2_basic_tiny
                        let module: ShipModuleInternal = deserialize_enum_from_str::<_, D::Error>(
                            (parts[1].to_string() + "_" + parts[2]).as_str(),
                            "ship module name",
                        )?;
                        let module_size: HardpointSize = deserialize_enum_from_str::<_, D::Error>(
                            parts[3],
                            "ship hardpoint size",
                        )?;
                        return Ok(ShipModule::OptionalInternalHptSized(module, module_size));
                    }
                    "electroniccountermeasure" | "chafflauncher" => {
                        // hpt_electroniccountermeasure_tiny
                        let module: ShipModuleInternal =
                            deserialize_enum_from_str::<_, D::Error>(parts[1], "ship module name")?;
                        let module_size: HardpointSize = deserialize_enum_from_str::<_, D::Error>(
                            parts[2],
                            "ship hardpoint size",
                        )?;
                        return Ok(ShipModule::OptionalInternalHptSized(module, module_size));
                    }
                    "antiunknownshutdown" => {
                        // hpt_antiunknownshutdown_tiny_v2
                        // hpt_antiunknownshutdown_tiny

                        let name = if parts.len() == 4 && parts[3] == "v2" {
                            parts[1].to_string() + parts[3]
                        } else {
                            parts[1].to_string()
                        };

                        let module: ShipModuleInternal = deserialize_enum_from_str::<_, D::Error>(
                            name.as_str(),
                            "ship module name",
                        )?;
                        let module_size: HardpointSize = deserialize_enum_from_str::<_, D::Error>(
                            parts[2],
                            "ship hardpoint size",
                        )?;
                        return Ok(ShipModule::OptionalInternalHptSized(module, module_size));
                    }
                    _ => { /* continue */ }
                }

                // optional internal had hpt prefix
                if let Ok(module) = deserialize_enum_from_str::<ShipModuleInternal, D::Error>(
                    parts[1],
                    "ship optional internal name",
                ) {
                    if matches!(
                        module,
                        ShipModuleInternal::ShieldBooster
                            | ShipModuleInternal::CloudScanner
                            | ShipModuleInternal::MRAScanner
                            | ShipModuleInternal::CrimeScanner
                            | ShipModuleInternal::CargoScanner
                    ) {
                        // hpt_cloudscanner_size0_class4 or hpt_mrascanner_size0_class5
                        let module_size: ShipModuleSize =
                            deserialize_enum_from_str::<_, D::Error>(parts[2], "ship module size")?;

                        let module_class: ShipModuleClass = deserialize_enum_from_str::<_, D::Error>(
                            parts[3],
                            "ship module class",
                        )?;
                        return Ok(ShipModule::OptionalInternalSized(
                            module,
                            module_size,
                            module_class,
                        ));
                    }

                    if module == ShipModuleInternal::ShipDataLinkScanner {
                        return Ok(ShipModule::Internal(
                            ShipModuleInternal::ShipDataLinkScanner,
                            ShipModuleSize::None,
                            ShipModuleClass::None,
                        ));
                    }
                }

                // $hpt_name_connection_size_name;

                let mut module = deserialize_enum_from_str::<ShipModuleHardpoint, D::Error>(
                    parts[1],
                    "ship hardpoint name",
                )?;

                let module_conn: HardpointConnection = deserialize_enum_from_str::<_, D::Error>(
                    parts[2],
                    "ship hardpoint connection",
                )?;

                let module_size: HardpointSize =
                    deserialize_enum_from_str::<_, D::Error>(parts[3], "ship hardpoint size")?;

                // check for range in $hpt_slugshot_fixed_large_range_name;
                // check for burst in $hpt_railgun_fixed_medium_burst_name;
                // check for scatter in $hpt_pulselaserburst_fixed_small_scatter_name;
                // check for v2 in $hpt_atmulticannon_turret_medium_v2_name;
                if parts.len() > 4 {
                    match module {
                        ShipModuleHardpoint::Slugshot if parts[4] == "range" => {
                            module = ShipModuleHardpoint::SlugshotRange;
                        }
                        ShipModuleHardpoint::RailGun if parts[4] == "burst" => {
                            module = ShipModuleHardpoint::TheHammer;
                        }
                        ShipModuleHardpoint::PulseLaserBurst if parts[4] == "scatter" => {
                            module = ShipModuleHardpoint::Cytoscrambler;
                        }
                        ShipModuleHardpoint::ATMultiCannon if parts[4] == "v2" => {
                            module = ShipModuleHardpoint::ATMultiCannonV2;
                        }
                        _ => { /* continue */ }
                    }
                }

                Ok(ShipModule::Hardpoint(module, module_conn, module_size))
            }
            "ext" => {
                // $ext_drive_class3_cob_name;
                // $ext_drive_krait_light_name;
                let module: ShipModuleExternal = deserialize_enum_from_str::<_, D::Error>(
                    parts[1],
                    "ship external module name",
                )?;
                if let Ok(module_class) =
                    deserialize_enum_from_str::<_, D::Error>(parts[2], "ship external module class")
                {
                    let rest = parts[3..].join("_");
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
                // paintjob_asp_halloween01_01
                // paintjob_smallcombat01_nx_03_01
                let (i, ship_type) = deserialize_enum_from_str(parts[1], "ship type")
                    .map(|s| (2, s))
                    .or_else(|_: D::Error| {
                        deserialize_enum_from_str(parts[1..3].join("_").as_str(), "ship type")
                            .map(|s| (3, s))
                    })?;
                let paintjob_name = parts[i..].join("_");
                Ok(ShipModule::Paintjob(ship_type, paintjob_name.into()))
            }
            "voicepack" => Ok(ShipModule::VoicePack(parts[1].into())),
            "decal" => Ok(ShipModule::Decal(parts[1..].join("_").into())),
            "weaponcustomisation" => {
                Ok(ShipModule::WeaponCustomisation(parts[1..].join("_").into()))
            }
            "enginecustomisation" => {
                Ok(ShipModule::EngineCustomisation(parts[1..].join("_").into()))
            }
            "nameplate" => Ok(ShipModule::NamePlate(parts[1..].join("_").into())),
            "bobble" => Ok(ShipModule::Bobble(parts[1..].join("_").into())),
            "string" => Ok(ShipModule::String(parts[1..].join("_").into())),
            unknown => {
                // check on cockpit module that follows $shipname_cockpit_name; pattern
                if let Some(index) = s.rfind("cockpit") {
                    // we are working on s, not on parts, so a possible $ is not removed
                    let name = s[0..index - 1].trim_matches('$');
                    let shiptype: ShipType =
                        deserialize_enum_from_str(name, "ship name of cockpit")?;
                    return Ok(ShipModule::Cockpit(shiptype));
                }

                // check on ship_armour_gradeN pattern, also $ship_ext_armour_gradeN;
                // ship_name can have more than one part, i.e. explorer_nx
                if let Some(i) = parts.iter().position(|p| *p == "armour") {
                    let (ship_name, grade_name) = (parts[0..i].join("_"), parts[i + 1]);
                    let ship_type: ShipType =
                        deserialize_enum_from_str(ship_name.as_str(), "ship type")?;
                    let grade: ShipArmourGrade =
                        deserialize_enum_from_str(grade_name, "armour grade")?;
                    return Ok(ShipModule::Armour(ship_type, grade));
                }

                if let Some(i) = parts
                    .iter()
                    .position(|p| p.starts_with("shipkit"))
                    .or_else(|| parts.iter().position(|p| p.starts_with("thargoidreward")))
                {
                    let ship_name = parts[0..i].join("_");
                    let shipkit_name = parts[i..].join("_");
                    let ship_type: ShipType =
                        deserialize_enum_from_str(ship_name.as_str(), "ship type")?;
                    return Ok(ShipModule::ShipKit(ship_type, shipkit_name.into()));
                }

                Err(serde::de::Error::custom(format!(
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipArmourGrade {
    Grade1,
    Grade2,
    Grade3,
    Grade4,
    Grade5,
    Reactive,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    Size0,
    Size1,
    Size2,
    Size3,
    Size4,
    Size5,
    Size6,
    Size7,
    Size8,
}

#[derive(Clone, Debug, Display, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ShipModuleClass {
    None,
    Class0,
    Class1,
    Class2,
    Class3,
    Class4,
    Class5,
    Class6,
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
pub enum ShipModuleInternal {
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(alias = "antiunknownshutdown")]
    AntiUnknownShutdown,
    #[strum(to_string = "Shutdown Field Neutraliser")]
    #[serde(alias = "antiunknownshutdown_v2")]
    AntiUnknownShutdownV2,
    #[strum(to_string = "Bi-Weave Shield")]
    BiWeaveShieldGenerator,
    #[strum(to_string = "Planetary Vehicle Hangar")]
    BuggyBay,
    #[strum(to_string = "Cargo Rack")]
    CargoRack,
    #[strum(to_string = "Manifest Scanner")]
    CargoScanner,
    #[strum(to_string = "Chaff")]
    ChaffLauncher,
    #[strum(to_string = "Wake Scanner")]
    CloudScanner,
    #[strum(to_string = "Anti-Corrosion Cargo Rack")]
    CorrosionProofCargoRack,
    #[strum(to_string = "K-Warrant Scanner")]
    CrimeScanner,
    #[strum(to_string = "Surface Scanner")]
    DetailedSurfaceScanner,
    #[strum(to_string = "DataLinkScanner")]
    ShipDataLinkScanner,
    #[strum(to_string = "DiscoveryScanner")]
    StellarBodyDiscoveryScanner,
    #[strum(to_string = "ColonisationSuite")]
    Colonisation,
    #[strum(to_string = "CodexScanner")]
    CodexScanner,
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
    #[strum(to_string = "ECM")]
    ElectronicCounterMeasure,
    #[strum(to_string = "Thrusters")]
    Engine,
    #[strum(to_string = "Thrusters")]
    EngineGravityOptimisedMkII,
    #[strum(to_string = "Thrusters")]
    EngineMkIIAgileBoost,
    #[strum(to_string = "Experimental Weapon Stabiliser")]
    ExpModuleStabiliser,
    #[strum(to_string = "Fighter Hangar")]
    FighterBay,
    #[strum(to_string = "FSD Interdictor")]
    FSDInterdictor,
    #[strum(to_string = "FSD")]
    Hyperdrive,
    #[strum(to_string = "FSD (SCO)")]
    HyperdriveOvercharge,
    #[strum(to_string = "FSD (SCO)")]
    FSDSCOOverchargeBoosterMkII,
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
    #[strum(to_string = "Heatsink")]
    HeatsinkLauncher,
    #[strum(to_string = "Hull Reinforcement")]
    HullReinforcement,
    #[strum(to_string = "Mk II Cargo Rack")]
    LargeCargoRack,
    #[strum(to_string = "Life Support")]
    LifeSupport,
    #[strum(to_string = "Module Reinforcement")]
    ModuleReinforcement,
    #[strum(to_string = "Pulse Wave")]
    MRAScanner,
    #[strum(to_string = "Rescue Limpet Controller")]
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
    #[strum(to_string = "Power Distributor")]
    PowerDistributor,
    #[strum(to_string = "Power Plant")]
    PowerPlant,
    #[strum(to_string = "Prismatic Shield")]
    PrismaticShieldGenerator,
    #[strum(to_string = "Refinery")]
    Refinery,
    #[strum(to_string = "AFM Unit")]
    Repairer,
    #[strum(to_string = "Advanced Planetary Approach Suite")]
    PlanetApproachSuiteAdvanced,
    #[strum(to_string = "Planetary Approach Suite")]
    PlanetApproachSuite,
    #[strum(to_string = "Sensors")]
    Sensors,
    #[strum(to_string = "Shield Booster")]
    ShieldBooster,
    #[strum(to_string = "Shield Cell Bank")]
    ShieldCellBank,
    #[strum(to_string = "Shield Generator")]
    ShieldGenerator,
    #[strum(to_string = "Supercruise Assist")]
    SupercruiseAssist,
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
    #[strum(to_string = "Advanced Torpedo Pylon")]
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
    #[strum(to_string = "Caustic Sink Launcher")]
    CausticSinkLauncher,
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
    #[strum(to_string = "Flechette Launcher")]
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
    #[strum(to_string = "Heatsink")]
    HeatSinkLauncher,
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
    #[strum(to_string = "Point Defence Turret")]
    PlasmaPointDefence,
    #[strum(to_string = "Pulse Laser")]
    PulseLaser,
    #[strum(to_string = "Burst Laser")]
    PulseLaserBurst,
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
    Internal(ShipModuleInternal, ShipModuleSize, ShipModuleClass),
    ModularCargoBayDoor,
    ModularCargoBayDoorFDL,
    NamePlate(EDString), // name plate name
    OptionalInternalSized(ShipModuleInternal, ShipModuleSize, ShipModuleClass),
    OptionalInternalHptSized(ShipModuleInternal, HardpointSize),
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
                ShipArmourGrade::Grade2 => write!(f, "Reinforced Alloys"),
                ShipArmourGrade::Grade3 => write!(f, "Military Grade Composite"),
                ShipArmourGrade::Grade4 => write!(f, "Armour Grade 4"),
                ShipArmourGrade::Grade5 => write!(f, "Armour Grade 5"),
                ShipArmourGrade::Reactive => write!(f, "Reactive Surface Composite"),
            },
            ShipModule::Internal(module, _size, class) => match module {
                ShipModuleInternal::PassengerCabin => {
                    write!(
                        f,
                        "{}Passenger Cabin",
                        class.passenger_cabin_abbr().unwrap_or("").to_owned() + " "
                    )
                }
                ShipModuleInternal::PassengerCabinMkII => {
                    write!(
                        f,
                        "Mk II {}Passenger Cabin",
                        class.passenger_cabin_abbr().unwrap_or("").to_owned() + " "
                    )
                }
                _ => write!(f, "{module}"),
            },
            ShipModule::OptionalInternalSized(module, _size, _class) => write!(f, "{module}"),
            ShipModule::OptionalInternalHptSized(module, _size) => write!(f, "{module}"),
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
