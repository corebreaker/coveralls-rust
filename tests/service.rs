//! Integration tests for the public [`Service`] API.

use coveralls::Service;

#[test]
fn known_service_names_round_trip() {
    let names = [
        "circleci",
        "travis-ci",
        "appveyor",
        "jenkins",
        "semaphore-ci",
        "github-actions",
        "buildkite",
    ];

    for name in names {
        let service = Service::from_name(name).unwrap_or_else(|| panic!("`{name}` should be a known service"));

        assert_eq!(service.get_name(), name, "the service name should round-trip");
    }
}

#[test]
fn unknown_service_names_are_rejected() {
    assert!(Service::from_name("not-a-ci").is_none());
    assert!(Service::from_name("CircleCI").is_none(), "the lookup is case-sensitive");
    assert!(Service::from_name("").is_none());
}
