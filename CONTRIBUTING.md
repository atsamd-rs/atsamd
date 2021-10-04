### Adding a new BSP/PAC

The build and release workflows will automatically recognize any new crate added to the repo,
but you will need to add some metadata to the crates's `Cargo.toml`.


```toml
# example of metadata for a BSP
[package.metadata.atsamd]
# also_build (optional): indicate if any additional crates (HAL | PAC | BSP's) should be built
# to properly test this crate
# also_build = ["crate", "crate2"]

[package.metadata.atsamd.bsp]

# tier (required)
# see: https://github.com/atsamd-rs/atsamd#how-to-use-a-bsp-ie-getting-started-writing-your-own-code
# for determining what tier your BSP is
tier = 2

# build_command (required)
# used to invoke the appropriate build command for the bsp
build_command = "cargo build --examples --features=unproven"

[package.metadata.atsamd.pac]
# target (required): the build target, used to configure the runner properly
target = "thumbv6m-none-eabi"
# features to build the pac with
features = ["samd21g", "unproven", "usb"]
```
