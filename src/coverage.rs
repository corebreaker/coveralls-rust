use super::git::GitInfos;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read, Result};

#[derive(Serialize, Deserialize)]
pub(crate) struct SourceFile {
    pub(crate) name: String,
    pub(crate) source_digest: String,
    pub(crate) coverage: Vec<Option<usize>>,

    #[serde(skip_serializing_if = "Vec::<usize>::is_empty")]
    #[serde(default)]
    pub(crate) branches: Vec<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) source: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Coverage {
    #[serde(default)]
    pub(crate) repo_token: String,

    #[serde(default)]
    pub(crate) service_name: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub(crate) service_number: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub(crate) service_job_id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub(crate) service_pull_request: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) flag_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) git: Option<GitInfos>,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub(crate) run_at: String,

    #[serde(default)]
    pub(crate) source_files: Vec<SourceFile>,
}

impl Coverage {
    pub fn from_reader<R: Read>(rdr: R) -> Result<Self> {
        Ok(serde_json::from_reader(rdr)?)
    }

    pub fn new_reader(&self) -> Result<Box<dyn Read>> {
        Ok(Box::new(Cursor::new(serde_json::to_string(&self)?)))
    }
}
