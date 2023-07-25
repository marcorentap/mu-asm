mod mu_asm;

use mu_asm::assembler::assemble;
use std::io;
use std::io::BufRead;

fn main() {
    let handle = io::stdin().lock();
    let lines: Vec<String> = handle
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect();

    let bytes = assemble(&lines);
}
