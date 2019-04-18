pathtools
======

pathtools is a small library that adds some helper methods which are useful when working with Paths.

This mostly usefuly when you're doing low-level stuff on UNIX systems with files. I factored this library out of ![hunter](https://github.com/rabite0/hunter), so for the most part it contains stuff I needed there, plus a bit more I stopped using.

Available methods and their singatures are:

```rust
fn short_path(&self) -> PathBuf;
fn short_string(&self) -> String;
fn name_starts_with(&self, pat: &str) -> bool;
fn quoted_file_name(&self) -> Option<OsString>;
fn quoted_path(&self) -> OsString;
```

NOTE: short_path() removes the $HOME component of a Path and replaces it with ~, so "/home/foo/bar" becomes "~/bar".
