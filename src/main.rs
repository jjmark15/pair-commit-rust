// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
#[macro_use]
extern crate clap;

use crate::cli::init;

mod cli;
mod config;
mod persistence;

fn main() {
    init();
}
