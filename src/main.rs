use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from_branch: String,

    #[arg(short, long)]
    into_branch: String,
}

fn main() {
    let Args { from_branch, into_branch } = Args::parse();
    let from_arch = read_arch(from_branch);
    let into_arch = read_arch(into_branch);
    let arch_diff = git_diff(from_arch, into_arch);
    println!("{}", arch_diff);
}