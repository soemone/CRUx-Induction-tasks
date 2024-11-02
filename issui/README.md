## Task 2: Create a GitHub issue tracker

This is a very basic TUI wrapper that allows you to view your GitHub issues using the GitHub CLI.

Run this binary without any arguments, or with an argument of the form `<orgname/owner>/<repository>`

```bash
# Directly open up repo selector UI
cargo run --release
# Or:
cargo run --release -- "<orgname/owner>/<repository>"
```

Select your repository in the UI
Then view the associated issues

### This app is built using:
- serde, serde-json: To parse the JSON that the GitHub CLI returns
- cursive: For creating the TUI