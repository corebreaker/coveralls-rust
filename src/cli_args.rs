use clap::{ArgMatches, clap_app, crate_authors, crate_version, crate_description};

pub fn make_args() -> ArgMatches<'static> {
    clap_app!(Hypscript =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (after_help: "\
            The sub-command name is the service name (i.e.: circleci for Circle-CI.\n\
            For each sub-command, options override environment variable.\
        ")
        (@setting ColoredHelp)
        (@setting GlobalVersion)
        (after_help: "\
            Used environment variables in a generic context:\n\
            - Git branch:           CI_BRANCH\n\
            - Service number:       CI_BUILD_NUMBER\n\
            - Service pull request: CI_PULL_REQUEST\n\
            - Service build url:    CI_BUILD_URL\n\
            - Service job id:       CI_JOB_ID\n\
            - Service job number:   CI_JOB_NUMBER\n\
            \n\
            Used Coveralls environment variables:\n\
            - Coveralls token:      COVERALLS_REPO_TOKEN\n\
            - Git branch:           COVERALLS_BRANCH\n\
            - Git branch:           COVERALLS_BRANCH\n\
            - Flag name:            COVERALLS_FLAG_NAME\n\
            - Service number:       COVERALLS_SERVICE_NUMBER\n\
            - Service pull request: COVERALLS_PULL_REQUEST\n\
            - Service build url:    COVERALLS_BUILD_URL\n\
            - Service job id:       COVERALLS_SERVICE_JOB_ID\n\
            - Service job number:   COVERALLS_SERVICE_JOB_NUMBER\n\
        ")
        (@arg input: -i --input <file_name> !required "Use `file_name` as input file instead of standard input")
        (@arg source_prefix: -P --prefix <prefix> !required "Add a prefix to all files")
        (@arg prune_dir: -D --("prune-dir") <dir> ... !required "Prune directory")
        (@arg prune_absolutes: -X --("prune-absolutes") !required "Prune absolute paths")
        (@subcommand circleci =>
            (author: crate_authors!())
            (about: "Service Circle-CI")
            (after_help: "Used environment variables:\n\
                - Git branch:           CIRCLE_BRANCH\n\
                - Service number:       CIRCLE_WORKFLOW_ID, CIRCLE_BUILD_NUM\n\
                - Service pull request: CIRCLE_PULL_REQUEST\n\
                - Service build url:    CIRCLE_BUILD_URL\n\
                - Service job id:       CIRCLE_WORKFLOW_JOB_ID\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand actions =>
            (author: crate_authors!())
            (about: "Service Github-Actions")
            (after_help: "Used environment variables:\n\
                - Git branch:           GITHUB_REF, GITHUB_HEAD_REF\n\
                - Service number:       GITHUB_RUN_ID\n\
                - Service pull request: GITHUB_REF\n\
                - Service job id:       GITHUB_JOB\n\
                - Service job number:   GITHUB_RUN_NUMBER\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand appveyor =>
            (author: crate_authors!())
            (about: "Service AppVeyor")
            (after_help: "Used environment variables:\n\
                - Git branch:           APPVEYOR_REPO_BRANCH\n\
                - Service number:       APPVEYOR_BUILD_NUMBER\n\
                - Service pull request: APPVEYOR_PULL_REQUEST_NUMBER\n\
                - Service job id:       APPVEYOR_BUILD_ID\n\
                - Service job number:   APPVEYOR_JOB_NUMBER\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand buildkite =>
            (author: crate_authors!())
            (about: "Service BuildKite")
            (after_help: "Used environment variables:\n\
                - Git branch:           BUILDKITE_BRANCH\n\
                - Service number:       BUILDKITE_BUILD_NUMBER\n\
                - Service pull request: BUILDKITE_PULL_REQUEST\n\
                - Service build url:    BUILDKITE_BUILD_URL\n\
                - Service job id:       BUILDKITE_JOB_ID\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand travis =>
            (author: crate_authors!())
            (about: "Service Travis")
            (after_help: "Used environment variables:\n\
                - Git branch:           TRAVIS_BRANCH\n\
                - Service number:       TRAVIS_BUILD_NUMBER\n\
                - Service pull request: TRAVIS_PULL_REQUEST\n\
                - Service build url:    TRAVIS_BUILD_WEB_URL\n\
                - Service job id:       TRAVIS_JOB_ID\n\
                - Service job number:   TRAVIS_JOB_NUMBER\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand semaphore =>
            (author: crate_authors!())
            (about: "Service Semaphore-CI")
            (after_help: "Used environment variables:\n\
                - Git branch:           SEMAPHORE_GIT_BRANCH\n\
                - Service number:       SEMAPHORE_EXECUTABLE_UUID, SEMAPHORE_WORKFLOW_ID\n\
                - Service pull request: SEMAPHORE_BRANCH_ID, SEMAPHORE_GIT_PR_NUMBER\n\
                - Service job id:       SEMAPHORE_JOB_UUID, SEMAPHORE_JOB_ID\n\
                - Service job number:   SEMAPHORE_WORKFLOW_NUMBER\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
        (@subcommand jenkins =>
            (author: crate_authors!())
            (about: "Service Jenkins")
            (after_help: "Used environment variables:\n\
                - Service number:       BUILD_NUMBER\n\
                - Service pull request: CI_PULL_REQUEST\n\
                - Service build url:    BUILD_URL\n\
                - Service job id:       BUILD_ID\
            ")
            (@arg repo_token: -t --("repo-token") <value> !required "Repo token")
            (@arg flag_name: -f --("flag-name") <value> !required "Flag name")
            (@arg service_number: -s --("service-number") <value> !required "Service number")
            (@arg service_build_url: -u --("service-build-url") <value> !required "Service build url")
            (@arg service_pull_request: -p --("service-pull-request") <value> !required "Service pull request")
            (@arg service_job_id: -j --("service-job-id") <value> !required "Service job id")
            (@arg service_job_number: -n --("service-job-number") <value> !required "Service job number")
            (@arg git_id: -k --("git-id") <value> !required "Git id")
            (@arg git_branch: -b --("git-branch") <value> !required "Git branch")
            (@arg git_message: -m --("git-message") <value> !required "Git message")
            (@arg git_author_name: -a --("git-author-name") <value> !required "Git author name")
            (@arg git_author_email: -A --("git-author-email") <value> !required "Git author email")
            (@arg git_committer_name: -c --("git-committer-name") <value> !required "Git committer name")
            (@arg git_committer_email: -C --("git-committer-email") <value> !required "Git committer email")
            (@arg git_remote_name: -r --("git-remote-name") <value> !required "Git remote name")
            (@arg git_remote_url: -R --("git-remote-url") <value> !required "Git remote url")
        )
    ).get_matches()
}