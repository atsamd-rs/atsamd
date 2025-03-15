extern crate libxslt;
extern crate libxml;
extern crate svd2rust;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use libxml::{parser::Parser as XMLParser, tree::SaveOptions};
use libxslt::parser as xslt_parser;

//TODO: Make the name of the SVD file a parameter.
//TODO: Make the XSLT optional.
//TODO: Avoid the need to process the output of svd2rust.

fn main() {
    let xml_parser = XMLParser::default();
    let mut stylesheet = xslt_parser::parse_file("atsamc21j18a.xsl").unwrap();
    let source = xml_parser.parse_file("ATSAMC21J18A.svd").unwrap();
    println!("Applying XSLT");
    let result = stylesheet.transform(&source, Vec::new()).unwrap();
    let input = result.to_string_with_options(SaveOptions {
        format: true,
        ..SaveOptions::default()
    });

    let mut config = svd2rust::util::Config::default();
    config.make_mod = true;

    println!("Parsing device from SVD file");
    let device = svd2rust::load_from(&input, &config).unwrap();

    let mut device_x = String::new();
    println!("Rendering device");
    let items = svd2rust::generate::device::render(&device, &config, &mut device_x).unwrap();
    let data = items.to_string().replace("] ", "]\n");
    let (_, data) = data.split_at(data.find("use").unwrap());

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut file = File::create(out.join("pac.rs")).expect("Couldn't create output file pac.rs");
    file.write_all(data.as_ref()).expect("Could not write code to pac.rs");

    let mut file = File::create(out.join("device.x")).expect("Couldn't create output file device.x");
    file.write_all(device_x.as_ref()).expect("Could not write device.x");
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        println!("cargo:rustc-link-search={}", out.display());
    }

}