use commit::Commit;
use git_diff::GitDiff;
use indoc::formatdoc;

pub mod commit;
pub mod git_diff;

#[derive(Debug, Clone)]
pub enum GitEntity {
    Commit(Commit),
    Diff(GitDiff),
}

impl GitEntity {
    pub fn format_static_details(&self) -> String {
        match self {
            GitEntity::Commit(commit) => formatdoc! {"
                # Entity: Commit
                `commit {hash}` | {author} <{email}> | {date}

                {message}
                -----",
                hash = commit.full_hash,
                author = commit.author_name,
                email = commit.author_email,
                date = commit.date,
                message = commit.message,
            },
            GitEntity::Diff(diff) => formatdoc! {"
                # Entity: Diff{staged}",
                staged = if diff.staged { " (staged)" } else { "" }
            },
        }
    }
}

impl AsRef<Commit> for GitEntity {
    fn as_ref(&self) -> &Commit {
        match self {
            GitEntity::Commit(commit) => commit,
            _ => panic!("Not a Commit"),
        }
    }
}

impl AsRef<GitDiff> for GitEntity {
    fn as_ref(&self) -> &GitDiff {
        match self {
            GitEntity::Diff(diff) => diff,
            _ => panic!("Not a Diff"),
        }
    }
}
