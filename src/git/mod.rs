mod github;

pub enum Repo {
    GitHub(String),
}

impl Repo {
    pub fn latest_commit(&self, branch: &str) -> String {
        match self {
            Self::GitHub(repo) => github::latest_commit(repo, branch),
        }
    }
}
