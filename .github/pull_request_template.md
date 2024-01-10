# Summary
[describe your changes here]

# Checklist
  - [ ] `CHANGELOG.md` for the BSP or HAL updated
  - [ ] All new or modified code is well documented, especially public items
  - [ ] No new warnings or clippy suggestions have been introduced - CI will **deny** clippy warnings by default! You may `#[allow]` certain lints where reasonable, but ideally justify those with a short comment. 

## If Adding a new Board
  - [ ] Board CI added to `crates.json`
  - [ ] Board is properly following "Tier 2" conventions, unless otherwise decided to be "Tier 1"

## If Adding a new cargo `feature` to the HAL
  - [ ] Feature is added to the test matrix for applicable boards / PACs in `crates.json`
