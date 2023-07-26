mod mu_asm;

use clap::Parser;
use mu_asm::MuAsm;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

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

    let reader: Box<dyn BufRead> = match cli.input_file {
        Some(input_file) => {
            let file = File::open(input_file)?;
            Box::new(BufReader::new(file))
        }
        None => Box::new(BufReader::new(io::stdin())),
    };

    let writer: Box<dyn Write> = match cli.output_file {
        Some(output_file) => {
            let file = File::open(output_file)?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };

    if cli.disassemble {
        mu_asm.disassemble(&reader, &writer);
    } else {
        mu_asm.disassemble(&reader, &writer);
    }

    Ok(())
}
