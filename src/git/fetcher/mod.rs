#[cfg(not(feature = "use-libgit"))]
mod cmdgit;

#[cfg(feature = "use-libgit")]
mod libgit;

#[cfg(not(feature = "use-libgit"))]
pub(super) use cmdgit::GitFetcher;

#[cfg(feature = "use-libgit")]
pub(super) use libgit::GitFetcher;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) enum LogValueKind {
    Id,
    AuthorName,
    AuthorEmail,
    CommitterName,
    CommitterEmail,
    Message,
}

impl LogValueKind {
    #[cfg(not(feature = "use-libgit"))]
    fn to_format_str(&self) -> &'static str {
        match self {
            Self::Id => "H",
            Self::AuthorName => "aN",
            Self::AuthorEmail => "ae",
            Self::CommitterName => "cN",
            Self::CommitterEmail => "ce",
            Self::Message => "s",
        }
    }
}