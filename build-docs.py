#!/usr/bin/env python
import os
import sys
import subprocess
import shutil
import argparse

# What we're going to document.
# We break this out by peripheral access crate as the features that
# get enabled for the HAL module are different and we don't want to
# emit misleading docs.
# ** Please keep the board names alpha-sorted **
crates_by_pac = {
    'atsamd21e18a': [
        'gemma_m0',
        'trinket_m0',
    ],
    'atsamd21g18a': [
        'arduino_mkrzero',
        'circuit_playground_express',
        'feather_m0',
        'itsybitsy_m0',
        'metro_m0',
        'samd21_mini',
    ],
}

def generate_docs():
    """ Run cargo doc on each crate to generate the docs """
    for pac, crates in crates_by_pac.items():
        print(pac, crates)
        for crate in crates:
            subprocess.call(['cargo', 'doc',
                            '--target-dir', 'target/%s' % pac,
                            '--lib',
                            '--manifest-path', 'boards/%s/Cargo.toml' % crate])

def copy_to_docs_dir():
    """ Build out the doc tree from the cargo generated docs """
    if os.path.exists('doc'):
        shutil.rmtree('doc')
    os.makedirs('doc')
    for pac in crates_by_pac.keys():
        shutil.copytree('target/%s/thumbv6m-none-eabi/doc' % pac,
                        'doc/%s' % pac)

def generate_index_html(path):
    """ Generate a minimal directory listing index.html for child dirs """
    with open(os.path.join(path, 'index.html'), 'w') as f:
        f.write('<!DOCTYPE html>\n<html><head><meta charset="UTF-8">\n')
        f.write('<title>directory listing</title></head><body><ul>\n')
        for child in os.listdir(path):
            if os.path.isdir(os.path.join(path, child)):
                f.write('<li><a href="%s">%s</a></li>\n' % (child,child))
        f.write('</ul></body></html>\n')

def generate_all_index_html():
    """ Github pages doesn't auto generate directory listings, so generate
        index.html files for the subdirs we made """
    generate_index_html('doc')
    for pac in crates_by_pac.keys():
        generate_index_html('doc/%s' % pac)

def push_using_ghp_import():
    """ copy the doc dir to the gh_pages branch and push to github.
    You can `apt install ghp-import` on ubuntu, or pip install it elsewhere. """
    subprocess.call(['ghp-import', '-n', '-p', 'doc'])

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--push', action='store_true')
    args = parser.parse_args()

    generate_docs()
    copy_to_docs_dir()
    generate_all_index_html()

    if args.push:
        push_using_ghp_import()

if __name__ == '__main__':
    main()

