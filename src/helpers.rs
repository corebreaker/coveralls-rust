//! Small helpers shared across the crate.

use itertools::Itertools;
use std::{
    path::{Component, PathBuf},
    borrow::Cow,
};

/// Render a single path [`Component`] as a string, dropping the platform prefix.
///
/// The Windows path prefix (drive letter, UNC share, ...) is discarded so that the resulting paths
/// are platform-independent; the root directory becomes an empty segment.
fn path_component_as_str(c: Component) -> Option<Cow<str>> {
    match c {
        Component::Prefix(_) => None,
        Component::RootDir => Some(Cow::Borrowed("")),
        Component::CurDir => Some(Cow::Borrowed(".")),
        Component::ParentDir => Some(Cow::Borrowed("..")),
        Component::Normal(v) => Some(v.to_string_lossy()),
    }
}

/// Convert a path to a forward-slash separated string, regardless of the host platform.
///
/// This normalizes the file names reported to Coveralls so that a report produced on Windows uses
/// the same separators as one produced on Unix.
pub(super) fn path_to_string(path: &PathBuf) -> String {
    path.components().filter_map(path_component_as_str).join("/")
}
