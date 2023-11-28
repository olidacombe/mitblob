use color_eyre::Result;

use super::split_owner_repo;

pub async fn latest_commit(repo: &str, branch: &str) -> Result<String> {
    let (owner, repo) = split_owner_repo(repo)?;
    let commits = octocrab::instance()
        .repos(owner, repo)
        .list_commits()
        .send()
        .await?;
    // TODO not panic unwrap
    let latest = commits.items.first().unwrap();
    Ok(latest.sha.clone())
}
