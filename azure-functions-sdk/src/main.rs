#![feature(rust_2018_preview)]
#![feature(in_band_lifetimes)]
//#![deny(missing_docs)]
#![deny(unused_extern_crates)]

extern crate atty;
extern crate clap;
extern crate colored;
extern crate handlebars;
#[macro_use]
extern crate serde_json;

mod commands;

use clap::{App, AppSettings};
use colored::Colorize;
use commands::NewApp;
use std::env;
use std::process;

pub fn create_app() -> App<'a, 'b> {
    App::new("Azure Functions for Rust")
        .bin_name("cargo func")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Azure Functions for Rust Developer Tools")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::NoBinaryName)
        .subcommand(NewApp::create_subcommand())
}

pub fn print_running(message: &str) {
    print!("{} {}", "️🚀".cyan(), message);
}

pub fn print_success() {
    println!(" {}", "✓".green());
}

pub fn print_failure() {
    println!(" {}", "✗".red());
}

fn print_error_and_exit(message: &str) {
    eprintln!("{}: {}", "error".red().bold(), message);
    process::exit(1);
}

fn main() {
    // Support both cargo-func <command> and cargo-func func <command>
    // This enables running both `cargo-func` and `cargo func`, which passes the `func` command down
    let mut matches = None;
    if let Some(first) = env::args().nth(1) {
        if first == "func" {
            matches = Some(create_app().get_matches_from(env::args().skip(2)));
        }
    }

    let matches = matches.get_or_insert_with(|| create_app().get_matches_from(env::args().skip(1)));

    if let Some(args) = matches.subcommand_matches("new-app") {
        let command = NewApp::from_args(args);
        if let Err(e) = command.execute() {
            print_error_and_exit(&e);
        }
        return;
    }

    panic!("expected a subcommand.");
}
