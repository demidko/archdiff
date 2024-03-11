use std::env::args;

use clap::Parser;

mod git;
mod arch;

fn main() {
    let from_branch = args().nth(1).expect("expected source branch name");
    let into_branch = git::current_branch();

    git::co(&from_branch);
    let from_branch = arch::read();

    git::co(&into_branch);
    let into_branch = arch::read();

    println!("{}", git::diff(&from_branch, &into_branch));
}