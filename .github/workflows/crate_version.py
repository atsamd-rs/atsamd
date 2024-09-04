import tomlkit
import sys
import argparse

def get_crate_version(manifest_path):
    with open(manifest_path, 'r', encoding='utf-8') as f:
        cargo_toml = tomlkit.parse(f.read())
    
    return cargo_toml["package"]["version"]


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Get crate version from a Cargo manifest file")
    parser.add_argument('manifest_path', help="Path to the Cargo.toml manifest")
    args = parser.parse_args()
    
    try:
        version = get_crate_version(args.manifest_path)
        print(version)
    except (FileNotFoundError, KeyError):
        sys.stderr.write("Error: Could not find the version in the specified Cargo.toml file.\n")
        sys.exit(1)
