mod mu_asm;

use clap::Parser;
use mu_asm::MuAsm;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    /// Disassemble input file. if unspecified, assemble input file
    #[arg(short, long)]
    disassemble: bool,

    /// Input file. If unspecified, read from stdin
    #[arg(short, long)]
    input_file: Option<String>,

    /// Output file. If unspecified, write to stdout
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mu_asm = MuAsm::new();

    let lines: Vec<String> = match cli.input_file {
        Some(input_file) => {
            let file = File::open(input_file)?;
            let reader = BufReader::new(file);
            reader
                .lines()
                .into_iter()
                .map(|line| line.unwrap())
                .collect()
        }
        None => {
            let handle = io::stdin().lock();
            handle
                .lines()
                .into_iter()
                .map(|line| line.unwrap())
                .collect()
        }
    };

    if cli.disassemble {
        mu_asm.disassemble(&lines);
    } else {
        mu_asm.assemble(&lines);
    }

    // let handle = io::stdin().lock();
    // let lines: Vec<String> = handle
    //     .lines()
    //     .into_iter()
    //     .map(|line| line.unwrap())
    //     .collect();

    // let mu_asm = MuAsm::new();
    // mu_asm.assemble(&lines);

    Ok(())
}
