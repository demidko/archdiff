use std::env::args;

mod mygit;
mod arch;

fn main() {
    let from_branch = args().nth(1).expect("expected source branch name");
    let into_branch = mygit::current_branch();

    mygit::co(&from_branch);
    let from_branch = arch::read();

    mygit::co(&into_branch);
    let into_branch = arch::read();

    println!("{}", mygit::diff(&from_branch, &into_branch));
}