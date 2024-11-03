## Task 2: Create a GitHub issue tracker

This is a very basic TUI wrapper that allows you to view your GitHub issues using the GitHub CLI.

Run this binary without any arguments, or with an argument of the form `<orgname/owner>/<repository>`

```bash
# Tested on Rust version 1.82.0, likely runs on other versions
# But for best support, use the most recent version of Rust

# Directly open up repo selector UI
cargo run --release
# Or:
cargo run --release -- "<orgname/owner>/<repository>"
```

Select your repository in the UI
Then view the associated issues

### Limitations
- Currently shows only the first 30 issues
- Issues that are longer than 50 characters are cutoff and cannot be horizontally scrolled to view

### This app is built using:
- serde, serde-json: To parse the JSON that the GitHub CLI returns
- cursive: For creating the TUI
- open: To open links in the browser