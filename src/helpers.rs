use itertools::Itertools;
use std::{path::{Component, PathBuf}, borrow::Cow};

fn path_component_as_str(c: Component) -> Option<Cow<str>> {
    match c {
        Component::Prefix(_) => None,
        Component::RootDir => Some(Cow::Borrowed("")),
        Component::CurDir => Some(Cow::Borrowed(".")),
        Component::ParentDir => Some(Cow::Borrowed("..")),
        Component::Normal(v) => Some(v.to_string_lossy()),
    }
}

pub(super) fn path_to_string(path: &PathBuf) -> String {
    path.components().filter_map(path_component_as_str).join("/")
}
