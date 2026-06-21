use owo_colors::OwoColorize;
use std::{io, process::Command};

fn main() {
    // ask user about his decide
    // update repos
    let repos_update = String::from("Update nixos repos? [1/0]:");
    println!("{}", repos_update.white());
    let repos_decide = take_input();
    // rebuild nixos
    let nixos_rebuild = String::from("Rebuild nixos? [1/0]:");
    println!("{}", nixos_rebuild.white());
    let rebuild_decide = take_input();
    // clean garbage
    let nixos_clean = String::from("Clean garbage? [1/0]:");
    println!("{}", nixos_clean.white());
    let clean_decide = take_input();
    // execute function depend on user choice
    if repos_decide == 1 {
        repos();
    }

    if rebuild_decide == 1 {
        rebuild();
    }

    if clean_decide == 1 {
        clean_all();
    }
}
// take input as integer
fn take_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap_or(0);
    // error handling by make any invalide input 0 wich cancel it
    if input > 1 {
        0
    } else {
        input
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
