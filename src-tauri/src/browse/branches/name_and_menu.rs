use crate::_backend_specific::git::executor::run_command;

mod utilities {

    use chrono::{Datelike, Timelike, Utc};
    use git2::{Oid, Repository};

    use crate::error::Result;

    fn generate_branch_name() -> String {
        let now = Utc::now();

        format!(
            "branch-{:04}-{:02}-{:02}-{:02}-{:02}-{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        )
    }

    pub fn create_branch(repo_path: String, start_point: Option<String>) -> Result<String> {
        let repo = Repository::open(&repo_path)?;
        let commit = match start_point {
            Some(start_point) => repo.find_commit(Oid::from_str(&start_point)?)?,
            None => {
                let head = match repo.head() {
                    Ok(head) => Ok(head),
                    Err(_) => Err("Repository HEAD Not Found"),
                }?;
                head.peel_to_commit()?
            }
        };

        let new_branch = generate_branch_name();

        repo.branch(&new_branch, &commit, false)?;

        Ok(new_branch)
    }
}

#[tauri::command]
pub fn create_branch(repo_path: String, start_point: Option<String>) -> Result<String, String> {
    let new_branch = match utilities::create_branch(repo_path, start_point) {
        Ok(new_branch) => new_branch,
        Err(error) => return Err(error.to_string()),
    };

    Ok(new_branch)
}

#[tauri::command]
pub fn merge_branch(repo_path: String, branch_name: String) -> Result<String, String> {
    match run_command(&repo_path, "merge", Some(&vec![&branch_name])) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
pub fn rebase_branch(repo_path: String, branch_name: String) -> Result<String, String> {
    match run_command(&repo_path, "rebase", Some(&vec![&branch_name])) {
        Ok(output) => Ok(output),
        Err(error) => Err(error.to_string()),
    }
}
