mod mu_asm;

use mu_asm::MuAsm;
use std::io;
use std::io::BufRead;

fn main() {
    let handle = io::stdin().lock();
    let lines: Vec<String> = handle
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect();

    let mu_asm = MuAsm::new();
    mu_asm.assemble(&lines);
}
