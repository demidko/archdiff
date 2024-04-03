use std::collections::hash_set::Difference;
use std::fmt::Debug;
use std::path::Path;
use std::str;

use git2::{Diff, DiffDelta, DiffFindOptions, DiffHunk, DiffLine, Object, ObjectType, Repository};

pub fn open_current_repo() -> Repository {
    match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open repo: {}", e.message())
    }
}

pub fn diff_branches<'a>(repo: &'a Repository, old_branch: &str, new_branch: &str) -> Diff<'a> {
    let old_obj = make_tree_object(repo, old_branch);
    let old_tree = old_obj.as_tree();
    let new_obj = make_tree_object(repo, new_branch);
    let new_tree = new_obj.as_tree();
    let diff = repo.diff_tree_to_tree(old_tree, new_tree, None);
    let mut opts = DiffFindOptions::new();
    opts.renames(true);
    opts.copies(true);
    let mut diff = diff.unwrap();
    let diff_ref = Some(&mut opts);
    diff.find_similar(diff_ref).unwrap();
    diff
}

pub fn print_diff_line(delta: DiffDelta, hunk: Option<DiffHunk>, line: DiffLine) -> bool {
    println!();
    println!("{:?} {:?}", delta.old_file().path(), delta.new_file().path());
    println!("{:?}", delta.status());
    println!("{:?} {:?}", line.old_lineno(), line.new_lineno());
    println!("{:?}", line.origin_value());

    //println!("{} {} {} {}", hunk.old_start(), hunk.old_lines(), hunk.new_start(), hunk.new_lines());
    print!("{}", str::from_utf8(line.content()).unwrap());
    true
}

fn extract_path<'a>(diff_delta: &'a DiffDelta) -> &'a Path {
    let new_file = diff_delta.new_file().path();
    let old_file = diff_delta.old_file().path();
    new_file.or(old_file).unwrap()
}

fn make_tree_object<'a>(repo: &'a Repository, arg: &str) -> Object<'a> {
    let obj = repo.revparse_single(arg).unwrap();
    let tree_object = obj.peel(ObjectType::Tree);
    tree_object.unwrap()
}

#[cfg(test)]
mod tests {
    use git2::DiffFormat;

    use crate::git::{diff_branches, open_current_repo, print_diff_line};

    #[test]
    fn it_works() {
        let repo = open_current_repo();
        let diff = diff_branches(&repo,  "test_branch", "main");
        let diff_format = DiffFormat::Patch;
        diff.print(DiffFormat::Patch, print_diff_line).unwrap();
    }
}