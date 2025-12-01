mod day01;


use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Problem {
    Day1A,
    Day1B,
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[arg(long, value_enum)]
    problem: Problem,

    #[arg(long)]
    input: std::path::PathBuf,
    dial_size: i32,
    starting_point: i32
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.problem {
        Problem::Day1A => {
            println!("{}", day01::part_a(&args.input, &args.dial_size, args.starting_point)?);
        }
        Problem::Day1B => {
            println!("{}", day01::part_b(&args.input, &args.dial_size, args.starting_point)?);
        }
    }
    Ok(())
}