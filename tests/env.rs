//! Integration test for the public [`Env`] accessor.
//!
//! A single test lives in this binary because it mutates the process environment, which would be
//! racy if it ran alongside other tests in the same binary.

use coveralls::Env;

#[test]
fn empty_variables_are_treated_as_unset() {
    // SAFETY: this is the only test in this binary, so no other thread reads or writes the
    // environment concurrently.
    unsafe {
        std::env::set_var("COVERALLS_IT_SET", "a-value");
        std::env::set_var("COVERALLS_IT_EMPTY", "");
        std::env::remove_var("COVERALLS_IT_MISSING");
    }

    let env = Env::new();

    assert_eq!(
        env.get_var("COVERALLS_IT_SET").expect("reading a set variable should succeed"),
        Some(String::from("a-value")),
    );

    assert_eq!(
        env.get_var("COVERALLS_IT_EMPTY").expect("reading an empty variable should succeed"),
        None,
        "an empty variable must be reported as unset",
    );

    assert_eq!(
        env.get_var("COVERALLS_IT_MISSING").expect("reading a missing variable should succeed"),
        None,
    );
}
