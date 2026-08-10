use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::Command;

use anyhow::Context;
use libxml::{parser::Parser as XMLParser, tree::SaveOptions};
use libxslt::parser as xslt_parser;
use svd2rust::config::IdentFormats;
use tap::Tap;

/// Generate the PAC code using svd2rust, and write it to the provided
/// directory. First reads the SVD file associated with the relevant part number
/// (which is extracted from the PAC's package name), then applies the relevant
/// XSL templates before running the codegen pass.
///
/// # Inputs
///
/// * `pac_pkg_name`: The PAC's package name as reported by the `CARGO_PKG_NAME`
///   environment variable
/// * `pac_out_dir`: The PAC's output directory as reported by the `OUT_DIR`
///   environment variable
/// * `svd_dir`: The root directory where the SVD and XSLT files are stored
pub fn generate_pac(
    pac_pkg_name: impl AsRef<str>,
    pac_out_dir: impl AsRef<Path>,
    svd_root: impl AsRef<Path>,
) -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../svd");

    let out_dir = pac_out_dir.as_ref();

    // Extract chip name, and find SVD + XSL
    let chip_name = get_chip_name(pac_pkg_name).context("Could not extract chip name")?;
    let xsl = find_chip_file(&chip_name, "xsl", svd_root.as_ref().join("devices"))
        .context("Could not find xsl")?;
    let svd = find_chip_file(&chip_name.to_ascii_uppercase(), "svd", svd_root)
        .context("Could not find SVD")?;

    // Parse SVD
    let xml_parser = XMLParser::default();
    let source = xml_parser.parse_file(&svd).unwrap();

    // Apply XSLT
    let mut stylesheet = xslt_parser::parse_file(&xsl).unwrap();
    let transform_result = stylesheet.transform(source, Vec::new()).unwrap();
    let patched_svd = transform_result.to_string_with_options(SaveOptions {
        format: true,
        ..SaveOptions::default()
    });

    // svd2rust config
    let config = svd2rust::config::Config::default().tap_mut(|c| {
        c.atomics = true;
        c.make_mod = true;
        c.ident_formats = IdentFormats::default_theme();
        c.reexport_core_peripherals = true;
        c.edition = svd2rust::config::RustEdition::E2024;
    });

    // Generate PAC code
    let generated = svd2rust::generate(&patched_svd, &config)
        .context("Failed to generate PAC using svd2rust")?;

    let pac_mod_out = out_dir.join("pac.rs");
    let generated_out = out_dir.join("pac_impl.rs");
    fs::write(&generated_out, &generated.lib_rs)
        .context("failed to write generated code to pac_impl.rs")?;

    // Write a "wrapper" module in OUT_DIR/pac.rs, which links to the real PAC
    // generated code
    fs::write(
        &pac_mod_out,
        format!(r#"#[path="{}"] mod pac;"#, generated_out.display()),
    )?;

    // `rustfmt`ting the generated files is important, otherwise `cargo doc` takes
    // FOREVER. I'm guessing it works line by line.
    let _ = Command::new("rustfmt").arg(pac_mod_out).status();
    let _ = Command::new("rustfmt").arg(generated_out).status();

    Ok(())
}

/// Include `memory.x` in the linker search path
pub fn include_linker_script(
    pac_out_dir: impl AsRef<Path>,
    linker_script: impl AsRef<Path>,
) -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RT");

    let out_dir = pac_out_dir.as_ref();

    // Copy linker script to out dir
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let mut rdr = BufReader::new(File::open(linker_script.as_ref())?);
        let mut buf = vec![];
        rdr.read_to_end(&mut buf)?;

        File::create(out_dir.join("device.x"))
            .unwrap()
            .write_all(&buf)
            .unwrap();
        println!("cargo:rustc-link-search={}", out_dir.display());
    }

    Ok(())
}

// Find a file for `chip_name`, ending with `extension` in the provided `dir`.
// Omits the part number suffix, i.e., the memory variant.
//
// May be used to find SVD or XSL files for ATSAMD chips.
fn find_chip_file(chip_name: &str, extension: &str, dir: impl AsRef<Path>) -> Option<String> {
    // Get all entries in the svd directory
    let dir = dir.as_ref();

    // Check if the svd directory exists
    if !dir.exists() || !dir.is_dir() {
        println!("SVD directory not found");
        return None;
    }

    // Read all entries in the directory
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error reading SVD directory: {}", e);
            return None;
        }
    };

    // Look for files that start with the chip name
    for entry in entries.flatten() {
        let path = entry.path();

        // Skip if not a file
        if !path.is_file() {
            continue;
        }

        // Get file stem and extension
        if let Some(file_name) = path.file_name() {
            let file_name_str = file_name
                .to_os_string()
                .into_string()
                .expect("Could not get path")
                .to_lowercase();
            let chip_name_lower = chip_name.to_lowercase();

            // Check if the file name starts with the chip name (case insensitive)
            if file_name_str.starts_with(&chip_name_lower) {
                // Check for extension
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if ext_str == extension {
                        let path = path
                            .into_os_string()
                            .into_string()
                            .expect("Could not get path");
                        return Some(path);
                    }
                }
            }
        }
    }

    None
}

/// Get the chip name from the PAC package name
fn get_chip_name(pkg_name: impl AsRef<str>) -> Option<String> {
    // let pkg_name = env!("CARGO_PKG_NAME");
    let re = regex::Regex::new(r"atsam[a-z]\d{2}[a-z]+").unwrap();

    Some(re.captures(pkg_name.as_ref())?.get(0)?.as_str().to_owned())
}
