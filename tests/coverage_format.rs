//! Integration tests for the public coverage report API ([`Coverage`]).
//!
//! These drive the crate as an external consumer would: parsing a Coveralls JSON report and
//! serializing it back, without any network access.

use coveralls::Coverage;
use std::io::Read;

/// Read everything produced by [`Coverage::new_reader`] into a string.
fn payload_of(coverage: &Coverage) -> String {
    let mut reader = coverage.new_reader().expect("serialize the coverage report");
    let mut buf = String::new();

    reader.read_to_string(&mut buf).expect("read the serialized payload");

    buf
}

#[test]
fn parsing_then_serializing_preserves_the_main_fields() {
    let report = r#"{
        "repo_token": "a-repo-token",
        "service_name": "circleci",
        "source_files": [
            {"name": "src/lib.rs", "source_digest": "deadbeef", "coverage": [1, null, 0]}
        ]
    }"#;

    let coverage = Coverage::from_reader(report.as_bytes()).expect("parse the report");
    let json: serde_json::Value = serde_json::from_str(&payload_of(&coverage)).expect("a valid JSON payload");

    assert_eq!(json["repo_token"], "a-repo-token");
    assert_eq!(json["service_name"], "circleci");
    assert_eq!(json["source_files"][0]["name"], "src/lib.rs");
    assert_eq!(json["source_files"][0]["source_digest"], "deadbeef");
    assert_eq!(json["source_files"][0]["coverage"][0], 1);
    assert!(json["source_files"][0]["coverage"][1].is_null());
    assert_eq!(json["source_files"][0]["coverage"][2], 0);
}

#[test]
fn git_metadata_is_exposed_when_present() {
    let report = r#"{
        "repo_token": "t",
        "service_name": "travis-ci",
        "git": {
            "head": {
                "id": "abc123",
                "author_name": "Ada",
                "author_email": "ada@example.com",
                "committer_name": "Ada",
                "committer_email": "ada@example.com",
                "message": "Initial commit"
            },
            "branch": "main",
            "remotes": [{"name": "origin", "url": "https://example.com/repo.git"}]
        },
        "source_files": []
    }"#;

    let coverage = Coverage::from_reader(report.as_bytes()).expect("parse the report");

    assert!(coverage.git().is_some(), "git metadata should be exposed");

    // The git object must survive the round-trip back to JSON.
    let json: serde_json::Value = serde_json::from_str(&payload_of(&coverage)).expect("a valid JSON payload");

    assert_eq!(json["git"]["head"]["id"], "abc123");
    assert_eq!(json["git"]["branch"], "main");
    assert_eq!(json["git"]["remotes"][0]["name"], "origin");
}

#[test]
fn git_metadata_is_absent_when_not_provided() {
    let report = r#"{"service_name": "jenkins", "source_files": []}"#;
    let coverage = Coverage::from_reader(report.as_bytes()).expect("parse the report");

    assert!(coverage.git().is_none(), "no git metadata should be exposed");
}

#[test]
fn invalid_json_is_rejected() {
    let result = Coverage::from_reader(b"this is not json".as_slice());

    assert!(result.is_err(), "invalid JSON must be rejected");
}
