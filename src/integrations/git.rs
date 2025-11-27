//! Git integration for MateriaTrack
//!
//! Auto-imports git commits made during tracking sessions.

use crate::config::Config;
use crate::error::Result;
use crate::models::Entry;
use chrono::{DateTime, Utc};
use git2::{Repository, Sort};
use std::path::{Path, PathBuf};

use super::{detect_git_repo, Integration};

pub struct GitIntegration {
    repos: Vec<PathBuf>,
}

impl GitIntegration {
    pub fn new() -> Self {
        Self { repos: Vec::new() }
    }

    pub fn with_repo(mut self, path: PathBuf) -> Self {
        self.repos.push(path);
        self
    }

    pub fn with_repos(mut self, paths: Vec<PathBuf>) -> Self {
        self.repos.extend(paths);
        self
    }

    pub fn add_repo(&mut self, path: PathBuf) {
        if !self.repos.contains(&path) {
            self.repos.push(path);
        }
    }

    pub fn detect_repo_from_cwd() -> Option<PathBuf> {
        std::env::current_dir()
            .ok()
            .and_then(|cwd| detect_git_repo(&cwd))
    }

    pub fn get_commits_in_range(
        &self,
        repo_path: &Path,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<Vec<GitCommit>> {
        let repo = Repository::open(repo_path)?;
        let mut revwalk = repo.revwalk()?;

        revwalk.set_sorting(Sort::TIME)?;
        revwalk.push_head()?;

        let since_ts = since.timestamp();
        let until_ts = until.timestamp();

        let mut commits = Vec::new();

        for oid in revwalk.flatten() {
            if let Ok(commit) = repo.find_commit(oid) {
                let commit_time = commit.time().seconds();

                if commit_time < since_ts {
                    break;
                }

                if commit_time <= until_ts && commit_time >= since_ts {
                    let git_commit = GitCommit {
                        hash: commit.id().to_string(),
                        short_hash: commit.id().to_string()[..7].to_string(),
                        message: commit.summary().unwrap_or("").to_string(),
                        full_message: commit.message().unwrap_or("").to_string(),
                        author: commit.author().name().unwrap_or("Unknown").to_string(),
                        author_email: commit.author().email().unwrap_or("").to_string(),
                        timestamp: DateTime::from_timestamp(commit_time, 0)
                            .unwrap_or_else(Utc::now),
                        repo_path: repo_path.to_path_buf(),
                    };
                    commits.push(git_commit);
                }
            }
        }

        commits.reverse();
        Ok(commits)
    }

    pub fn get_commits_for_entry(&self, entry: &Entry, repo_path: &Path) -> Result<Vec<GitCommit>> {
        let until = entry.end.unwrap_or_else(Utc::now);
        self.get_commits_in_range(repo_path, entry.start, until)
    }

    pub fn get_all_commits_for_entry(&self, entry: &Entry) -> Result<Vec<GitCommit>> {
        let mut all_commits = Vec::new();

        for repo_path in &self.repos {
            if let Ok(commits) = self.get_commits_for_entry(entry, repo_path) {
                all_commits.extend(commits);
            }
        }

        all_commits.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        Ok(all_commits)
    }

    pub fn format_commits_for_notes(commits: &[GitCommit]) -> String {
        if commits.is_empty() {
            return String::new();
        }

        let mut output = String::from("\n--- Git Commits ---\n");

        for commit in commits {
            output.push_str(&format!("• {} {}\n", commit.short_hash, commit.message));
        }

        output
    }

    pub fn format_commits_compact(commits: &[GitCommit]) -> Vec<String> {
        commits
            .iter()
            .map(|c| format!("{}: {}", c.short_hash, truncate(&c.message, 50)))
            .collect()
    }

    pub fn get_repo_info(repo_path: &Path) -> Result<RepoInfo> {
        let repo = Repository::open(repo_path)?;

        let head = repo.head()?;
        let branch = head
            .shorthand()
            .map(String::from)
            .unwrap_or_else(|| "HEAD".to_string());

        let remote_url = repo
            .find_remote("origin")
            .ok()
            .and_then(|r| r.url().map(String::from));

        let name = repo_path
            .file_name()
            .and_then(|n| n.to_str())
            .map(String::from)
            .unwrap_or_else(|| "unknown".to_string());

        Ok(RepoInfo {
            name,
            path: repo_path.to_path_buf(),
            branch,
            remote_url,
        })
    }

    pub fn has_uncommitted_changes(repo_path: &Path) -> Result<bool> {
        let repo = Repository::open(repo_path)?;
        let statuses = repo.statuses(None)?;

        Ok(!statuses.is_empty())
    }
}

impl Default for GitIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Integration for GitIntegration {
    fn name(&self) -> &'static str {
        "Git"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.tracking.auto_import_git
    }

    fn validate_config(&self, config: &Config) -> Result<()> {
        if config.tracking.auto_import_git && !config.tracking.git_repo_path.is_empty() {
            let path = super::expand_path(&config.tracking.git_repo_path)?;
            if !path.join(".git").exists() {
                return Err(crate::error::Error::Config(
                    crate::error::ConfigError::InvalidPath(format!(
                        "Not a git repository: {}",
                        path.display()
                    )),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub full_message: String,
    pub author: String,
    pub author_email: String,
    pub timestamp: DateTime<Utc>,
    pub repo_path: PathBuf,
}

impl GitCommit {
    pub fn format_short(&self) -> String {
        format!("{}: {}", self.short_hash, truncate(&self.message, 50))
    }

    pub fn format_full(&self) -> String {
        format!(
            "[{}] {} - {} <{}>\n{}",
            self.timestamp.format("%Y-%m-%d %H:%M"),
            self.short_hash,
            self.author,
            self.author_email,
            self.full_message
        )
    }
}

#[derive(Debug, Clone)]
pub struct RepoInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub remote_url: Option<String>,
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn auto_import_commits(entry: &mut Entry, config: &Config) -> Result<usize> {
    if !config.tracking.auto_import_git {
        return Ok(0);
    }

    let mut git = GitIntegration::new();

    if !config.tracking.git_repo_path.is_empty() {
        let path = super::expand_path(&config.tracking.git_repo_path)?;
        git.add_repo(path);
    }

    if let Some(cwd_repo) = GitIntegration::detect_repo_from_cwd() {
        git.add_repo(cwd_repo);
    }

    let commits = git.get_all_commits_for_entry(entry)?;

    if commits.is_empty() {
        return Ok(0);
    }

    let formatted = GitIntegration::format_commits_compact(&commits);
    entry.git_commits.extend(formatted.clone());

    Ok(commits.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a very long string", 10), "this is...");
    }

    #[test]
    fn test_git_integration_new() {
        let git = GitIntegration::new();
        assert!(git.repos.is_empty());
    }

    #[test]
    fn test_format_commits_empty() {
        let formatted = GitIntegration::format_commits_for_notes(&[]);
        assert!(formatted.is_empty());
    }
}
