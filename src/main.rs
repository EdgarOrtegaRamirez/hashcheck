mod algorithm;
mod cli;
mod hasher;
mod output;
mod verifier;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}