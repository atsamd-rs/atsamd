use std::{env, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let svd_root = Path::new("../../svd");
    let pkg_name = env!("CARGO_PKG_NAME");
    let linker_script_path = Path::new("./device.x");

    atsamd_hal_pac_builder::generate_pac(pkg_name, &out_dir, svd_root)?;
    atsamd_hal_pac_builder::include_linker_script(&out_dir, linker_script_path)?;

    Ok(())
}
