use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};

const THUMBV7EM_TARGET: &str = "thumbv7em-none-eabihf";
const THUMBV6M_TARGET: &str = "thumbv6m-none-eabi";

const POTENTIAL_BSPS: [(&str, &str); 7] = [
    ("feather_m0", THUMBV6M_TARGET),
    ("feather_m4", THUMBV7EM_TARGET),
    ("metro_m0", THUMBV6M_TARGET),
    ("metro_m4", THUMBV7EM_TARGET),
    ("pygamer", THUMBV7EM_TARGET),
    ("samd11_bare", THUMBV6M_TARGET),
    ("wio_terminal", THUMBV7EM_TARGET),
];

fn main() {
    let mut enabled_bsp_found = None;
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut out_memory_file = File::create(out.join("memory.x")).unwrap();

    for (bsp, target) in POTENTIAL_BSPS {
        let mut feature_string = String::from_str("CARGO_FEATURE_BSP_").unwrap();
        feature_string.push_str(bsp);
        let feature_string = feature_string.to_uppercase().replace("-", "_");

        if env::var_os(feature_string).is_some() {
            let build_target = env::var_os("TARGET").unwrap();
            if build_target != target {
                eprintln!(
                    "incorrect target for {}. Expected {}, got {:?}",
                    bsp, target, build_target
                );
                std::process::exit(-1);
            }
            if enabled_bsp_found.is_some() {
                eprintln!(
                    "Only one BSP can be enabled at a time (at least {} and {} enabled)",
                    enabled_bsp_found.unwrap(),
                    bsp
                );
                std::process::exit(-1);
            }

            eprintln!("enabled BSP {}", bsp);
            enabled_bsp_found = Some(bsp);
            let memory_file_path = &PathBuf::from(format!("../boards/{}/memory.x", bsp));
            let mut memory_file = File::open(memory_file_path).unwrap();

            let mut memory_file_string = String::new();
            memory_file.read_to_string(&mut memory_file_string).unwrap();
            out_memory_file
                .write(&memory_file_string.as_bytes())
                .unwrap();

            println!("cargo:rerun-if-changed={}", memory_file_path.display());
        }
    }

    if enabled_bsp_found.is_none() {
        panic!("Failed to find an enabled BSP");
    }

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=build.rs");
}
