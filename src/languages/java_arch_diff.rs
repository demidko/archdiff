use std::fmt::Display;
use std::ops::Add;

use diffy::create_patch;
use git2::DiffLine;

use crate::languages::arch_diff::ArchDiff;

struct JavaArchDiff {
    filename: String,
    old_arch: String,
    new_arch: String,
    diff: String,
}

impl JavaArchDiff {
    fn flush(&mut self) {
        let old = &self.old_arch;
        let new = &self.new_arch;
        let diff = create_patch(old, new);
        let diff = diff.to_string();
        self.diff += &diff
    }
}

impl Display for JavaArchDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.diff)
    }
}

impl ArchDiff for JavaArchDiff {
    fn arch_diff(&mut self, filename: &str, diff: &DiffLine) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use diffy::{create_patch, DiffOptions};

    #[test]
    fn it_works() {
        let old = "ool\nkek\njey trek\n";
        let new = "lol\nkek\njey trep";
        let diff = create_patch(old, new);
        println!("{}", diff);
    }
}

