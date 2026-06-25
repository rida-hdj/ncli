// Copyright © 2026 rida-hdj <https://github.com/rida-hdj>

use std::process::Command;
use crate::print_status;

// update repos
pub fn repos() {
    let status = Command::new("sudo")
        .args(["nix", "flake", "update", "--flake", "/etc/nixos/"])
        .status()
        .expect("failed to run command");
    crate::print_status(status);
}
// rebuild nixos
pub fn rebuild() {
    let status = Command::new("sudo")
        .args(["nixos-rebuild", "switch"])
        .status()
        .expect("failed to run command");
    crate::print_status(status);
}
// merge two functions
pub fn clean_all() {
    clean();
    optimise();
}
// clean nixos garbage (old generation)
pub fn clean() {
    let status = Command::new("sudo")
        .args(["nix-collect-garbage", "-d"])
        .status()
        .expect("failed to run command");
    crate::print_status(status);
}
// optimise nix store
pub fn optimise() {
    let status = Command::new("nix")
        .args(["store", "optimise"])
        .status()
        .expect("failed to run command");
    print_status(status);
}
