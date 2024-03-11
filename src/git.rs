use std::process::Command;

pub fn co(branch: &str) {
    let status =
        Command::new("git")
            .arg("checkout")
            .arg(branch)
            .status()
            .expect("failed to find git");
    assert!(status.success())
}

pub fn current_branch() -> String {
    let output =
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .expect("failed to find git")
            .stdout;
    String::from_utf8(output).expect("failed to find current branch")
}

pub fn diff(from: &str, into: &str) -> String {
    todo!()
}