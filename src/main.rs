mod mu_asm;

use clap::Parser;
use mu_asm::MuAsm;
use std::fs::File;
use std::fs::OpenOptions;
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
    let mut mu_asm = MuAsm::new();

    let mut reader: BufReader<Box<dyn Read>> = match cli.input_file {
        Some(input_file) => {
            let file = File::open(input_file)?;
            BufReader::new(Box::new(file))
        }
        None => {
            let io = io::stdin();
            BufReader::new(Box::new(io))
        }
    };

    let mut writer: Box<dyn Write> = match cli.output_file {
        Some(output_file) => {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(output_file)?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(BufWriter::new(io::stdout())),
    };

    if cli.disassemble {
        mu_asm.disassemble(&mut reader, &mut writer);
    } else {
        mu_asm.assemble(&mut reader, &mut writer);
    }

    Ok(())
}
