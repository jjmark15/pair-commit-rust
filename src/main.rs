// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
#[macro_use]
extern crate clap;

use crate::cli::init;

pub mod persistence;
mod cli;
mod config;

fn main() {
    init();
}
