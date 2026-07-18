#![allow(clippy::type_complexity)]
#![allow(clippy::ptr_arg)]

mod algorithm;
mod cli;
mod hasher;
mod output;
mod verifier;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
