//! Git metadata of a coverage report and its collection from the local repository.
//!
//! [`GitInfos`] is the serializable structure embedded in a [`Coverage`](crate::Coverage) report.
//! It is filled from the [`Config`](crate::config::Config) and, when needed, from the local
//! repository through the [`fetcher`] backend (either the `git` command or `libgit2`).

mod fetcher;
mod head;
mod infos;
mod remote;

pub use infos::GitInfos;
