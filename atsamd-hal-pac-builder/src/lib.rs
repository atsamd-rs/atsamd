use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use svd2rust::config::IdentFormats;
use svdtools::patch::{Config as PatchConfig, process_file};
use tap::Tap;

/// Get the chip name from the `Cargo.toml` manifest.
///
/// The `Cargo.toml` must specify the chip name as following:
/// ```toml
/// [package.metadata]
/// chip = "atsamd21j"
/// ```
pub fn get_chip_name() -> Result<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let cargo_toml_path = format!("{}/Cargo.toml", manifest_dir);

    let contents = fs::read_to_string(&cargo_toml_path)
        .context(format!("Failed to read {cargo_toml_path}"))?;

    let value = toml::from_str::<toml::Value>(&contents).context("Failed to parse Cargo.toml")?;

    let chip_name = value
        .get("package")
        .and_then(|pkg| pkg.get("metadata"))
        .and_then(|meta| meta.get("chip"))
        .and_then(|chip| chip.as_str())
        .context("Missing package.metadata.chip in Cargo.toml")?
        .to_string();

    Ok(chip_name)
}

/// Generate the PAC code using svd2rust, and write it to the provided
/// directory. First reads the SVD file associated with the relevant part number
/// (which is extracted from the PAC's package name), then applies the relevant
/// XSL templates before running the codegen pass.
///
/// # Inputs
///
/// * `chip_name`: The name of the chip for which the PAC is being built,
///   including package configuration. Excludes memory configuration, e.g.
///   `ATSAMD51J`. Can be extracted from the PAC's `Cargo.toml` by using [`get_chip_name`].
/// * `pac_out_dir`: The PAC's output directory as reported by the `OUT_DIR`
///   environment variable
/// * `svd_dir`: The root directory where the SVD and XSLT files are stored
pub fn generate_pac(
    chip_name: impl AsRef<str>,
    pac_out_dir: impl AsRef<Path>,
    svd_root: impl AsRef<Path>,
) -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../build.rs");
    println!("cargo:rerun-if-changed=../../svd");
    println!("cargo:rerun-if-changed=../../svd/devices");
    println!("cargo:rerun-if-changed=../../svd/include");

    let out_dir = pac_out_dir.as_ref();
    let chip_name = chip_name.as_ref();

    // Find the device patch YAML
    let yaml = find_chip_file(chip_name, "yml", svd_root.as_ref().join("devices"))
        .context("Could not find device patch YAML")?;

    let patched_svd_path = out_dir.join("patched.svd");
    process_file(
        Path::new(&yaml),
        Some(&patched_svd_path),
        None,
        &PatchConfig::default(),
    )
    .with_context(|| format!("Failed to patch SVD using {yaml}"))?;

    let patched_svd =
        fs::read_to_string(&patched_svd_path).context("Could not read patched SVD")?;

    // svd2rust config
    let config = svd2rust::config::Config::default().tap_mut(|c| {
        c.target = svd2rust::Target::CortexM;
        c.atomics = true;
        c.make_mod = true;
        c.ident_formats = IdentFormats::default_theme();
        c.reexport_core_peripherals = true;
        c.reexport_interrupt = true;
        c.edition = svd2rust::config::RustEdition::E2024;
    });

    // Generate PAC code
    let generated = svd2rust::generate(&patched_svd, &config)
        .context("Failed to generate PAC using svd2rust")?;

    let pac_mod_out = out_dir.join("pac.rs");
    let generated_out = out_dir.join("pac_impl.rs");
    let linker_script = out_dir.join("device.x");
    let build_script = out_dir.join("build.rs");

    let device_specific_code = generated
        .device_specific
        .ok_or_else(|| anyhow::anyhow!("No device specific generated code"))?;

    fs::write(&generated_out, &generated.lib_rs)
        .context("failed to write generated code to pac_impl.rs")?;
    fs::write(&linker_script, device_specific_code.device_x)?;
    fs::write(&build_script, device_specific_code.build_rs)?;

    // Write a "wrapper" module in OUT_DIR/pac.rs, which links to the real PAC
    // generated code
    fs::write(
        &pac_mod_out,
        format!(r#"#[path="{}"] mod __pac_impl;"#, generated_out.display()),
    )?;

    // `rustfmt`ting the generated files is important, otherwise `cargo doc`
    // takes FOREVER. I'm guessing it works line by line.
    let _ = Command::new("rustfmt").arg(pac_mod_out).status();
    let _ = Command::new("rustfmt").arg(generated_out).status();

    Ok(())
}

/// Include `device.x` in the linker search path
pub fn include_linker_script(pac_out_dir: impl AsRef<Path>) -> Result<()> {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RT");

    let out_dir = pac_out_dir.as_ref();

    // Add out dir to link search. device.x is generated directly by svd2rust and written into OUT_DIR by generate_pac.
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    Ok(())
}

// Find a file for `chip_name`, ending with `extension` in the provided `dir`.
//
// May be used to find yaml patch files for ATSAMD chips.
fn find_chip_file(chip_name: &str, extension: &str, dir: impl AsRef<Path>) -> Result<String> {
    // Get all entries in the svd directory
    let dir = dir.as_ref();

    // Check if the svd directory exists
    if !dir.exists() || !dir.is_dir() {
        return Err(anyhow::anyhow!("SVD directory not found"));
    }

    // Read all entries in the directory
    let entries = fs::read_dir(dir).context("Error reading SVD directory")?;

    // Look for files that start with the chip name
    for entry in entries.flatten() {
        let path = entry.path();

        // Skip if not a file
        if !path.is_file() {
            continue;
        }

        // Get file stem and extension
        if let Some(file_stem) = path.file_stem() {
            let file_stem = file_stem
                .to_os_string()
                .into_string()
                .expect("Could not get path");

            if file_stem.to_lowercase() == chip_name.to_lowercase() {
                // Check for extension
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if ext_str == extension {
                        let path = path
                            .into_os_string()
                            .into_string()
                            .expect("Could not get path");
                        return Ok(path);
                    }
                }
            }
        }
    }

    Err(anyhow::anyhow!("Chip not found"))
}
