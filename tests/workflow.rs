//! End-to-end integration test of the documented library workflow.
//!
//! It mirrors the "Library usage" section of the crate documentation: build a [`Config`] from the
//! environment, parse a [`Coverage`] report, enrich it with a [`CoverallsManager`] and serialize
//! the resulting payload — all without uploading anything.
//!
//! The configuration is driven through environment variables, so this binary deliberately contains
//! a single test: integration test binaries run their tests on several threads, and mutating the
//! process environment from more than one test at a time would be racy. The Git metadata is fetched
//! from this crate's own repository (the working directory of the test), so the test assumes it runs
//! from a checkout with `git` available.

use coveralls::{Config, Coverage, CoverallsManager, Env};
use std::io::Read;

#[test]
fn library_workflow_enriches_and_serializes_the_report() {
    // SAFETY: this is the only test in this binary, so no other thread accesses the environment
    // concurrently. `CI_NAME` is the first source checked by `load_from_environment`, so it selects
    // the service regardless of the ambient CI variables.
    unsafe {
        std::env::set_var("CI_NAME", "circleci");
        std::env::set_var("COVERALLS_REPO_TOKEN", "integration-token");
    }

    let env = Env::new();
    let config = Config::load_from_environment(&env)
        .expect("loading the configuration should succeed")
        .expect("CI_NAME should select a service");

    let mut coverage =
        Coverage::from_reader(r#"{"source_files": []}"#.as_bytes()).expect("parse the coverage report");

    let manager = CoverallsManager::new();

    manager
        .apply_config(&config, &mut coverage, false)
        .expect("applying the configuration should succeed");

    let mut payload = String::new();
    coverage
        .new_reader()
        .expect("serialize the report")
        .read_to_string(&mut payload)
        .expect("read the serialized payload");

    let json: serde_json::Value = serde_json::from_str(&payload).expect("a valid JSON payload");

    assert_eq!(json["service_name"], "circleci");
    assert_eq!(json["repo_token"], "integration-token");
    assert!(json.get("git").is_some(), "git metadata should have been fetched from the repository");
}
