//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

extern crate alloc;

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;

fn main() -> sc_cli::Result<()> {
    command::run()
}
