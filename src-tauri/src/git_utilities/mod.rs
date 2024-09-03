use crate::{error::Result, git_executor::run};

pub struct BranchComparision {
    pub ahead: i32,
    pub _behind: i32,
}

pub fn compare_branches(
    repo_path: &String,
    branch1: &String,
    branch2: &String,
) -> Result<BranchComparision> {
    //
    let temp = format!("{}...{}", branch1, branch2);
    let output = run(
        repo_path,
        "rev-path",
        &vec!["--count", "--left-right", temp.as_str()],
    )?;

    let output: Vec<&str> = output.split_whitespace().collect();

    let ahead = output[0].parse::<i32>()?;
    let behind = output[1].parse::<i32>()?;

    Ok(BranchComparision {
        ahead,
        _behind: behind,
    })
}
