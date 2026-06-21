use owo_colors::OwoColorize;
use std::{io, process::Command, io::Write};

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
        repos();
    }

    if rebuild_decide == true {
        rebuild();
    }

    if clean_decide == true {
        clean_all();
    }
}
// take input as integer
fn take_input() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase();

    match input.as_str() {
        "yes" | "y" | "1" => true,
        _ => false,
    }
}
// print status ( done / failed )
fn print_status(status: std::process::ExitStatus) {
    if status.success() {
        println!("{}", "done".green());
    } else {
        println!("{}", "failed".red());
    }
}
// update repos
fn repos() {
    let status = Command::new("sudo")
        .args(["nix", "flake", "update", "--flake", "/etc/nixos/"])
        .status()
        .expect("failed to run command");
    print_status(status);
}
// rebuild nixos
fn rebuild() {
    let status = Command::new("sudo")
        .args(["nixos-rebuild", "switch"])
        .status()
        .expect("failed to run command");
    print_status(status);
}
// merge two functions
fn clean_all() {
    clean();
    optimise();
}
// clean nixos garbage (old generation)
fn clean() {
    let status = Command::new("sudo")
        .args(["nix-collect-garbage", "-d"])
        .status()
        .expect("failed to run command");
    print_status(status);
}
// optimise nix store
fn optimise() {
    let status = Command::new("nix")
        .args(["store", "optimise"])
        .status()
        .expect("failed to run command");
    print_status(status);
}
