use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub last_commit_date: DateTime<Local>,
    pub is_current: bool,
}

pub fn is_git_repo(path: &Path) -> Result<bool> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output()?;

    Ok(output.status.success())
}

pub fn get_current_branch(repo_path: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .arg("--show-current")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to get current branch"));
    }

    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn get_branches(repo_path: &Path) -> Result<Vec<Branch>> {
    let current_branch = get_current_branch(repo_path)?;

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("for-each-ref")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)|%(committerdate:iso-strict)")
        .arg("refs/heads")
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to get branches"));
    }

    let stdout = String::from_utf8(output.stdout)?;
    let mut branches = Vec::new();

    for line in stdout.lines() {
        if let Some((name, date_str)) = line.split_once('|') {
            let name = name.to_string();
            let is_current = name == current_branch;
            let last_commit_date = DateTime::parse_from_rfc3339(date_str)?
                .with_timezone(&Local);

            branches.push(Branch {
                name,
                last_commit_date,
                is_current,
            });
        }
    }

    Ok(branches)
}

pub fn delete_branch(repo_path: &Path, branch_name: &str) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("branch")
        .arg("-d")
        .arg(branch_name)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("not fully merged") {
            let output_force = Command::new("git")
                .arg("-C")
                .arg(repo_path)
                .arg("branch")
                .arg("-D")
                .arg(branch_name)
                .output()?;

            if !output_force.status.success() {
                return Err(anyhow!("Failed to delete branch {}: {}", branch_name,
                    String::from_utf8_lossy(&output_force.stderr)));
            }
        } else {
            return Err(anyhow!("Failed to delete branch {}: {}", branch_name, stderr));
        }
    }

    Ok(())
}