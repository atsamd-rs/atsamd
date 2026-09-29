use std::{env, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let svd_root = Path::new("../../svd");

    let chip_name = atsamd_hal_pac_builder::get_chip_name()?;
    atsamd_hal_pac_builder::generate_pac(chip_name, &out_dir, svd_root)?;
    atsamd_hal_pac_builder::include_linker_script(&out_dir)?;

    Ok(())
}
