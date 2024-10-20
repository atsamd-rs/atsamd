# Summary
[describe your changes here]

# Checklist
  - [ ] All new or modified code is well documented, especially public items
  - [ ] No new warnings or clippy suggestions have been introduced - CI will **deny** clippy warnings by default! You may `#[allow]` certain lints where reasonable, but ideally justify those with a short comment. 

## If Adding a new Board
  - [ ] Board CI added to `crates.json`
  - [ ] Board is properly following "Tier 2" conventions, unless otherwise decided to be "Tier 1"

## If Adding a new cargo `feature` to the HAL
  - [ ] Feature is added to the test matrix for applicable boards / PACs in `crates.json`

#### Note
The crate changelogs **should no longer** be manually updated! Changelogs are now automatically generated. Instead:

- If your PR is contained to a single crate, or a single feature:
  - Nothing else to do; your PR will likely be squashed down to a single commit.
  - Please consider using [conventional commmit phrasing](https://www.conventionalcommits.org) in the PR title.
- If your PR brings in large, sweeping changes across multiple crates:
  - Organize your commits such that each commit only touches a single crate, or a single feature across multiple crates. Please don't create commits that span multiple features over multiple crates.
  - Use [conventional commmits](https://www.conventionalcommits.org) for your commit messages.
