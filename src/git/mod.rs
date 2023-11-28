use std::fmt;

use regex::Regex;
use serde::de::{Deserialize, Deserializer, Error, Visitor};

mod github;
mod gitlab;

#[derive(Clone, Debug, PartialEq)]
pub enum Repo {
    GitHub(String),
    GitLab(String),
}

impl Repo {
    pub fn latest_commit(&self, branch: &str) -> String {
        match self {
            Self::GitHub(repo) => github::latest_commit(repo, branch),
            Self::GitLab(repo) => gitlab::latest_commit(repo, branch),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GitRepoSpecError {
    #[error("Invalid git repo URL")]
    Invalid,
}

fn split_git_repo_url(url: &str) -> Result<(String, String), GitRepoSpecError> {
    let re = Regex::new(r"(https?:\/\/|git@)(?<provider>[^\/]+)\/(?<path>.*)").unwrap();
    let caps = re.captures(url).ok_or(GitRepoSpecError::Invalid)?;
    Ok((caps["provider"].to_string(), caps["path"].to_string()))
}

impl<'de> Deserialize<'de> for Repo {
    fn deserialize<D>(deserializer: D) -> Result<Repo, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Repo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("supported git repo url")
            }

            fn visit_str<E>(self, value: &str) -> Result<Repo, E>
            where
                E: Error,
            {
                let Ok((provider, path)) = split_git_repo_url(value) else {
                    return Err(Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &"A supported git repo url",
                    ));
                };
                match provider.as_str() {
                    "github.com" => Ok(Self::Value::GitHub(path)),
                    "gitlab.com" => Ok(Self::Value::GitLab(path)),
                    _ => Err(Error::unknown_variant(
                        provider.as_str(),
                        &["github.com", "gitlab.com"],
                    )),
                }
            }
        }

        deserializer.deserialize_string(FieldVisitor {})
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use color_eyre::Result;

    #[test]
    fn split_github_repo_https() -> Result<()> {
        let (provider, path) = split_git_repo_url("https://github.com/olidacombe/mitblob")?;
        assert_eq!(provider, "github.com".to_string());
        assert_eq!(path, "olidacombe/mitblob".to_string());
        Ok(())
    }

    #[test]
    fn split_gitlab_repo_https() -> Result<()> {
        let (provider, path) = split_git_repo_url("https://gitlab.com/olidacombe/mitblob")?;
        assert_eq!(provider, "gitlab.com".to_string());
        assert_eq!(path, "olidacombe/mitblob".to_string());
        Ok(())
    }

    #[test]
    fn deserialize_github_repo_https() -> Result<()> {
        let deserialized: Repo = serde_json::from_str("\"https://github.com/olidacombe/mitblob\"")?;
        assert_eq!(deserialized, Repo::GitHub("olidacombe/mitblob".to_string()));
        Ok(())
    }

    #[test]
    fn deserialize_gitlab_repo_https() -> Result<()> {
        let deserialized: Repo = serde_json::from_str("\"https://gitlab.com/olidacombe/mitblob\"")?;
        assert_eq!(deserialized, Repo::GitLab("olidacombe/mitblob".to_string()));
        Ok(())
    }
}
