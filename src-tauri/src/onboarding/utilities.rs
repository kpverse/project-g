use crate::error::Result;
use git2::Repository;
use serde::Serialize;
use std::{
    fs::File,
    io::Write,
    path::Path,
    process::{Command, Output},
};

#[derive(Serialize)]
pub struct RemoteOrigin {
    fetch: Option<String>,
    push: Option<String>,
}

pub fn get_remote_origin(repo_path: String) -> Result<RemoteOrigin> {
    let repo = Repository::open(repo_path)?;

    let must_have_remote = "origin";

    let remote_origin = repo.find_remote(&must_have_remote)?;

    let fetch = match remote_origin.url() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    let push = match remote_origin.pushurl() {
        Some(url) => Some(url.to_string()),
        None => None,
    };

    Ok(RemoteOrigin { fetch, push })
}

pub fn get_origin_head(repo_path: String) -> Result<String> {
    let remote_name = "origin";

    let remote_head_path = format!("{}/.git/refs/remotes/{}/HEAD", repo_path, remote_name);
    let remote_head_path = Path::new(&remote_head_path);

    if remote_head_path.exists() && remote_head_path.is_file() {
        return Ok("origin/HEAD".to_string());
    }

    let Output {
        status,
        stdout,
        stderr: _,
    } = Command::new("git")
        .arg("remote")
        .arg("show")
        .arg("origin")
        .current_dir(repo_path)
        .output()?;

    if !status.success() || stdout.is_empty() {
        return Err("Cannot find \"origin/HEAD\"".into());
    }

    let stdout = String::from_utf8(stdout)?;
    let mut branch_name: Option<&str> = None;
    let identifier = "HEAD branch:";

    for line in stdout.lines() {
        match line.find(&identifier) {
            Some(position) => {
                branch_name = Some(&line[position + identifier.len()..].trim());
                break;
            }
            None => continue,
        }
    }

    let branch_name = match branch_name {
        Some(branch_name) => branch_name,
        None => return Err("Cannot find default branch of remote origin.".into()),
    };

    let mut origin_head_file = File::create(remote_head_path)?;

    origin_head_file.write_all(format!("ref: refs/remotes/origin/{}", branch_name).as_bytes())?;

    Ok("origin/HEAD".to_string())
}
