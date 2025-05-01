use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    path::PathBuf,
};

pub type PinCollection = BTreeSet<String>;

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
struct Devices {
    pin_groups: HashMap<String, PinCollection>,
    families: HashMap<String, Family>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Family {
    pins: HashMap<String, PinCollection>,
    peripherals: Vec<Peripheral>,
}

#[derive(Debug)]
struct Peripheral {
    name: String,
    variant: Option<String>,
    start: u32,
    count: Option<u32>,
    only: Option<Vec<String>>,
    except: Option<Vec<String>>,
}

impl<'de> serde::Deserialize<'de> for Peripheral {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        struct PeripheralBody {
            #[serde(default)]
            variant: Option<String>,
            #[serde(default)]
            only: Option<Vec<String>>,
            #[serde(default)]
            except: Option<Vec<String>>,
            #[serde(default)]
            count: Option<u32>,
            #[serde(default)]
            start: u32,
        }

        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Peripheral;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a string or a peripheral mapping")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Peripheral {
                    name: v.to_string(),
                    variant: None,
                    count: None,
                    only: None,
                    except: None,
                    start: 0,
                })
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Peripheral {
                    name: v,
                    variant: None,
                    count: None,
                    only: None,
                    except: None,
                    start: 0,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                if let Some((name, body)) = map.next_entry::<String, PeripheralBody>()? {
                    if map
                        .next_entry::<serde_yaml::Value, serde_yaml::Value>()?
                        .is_none()
                    {
                        Ok(Peripheral {
                            name,
                            variant: body.variant,
                            only: body.only,
                            except: body.except,
                            count: body.count,
                            start: body.start,
                        })
                    } else {
                        Err(serde::de::Error::invalid_length(
                            2,
                            &"expected peripheral map to have a single element",
                        ))
                    }
                } else {
                    Err(serde::de::Error::invalid_length(
                        0,
                        &"expected peripheral map to have a single element",
                    ))
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

fn normalize_pins(pin_groups: &HashMap<String, PinCollection>, pins: &mut PinCollection) -> bool {
    let mut new = Vec::new();
    let mut changed = false;
    loop {
        pins.retain(|pin| {
            if let Some(pins) = pin_groups.get(pin) {
                new.extend(pins.iter().cloned());
                false
            } else {
                true
            }
        });
        if new.is_empty() {
            break;
        }
        pins.extend(new.drain(..));
        changed = true;
    }
    changed
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=devices.yaml");
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

    let peripheral_mapping = load_peripheral_mapping(manifest_dir)?;
    let all_devices = peripheral_mapping
        .values()
        .flat_map(|value| value.iter())
        .map(|device| device.as_str())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|s| format!("{s:?}"))
        .collect::<Vec<String>>()
        .join(", ");
    let map = generate_phf_map(&peripheral_mapping);

    let file = std::fs::File::create(out_dir.join("generated.rs"))?;
    let mut file = std::io::BufWriter::new(file);
    use std::io::Write;
    write!(
        file,
        "static PERIPHERALS: phf::Map<&str, &[&str]> = {};",
        map.build()
    )?;

    write!(file, "static ALL_DEVICES: &[&str] = &[{all_devices}];",)?;

    Ok(())
}

fn generate_phf_map(
    peripheral_mapping: &BTreeMap<String, BTreeSet<String>>,
) -> phf_codegen::Map<&str> {
    let mut map = phf_codegen::Map::new();

    for (peripheral, devices) in peripheral_mapping {
        let mut value = String::from("&[");
        for device in devices {
            use std::fmt::Write;
            write!(value, "{device:?},").unwrap();
        }
        value.push(']');
        map.entry(peripheral.as_str(), &value);
    }
    map
}

fn load_peripheral_mapping(
    manifest_dir: PathBuf,
) -> Result<BTreeMap<String, BTreeSet<String>>, std::io::Error> {
    let mut devices: Devices =
        serde_yaml::from_reader(std::fs::File::open(manifest_dir.join("devices.yaml"))?).unwrap();
    loop {
        let mut changed = false;
        for key in devices
            .pin_groups
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .into_iter()
        {
            let mut cur = std::mem::take(devices.pin_groups.get_mut(&key).unwrap());
            changed |= normalize_pins(&devices.pin_groups, &mut cur);
            devices.pin_groups.insert(key, cur);
        }
        if !changed {
            break;
        }
    }
    let mut peripheral_mapping: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut peripheral_sub_mapping: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (family_name, mut family) in devices.families {
        for (device_name, pins) in family.pins.iter_mut() {
            normalize_pins(&devices.pin_groups, pins);
            for pin in pins.iter() {
                peripheral_mapping
                    .entry(pin.clone())
                    .or_default()
                    .insert(device_name.clone());
                let port_group = pin
                    .trim_end_matches(|c: char| c.is_ascii_digit())
                    .strip_prefix('p')
                    .unwrap();
                peripheral_mapping
                    .entry(format!("pin-group-{port_group}",))
                    .or_default()
                    .insert(device_name.clone());
            }
        }
        for peripheral in family.peripherals {
            let variant = peripheral.variant.unwrap_or_else(|| family_name.clone());
            for i in 0..peripheral.count.unwrap_or(1) {
                let name = if peripheral.count.is_some() {
                    format!("{}{}", peripheral.name, peripheral.start + i)
                } else {
                    peripheral.name.clone()
                };
                let combined_name = format!("{name}-{variant}");
                let devices = family
                    .pins
                    .keys()
                    .filter(|key| {
                        peripheral
                            .only
                            .as_ref()
                            .map_or(true, |only| only.contains(key))
                            && peripheral
                                .except
                                .as_ref()
                                .map_or(true, |except| !except.contains(key))
                    })
                    .cloned();
                peripheral_mapping
                    .entry(name)
                    .or_default()
                    .extend(devices.clone());
                peripheral_sub_mapping
                    .entry(combined_name)
                    .or_default()
                    .extend(devices);
            }
        }
    }

    for (sub_peripheral_key, sub_peripheral) in peripheral_sub_mapping {
        assert!(peripheral_mapping
            .insert(sub_peripheral_key, sub_peripheral)
            .is_none());
    }

    Ok(peripheral_mapping)
}
