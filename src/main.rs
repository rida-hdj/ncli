// Copyright © 2026 rida-hdj <https://github.com/rida-hdj>

use owo_colors::OwoColorize;
use std::{io, io::Write};
mod commands;
fn main() {
    // ask user about his decide
    // update repos
    let repos_update = String::from("Update nixos repos? [yes/n]: ");
    print!("{}", repos_update.white());
    std::io::stdout().flush().unwrap(); //fix print! problem
    let repos_decide = take_input();
    // rebuild nixos
    let nixos_rebuild = String::from("Rebuild nixos? [yes/n]: ");
    print!("{}", nixos_rebuild.white());
    std::io::stdout().flush().unwrap();
    let rebuild_decide = take_input();
    // clean garbage
    let nixos_clean = String::from("Clean garbage? [yes/n]: ");
    print!("{}", nixos_clean.white());
    std::io::stdout().flush().unwrap();
    let clean_decide = take_input();
    // execute function depend on user choice
    if repos_decide == true {
        commands::repos();
    }

    if rebuild_decide == true {
        commands::rebuild();
    }

    if clean_decide == true {
        commands::clean_all();
    }
}

// take input as boolean
pub fn take_input() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "yes" | "y" | "1" => true,
        _ => false,
    }
}

// print status ( done / failed )
pub fn print_status(status: std::process::ExitStatus) {
    if status.success() {
        println!("{}", "done".green());
    } else {
        println!("{}", "failed".red());
    }
}
