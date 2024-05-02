# coveralls

Send job to Coveralls (coveralls.io) from a rust program.

This command will help to send to _coveralls.io_ a coverage file in the format of Coveralls.

That can be used in local computer,
but this is made to be used with CI/CD environments like Travis, Circle-CI, Jenkins, or others.

## Install

For installing, you can use Cargo by invoking:

```shell
cargo install coveralls
```

## That's weird [coveralls-python][1] exists, so why another API client ?

The main reason is that `coveralls-python` can only send in `lcov` format.
But anymore, i saw that the format produced by `grcov` don't remove all dependencies.

Indeed, i used `Lalrpop` in one of my project,
and the generated file was included in the report produced by `grcov`.*
Here, we focus on Rust project and we remove all dependencies, on demand with a commend line argument.
We offer the possibility of including dependencies or to filter them, filter all or with an expression (a regex).

For this moment, we use only Coveralls format as input, but later we could use another formats.

## Configuration

As said, we accept only the Coveralls format, but other format is int the Todo list.

For parameters, we use environment variables for several CI environments:
- AppVeyor
- BuildKite
- Circle-CI
- Github Actions
- Jenkins
- Semaphore
- Travis

Command line parameters override configuration fetched from environment.

Command line argument `-h` gives a help on configuration.

## Todo list

- [ ] Input formats
  + [X] Coveralls
  + [ ] Lcov
- [ ] Add other entry points of the Coveralls API
- [ ] Add comments in code (with docs)


[1]: https://github.com/TheKevJames/coveralls-python