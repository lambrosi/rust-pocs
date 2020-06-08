#### Unit tests
* Live in the _src_ directory in each file with the code that they’re testing;
* Create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`;
* `#[cfg(test)]` annotation make tests run only with `cargo test` and not with `cargo build` (to reduce compile time);
* Private functions **can** be tested;

#### Integration tests
* Are entirely external to your library;
* Can only call functions that are part of your library’s public API;
* Create a _tests_ directory at the top level of our project directory, next to _src_ (multiple test files are allowed);
* Only library creates could have integration test (lib.rs). Binary crates (main.rs) not.
