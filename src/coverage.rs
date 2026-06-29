use super::git::GitInfos;
use serde::{Deserialize, Serialize};
use log::{debug, trace};
use std::io::{Cursor, Read, Result};

/// A single source file entry of a coverage report.
///
/// This mirrors the `source_files` objects of the Coveralls JSON format: the file `name`, the
/// digest of its contents and, for every line, the hit count (`None` for lines that are not
/// relevant to coverage). The optional `branches` and `source` fields are kept as they are when
/// present in the input.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SourceFile {
    pub(crate) name:          String,
    pub(crate) source_digest: String,
    pub(crate) coverage:      Vec<Option<usize>>,

    #[serde(skip_serializing_if = "Vec::<usize>::is_empty")]
    #[serde(default)]
    pub(crate) branches: Vec<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub(crate) source: Option<String>,
}

/// A coverage report, deserialized from and serialized to the Coveralls JSON format.
///
/// A `Coverage` is the in-memory representation of a Coveralls job: the repository token, the CI
/// service identifiers, the optional Git metadata and the list of covered source files. It is
/// parsed from a reader with [`Coverage::from_reader`], enriched in place by a
/// [`CoverallsManager`](crate::CoverallsManager), and finally serialized back for upload with
/// [`Coverage::new_reader`].
///
/// Fields that are empty or absent are skipped during serialization so that the produced JSON
/// stays close to what the Coveralls API expects.
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
    /// Parse a coverage report in the Coveralls JSON format from a reader.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if reading fails or if the data is not valid Coveralls JSON.
    pub fn from_reader<R: Read>(rdr: R) -> Result<Self> {
        let coverage: Coverage = serde_json::from_reader(rdr)?;
        debug!(
            "Parsed coverage report with {} source file(s)",
            coverage.source_files.len()
        );

        Ok(coverage)
    }

    /// Serialize the report to JSON and return a reader over the produced bytes.
    ///
    /// This is what gets uploaded to the Coveralls API as the `json_file` part of the request.
    ///
    /// # Errors
    ///
    /// Returns an [`std::io::Error`] if the report cannot be serialized to JSON.
    pub fn new_reader(&self) -> Result<Box<dyn Read>> {
        let json = serde_json::to_string(&self)?;

        trace!("Serialized coverage payload ({} bytes)", json.len());
        Ok(Box::new(Cursor::new(json)))
    }

    /// Return the Git metadata attached to the report, if any.
    pub fn git(&self) -> Option<&GitInfos> {
        self.git.as_ref()
    }
}
