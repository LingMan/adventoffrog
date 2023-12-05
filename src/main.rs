use anyhow::{bail, Context, Result};
use clap::Parser;

use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;

mod year2022;
mod year2023;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=31))]
    day: u8,

    #[arg(short, long, default_value_t = 2023)]
    #[arg(value_parser = clap::value_parser!(u32).range(2022..=2023))]
    year: u32,

    #[arg(long, default_value_t = false)]
    example: bool,

    #[arg(short, long, default_value = "input")]
    input_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.year {
        2022 => year_2022(args),
        2023 => year_2023(args),
        _ => bail!("Unknown year"),
    }
}

macro_rules! load {
    ($p:expr, $d:literal, $ex:expr) => {{
        let path = $p.join(format!("{}{}.txt", $d, if $ex { "_example" } else { "" }));
        &fs::read_to_string(&path).with_context(|| format!("Invalid path: {path:?}"))?
    }};
}

fn year_2022(args: Args) -> Result<()> {
    use crate::year2022::*;

    let path = args.input_path.join("2022");
    match args.day {
        1 => solve::<day01::Day01>(load!(path, "01", args.example)),
        _ => bail!("Unknown day"),
    }
}

fn year_2023(args: Args) -> Result<()> {
    use crate::year2023::*;

    let path = args.input_path.join("2023");
    match args.day {
        1 => solve::<day01::Day01>(load!(path, "01", args.example)),
        _ => bail!("Unknown day"),
    }
}

fn solve<'a, T: Puzzle<'a>>(input: &'a str) -> Result<()> {
    let start = std::time::Instant::now();
    let day_xy = T::parse(&input)?;
    let parse_time = std::time::Instant::now() - start;
    println!("Parsed in {parse_time:?})");

    let start = std::time::Instant::now();
    let solution_1 = day_xy.solve_problem_1();
    let s1_time = std::time::Instant::now() - start;
    println!("Solution 1 is: {solution_1:?} (Computed in {s1_time:?})");

    let start = std::time::Instant::now();
    let solution_2 = day_xy.solve_problem_2();
    let s2_time = std::time::Instant::now() - start;
    println!("Solution 2 is: {solution_2:?} (Computed in {s2_time:?})");
    println!("Total time: {:?}", parse_time + s1_time + s2_time);
    Ok(())
}

trait Puzzle<'a>: Sized {
    type Sol1Type: Debug;
    type Sol2Type: Debug;

    fn parse(input: &'a str) -> Result<Self>;
    fn solve_problem_1(&self) -> Self::Sol1Type;
    fn solve_problem_2(&self) -> Self::Sol2Type;
}
