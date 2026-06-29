//! Backend-agnostic representation of the last commit read by a `GitFetcher`.

/// Information about a single commit, as returned by a `GitFetcher`.
///
/// Every field is optional because a backend may fail to provide it (for instance an empty commit
/// message). This is an intermediate, backend-agnostic structure that the Git `HEAD` information is
/// then filled from.
#[derive(Default)]
pub(in super::super) struct LogInfos {
    id:              Option<String>,
    author_name:     Option<String>,
    author_email:    Option<String>,
    committer_name:  Option<String>,
    committer_email: Option<String>,
    message:         Option<String>,
}

impl LogInfos {
    /// Build a `LogInfos` from the individual commit fields.
    pub(super) fn new(
        id: Option<String>,
        author_name: Option<String>,
        author_email: Option<String>,
        committer_name: Option<String>,
        committer_email: Option<String>,
        message: Option<String>,
    ) -> Self {
        Self {
            id,
            author_name,
            author_email,
            committer_name,
            committer_email,
            message,
        }
    }

    /// The commit identifier (SHA).
    pub(in super::super) fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// The commit author name.
    pub(in super::super) fn author_name(&self) -> Option<&str> {
        self.author_name.as_deref()
    }

    /// The commit author email.
    pub(in super::super) fn author_email(&self) -> Option<&str> {
        self.author_email.as_deref()
    }

    /// The commit committer name.
    pub(in super::super) fn committer_name(&self) -> Option<&str> {
        self.committer_name.as_deref()
    }

    /// The commit committer email.
    pub(in super::super) fn committer_email(&self) -> Option<&str> {
        self.committer_email.as_deref()
    }

    /// The commit message subject.
    pub(in super::super) fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
