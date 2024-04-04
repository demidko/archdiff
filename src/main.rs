use std::env::args;
use std::fmt::Debug;
use std::ops::Not;
use std::str;

use DiffFormat::Patch;
use DiffLineType::{AddEOFNL, Addition, Binary, Context, ContextEOFNL, DeleteEOFNL, Deletion, FileHeader, HunkHeader};
use git2::{Diff, DiffDelta, DiffFindOptions, DiffFormat, DiffHunk, DiffLine, DiffLineType, Object, ObjectType, Repository};
use similar::DiffableStr;

fn main() {
    let help = "Usage: archdiff [OLD_BRANCH] [NEW_BRANCH]";
    let old_branch = args().nth(1).expect(help);
    let new_branch = args().nth(2).expect(help);
    let repo = open_current_repo();
    diff_branches(&repo, &old_branch, &new_branch)
        .print(Patch, print_diff_line)
        .unwrap();
}

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

pub fn print_diff_line(delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine) -> bool {
    let diff_type = line.origin_value();

    if is_unsupported_file(&delta) {
        return true;
    }
    if is_unsupported_diff(diff_type) {
        return true;
    }

    let line = line.content();
    let line = str::from_utf8(line).unwrap();

    if diff_type == FileHeader {
        print!("{}", line);
        return true;
    }
    if diff_type == HunkHeader {
        print!("{}", trim_hunk_header(line));
        return true;
    }
    if is_java_api(line) {
        println!("{}{}", prefix(diff_type), trim_java_api(line));
    }
    true
}

fn prefix(diff: DiffLineType) -> char {
    match diff {
        Context => ' ',
        Addition => '+',
        Deletion => '-',
        _ => panic!("Unexpected type here: {:?}", diff)
    }
}

fn trim_hunk_header(line: &str) -> &str {
    if line.starts_with("@@") {
        let line = line.trim_start_matches("@@");
        let idx = line.find("@@").unwrap() + 2;
        return &line[idx..];
    }
    line
}

fn is_java_api(line: &str) -> bool {
    let line = line.trim_start();
    if line.starts_with("public") {
        return true;
    }
    if line.starts_with("class") {
        return true;
    }
    false
}

fn trim_java_api(line: &str) -> &str {
    line.trim_end_matches(" {\n")
}

fn is_unsupported_file(delta: &DiffDelta) -> bool {
    let new_file = delta.new_file().path();
    let old_file = delta.old_file().path();
    let extension =
        new_file.or(old_file).unwrap()
            .extension().unwrap()
            .to_str().unwrap();
    extension != "java"
}

fn is_unsupported_diff(diff: DiffLineType) -> bool {
    match diff {
        ContextEOFNL => true,
        AddEOFNL => true,
        DeleteEOFNL => true,
        Binary => true,
        _ => false
    }
}

fn make_tree_object<'a>(repo: &'a Repository, arg: &str) -> Object<'a> {
    let obj = repo.revparse_single(arg).unwrap();
    let tree_object = obj.peel(ObjectType::Tree);
    tree_object.unwrap()
}

#[cfg(test)]
mod tests {
    use crate::trim_hunk_header;

    #[test]
    fn trim_hunk_test() {
        let header = "@@ -4,7 +4,7 @@ class Test {";
        let header = trim_hunk_header(&header);
        assert_eq!(header, " class Test {")
    }
}