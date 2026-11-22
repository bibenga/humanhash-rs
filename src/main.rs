use std::error::Error;
use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use humanhash::Uuid;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Format {
    Uuid,
    Text,
}

#[derive(Debug, Parser)]
#[command(
    name = "humanize",
    version,
    after_help = "Example commands:\n  \
                  $ humanize --value=938AEF34-912E-4729-8C30-7DC347504688\n  \
                  $ humanize --value=123456 --format=text"
)]
struct Cli {
    #[arg(short, long)]
    value: Option<String>,

    #[arg(short, long, value_enum, default_value_t = Format::Uuid)]
    format: Format,
}

fn run(cli: &Cli) -> Result<String, Box<dyn Error>> {
    let humanized = match cli.value.as_deref() {
        None => humanhash::new_uuid4()?.1,
        Some(value) => match cli.format {
            Format::Uuid => humanhash::humanize(Uuid::parse_str(value)?.as_bytes())?,
            Format::Text => humanhash::humanize(value.as_bytes())?,
        },
    };
    Ok(humanized)
}

fn main() -> ExitCode {
    // cargo run --features cli
    let cli = Cli::parse();
    match run(&cli) {
        Ok(humanized) => {
            println!("{humanized}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
