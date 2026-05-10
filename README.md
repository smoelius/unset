# unset

A [Rust test runner] that unsets environment variables before running a test executable.

Why would one use this? Setting `CARGO_TERM_COLOR=always` on GitHub makes the output much easier to read. But it can also interfere with snapshot tests that expect exact output. Running such tests with `unset` eliminates the interference.

To use `unset`, do the following.

1. Create a file `unset.txt` in the root of your repository with the names of the environment variables that should be unset, listed one per line. Blank lines and lines beginning with `#` are ignored. Example:

   ```
   # Ensure snapshot tests are not affected by colored Cargo output.
   CARGO_TERM_COLOR
   ```

2. In your GitHub workflow file, install `unset` and configure your tests to run with it. Example:

   ```yaml
   cargo install unset
   ...
   cargo test --config "target.'cfg(all())'.runner = 'unset'"
   ```

For a more detailed example, see the [`fixture`] in this repository.

[`fixture`]: fixture
[Rust test runner]: https://doc.rust-lang.org/cargo/reference/config.html#targettriplerunner
