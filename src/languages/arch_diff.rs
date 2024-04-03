use std::fmt::Display;

use git2::DiffLine;

pub trait ArchDiff: Display {
    fn arch_diff(&mut self, filename: &str, diff: &DiffLine);
}