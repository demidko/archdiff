use std::process::Command;
use std::str;

use git2::{BranchType, Repository};

struct MyGit {
    repo: Repository,
}

impl MyGit {
    pub fn new() -> Self {
        match Repository::init(".") {
            Ok(repo) => Self { repo },
            Err(e) => panic!("failed to init: {}", e)
        }
    }

    pub fn current_branch_name(&self) -> String {
        let repo = &self.repo;
        let head = repo.head().unwrap();
        let name = head.name().unwrap();
        let name = name.trim_start_matches("refs/heads/");
        name.to_string()
    }

    pub fn checkout(&self, branch_name: &str) {
        let repo = &self.repo;
    }
}

pub fn co(branch: &str) {
    let status =
        Command::new("git")
            .arg("checkout")
            .arg(branch)
            .status()
            .expect("failed to find 'git' command");
    assert!(status.success())
}

pub fn current_branch() -> String {
    let output =
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .expect("failed to find 'git' command")
            .stdout;
    str::from_utf8(&output)
        .expect("failed to find current branch")
        .trim()
        .to_string()
}

pub fn diff(from: &str, into: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::mygit::{current_branch, MyGit};

    #[test]
    fn it_works() {
        let branch = MyGit::new().current_branch_name();
        assert_eq!(branch, "main")
    }
}