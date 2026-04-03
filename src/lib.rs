//! Library surface for the `temprs` / `tp` stack utilities.
//!
//! User-facing CLI and development workflow (including CI parity commands and where
//! `clap` / `apply_permutation` tests live) are documented in the repository README.
//! Test totals are not documented here; run `cargo test --locked` and read each `test result:` line.

pub mod model;
pub mod util;
