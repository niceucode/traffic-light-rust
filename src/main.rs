mod lib;

use clap::Clap;
use std::fs;
use crate::lib::input::Problem;
use crate::lib::solutions::Solution;
use crate::lib::solutions::fair_share::Solver;
use std::str::FromStr;

/// HashCode 2021 online qualification solution
#[derive(Clap)]
#[clap(version = "1.0", author = "Vsevolod S. <floatdrop@gmail.com>")]
struct Opts {
    /// Path to input file.
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let contents = fs::read_to_string(opts.input)
        .expect("Something went wrong reading the input file");
    let problem = Problem::from_str(&*contents).expect("Something went wrong parsing input file");
    let submission = Solver{problem}.solve();
    println!("{}", submission);
}
