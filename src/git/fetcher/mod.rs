//! Backends that read Git metadata from the local repository.
//!
//! Two interchangeable implementations of `GitFetcher` expose the same API ([`get_branch`],
//! [`get_log`] and [`get_remotes`]): one shelling out to the `git` command (default) and one using
//! [`git2`](https://docs.rs/git2) in-process (the `libgit` feature). Exactly one is compiled in
//! depending on the feature flag.
//!
//! [`get_branch`]: cmdgit::GitFetcher::get_branch
//! [`get_log`]: cmdgit::GitFetcher::get_log
//! [`get_remotes`]: cmdgit::GitFetcher::get_remotes

mod infos;

#[cfg(not(feature = "libgit"))]
mod cmdgit;

#[cfg(feature = "libgit")]
mod libgit;

pub(super) use infos::LogInfos;

#[cfg(not(feature = "libgit"))]
pub(super) use cmdgit::GitFetcher;

#[cfg(feature = "libgit")]
pub(super) use libgit::GitFetcher;
