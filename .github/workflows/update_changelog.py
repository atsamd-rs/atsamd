import argparse
import re
import sys

def update_changelog(changelog_path, version):
    try:
        with open(changelog_path, 'r', encoding='utf-8') as f:
            changelog = f.read()
    except FileNotFoundError:
        sys.stderr.write(f"Error: Changelog file not found at {changelog_path}.")
        sys.exit(1)

    unreleased_pattern = re.compile(r'^# unreleased.*$', re.IGNORECASE | re.MULTILINE)

    if not unreleased_pattern.search(changelog):
        sys.stderr.write(f"Error: No 'Unreleased' section found in {changelog_path}.")
        sys.exit(2)

    replacement_text = f"# Unreleased Changes\n\n# v{version}"
    updated_changelog = re.sub(unreleased_pattern, replacement_text, changelog)
    with open(changelog_path, 'w', encoding='utf-8') as f:
        f.write(updated_changelog)

    print(f"Changelog updated successfully with version {version}.")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Update crate changelog by parsing version from Cargo manifest")
    parser.add_argument('crate_path', help="Path to the crate")
    parser.add_argument('crate_version', help="Version of the crate")
    
    args = parser.parse_args()

    # Get the crate version from Cargo.toml
    manifest_path = f'{args.crate_path}/Cargo.toml'
    
    # Update the changelog
    crate_changelog_path = f'{args.crate_path}/CHANGELOG.md'
    update_changelog(crate_changelog_path, args.crate_version)
