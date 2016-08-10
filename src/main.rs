#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

use std::env;

mod cli;
mod crypto_utils;
mod io;
mod vault;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Default command shows help page
    if args.len() < 2 {
        cli::help();
    }

    let commands = cli::get_commands();

    // TODO: Handle subarguments better
    for (i, a) in args[1..].iter().enumerate() {
        match a.as_ref() {
            "-h" | "--help" => cli::help(),
            "usage" => cli::usage(),
            "init" => vault::init(),
            "ls" => vault::list(),
            "add" => {
                let command = commands.get("add").unwrap();
                if cli::check_args(command, &args[2..]).is_err() { return }
                vault::add(&args[i+2]);
            },
            "rm" => {
                let command = commands.get("rm").unwrap();
                if cli::check_args(command, &args[2..]).is_err() { return }
                vault::remove(&args[i+2]);
            }
            "show" => {
                let command = commands.get("show").unwrap();
                if cli::check_args(command, &args[2..]).is_err() { return }
                vault::show(&args[i+2]);
            }
            _ => {
                // Handle unrecognized args and opts
                let mut type_ = "argument";

                if a.starts_with("-") || a.starts_with("--") {
                    type_ = "option";
                }

                println!("Unrecognized {} {}\n", type_, &a);
                cli::usage();
            }
        }
        return
    }
}
