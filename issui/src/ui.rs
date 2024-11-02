use std::process::Command;

use crate::desc::*;

use cursive::{self, align::Align, direction::Orientation, event::{Event, Key}, theme::{Color, ColorStyle, Effect, Style}, utils::span::SpannedString, view::{Nameable, Resizable, Scrollable}, views::{Dialog, EditView, LinearLayout, ListView, Panel, SelectView, TextView, ViewRef}, Cursive, CursiveExt};

pub struct UI {
    pub(crate) base: Cursive,
}

impl UI {
    pub fn new() -> Self {
        let mut base = cursive::Cursive::new();
        base.load_toml(include_str!("../Cursive.toml")).unwrap();

        base.add_global_callback('q', |app| app.quit());
        base.add_global_callback(Event::Key(Key::Esc), |app| app.quit());
        base.add_global_callback('h', |app| Self::help_ui(app));

        Self { base }
    }

    pub fn run(&mut self) {
        Self::select_repo_ui(&mut self.base);
        self.base.run();
    }

    fn select_repo_ui(app: &mut Cursive) {
        match Command::new("gh").args(["repo", "list", "--json", "name,owner,visibility"]).output() {
            Ok(value) => {
                let error = String::from_utf8(value.stderr).unwrap();
                let result = String::from_utf8(value.stdout).unwrap();
                if error.contains("auth login") {
                    let text = TextView::new("You have not yet logged in to the GitHub CLI. Log in to use this interface.\nTo login, run `gh auth login` and follow as directed");
                    app.add_layer(
                        Dialog::around(text)
                        .button("Ok, I'll come back once I've logged in", |app| app.quit())
                        .title("Login, please")
                    );
                } else if !error.is_empty() {
                    let text = TextView::new(format!("An error occured while trying to run the command to fetch your repos! Error {error}"));
                    app.add_layer(
                        Dialog::around(text)
                        .button("Quit", |app| app.quit())
                        .title("I failed")
                    );
                } else {
                    let parsed_json: Result<Vec<RepoData>, serde_json::Error> = serde_json::from_str(&result);
                    let mut list = LinearLayout::new(Orientation::Vertical);
                    let mut input =  EditView::new();

                    let remove_styles = |app: &mut Cursive, json: &RepoData| {
                        let key = format!("{}/{}", json.owner.login, json.name);
                        let item: Option<ViewRef<SelectView>> = app.find_name(key.as_str());
                        match item {
                            Some(mut item) => {
                                match item.get_item_mut(0) {
                                    Some((a, _)) => 
                                        *a = SpannedString::single_span(format!("{}", a.source()),  Style::default()),

                                    None => ()
                                };
                            }

                            None => (),
                        }
                    };

                    match parsed_json {
                        Ok(mut parsed_json) => {
                            parsed_json.sort_by(|a, b| a.owner.login.cmp(&b.owner.login));
                            let input_parsed_json = parsed_json.clone();
                            input.set_on_submit(move |app, data| {
                                let mut matches = (0, "".to_string());
                                for json in &input_parsed_json {
                                    remove_styles(app, json);

                                    let search_str = 
                                        format!(
                                            "{} {}", 
                                            if json.visibility == "PUBLIC" { format!("[{}] ", json.visibility) } 
                                            else { format!("[{}]", json.visibility) }, json.name
                                        );

                                    if data.len() >= 3 && Self::is_simple_match(&data.to_lowercase(), &search_str.to_lowercase()) {
                                        let key = format!("{}/{}", json.owner.login, json.name);
                                        let item: Option<ViewRef<SelectView>> = app.find_name(key.as_str());
                                        matches.0 += 1;
                                        matches.1 = key;
                                        match item {
                                            Some(mut item) => {
                                                match item.get_item_mut(0) {
                                                    Some((a, _)) => 
                                                        *a = SpannedString::single_span(a.source(), Style::highlight().combine(ColorStyle::front(Color::Rgb(0, 0, 0)))),
                                                    None => ()
                                                };
                                            }

                                            None => (),
                                        }
                                    }
                                }
                                if matches.0 == 1 {
                                    Self::open_repo_ui(app, &matches.1, false);
                                }
                            });
                            list.add_child(ListView::new().child("Search", input));
                            let mut prev = String::new();
                            for json in parsed_json {
                                if prev != json.owner.login {
                                    let styles = Style::from(ColorStyle::highlight()).combine(Style::from(ColorStyle::secondary()));
                            
                                    let text_view = 
                                        TextView::new(format!("{}", json.owner.login))
                                            .style(styles)
                                            .align(Align::center());
                                
                                    list.add_child(Panel::new(text_view));
                                    prev = json.owner.login.clone();
                                }
                                let clickable = 
                                    SelectView::new()
                                        .item(
                                            format!(
                                                "{} {}", 
                                                if json.visibility == "PUBLIC" { format!("[{}] ", json.visibility) } 
                                                else { format!("[{}]", json.visibility) }, json.name), 
                                                format!("{}/{}", json.owner.login, json.name)
                                            )
                                        .on_submit(|app, item: &String| { Self::open_repo_ui(app, item, false) })
                                        .with_name(format!("{}/{}", json.owner.login, json.name));
                                list.add_child(clickable);
                            }


                            app.add_layer(
                                Dialog::around(list.scrollable())
                                    .title("Select your repository")
                                    .button("Help", |app| Self::help_ui(app))
                                    .button("Quit", |app| app.quit())
                                );
                        }

                        Err(error) => {
                            app.add_layer(
                                Dialog::around(TextView::new(format!("{result}\nFailed to fetch your repositories! Error {error}")))
                                .button("Quit", |app| app.quit())
                                .title("I failed")
                            );
                        }
                    };
                }
            }
            Err(error) => {
                app.add_layer(TextView::new(format!("Failed to connect to GitHub repos! Please try checking your connection / configuration. Error {error}")));
            }
        };
    }

    pub fn open_repo_ui(app: &mut Cursive, repo_to_open: &String, from_command_line: bool) {
        // Remove the previous layer
        app.pop_layer();
        match Command::new("gh").args(["issue", "list", "--state", "all", "--repo", repo_to_open.as_str(), "--json", "title,state,labels,body,author,comments"]).output() { 
            Ok(value) => {
                let error = String::from_utf8(value.stderr).unwrap();
                let result = String::from_utf8(value.stdout).unwrap();

                if !error.is_empty() {
                    if from_command_line {
                        app.add_layer(
                            Dialog::around(TextView::new(
                                format!(
                                    "I could not find the repository `{repo_to_open}`, or I could not connect to it! Try checking for mispellings, make sure the format <owner>/<repo_name> is correct and check your connection.\nError {error}"
                                )    
                            ))
                            .button("Show Repositories", |app| { app.pop_layer(); Self::select_repo_ui(app); })
                            .button("Quit", |app| app.quit())
                            .title("I failed")
                        );
                    } else {
                        let text = TextView::new(format!("An error occured while trying to run the command to fetch your repos! Error {error}"));
                        app.add_layer(
                            Dialog::around(text)
                            .button("Back", |app| { app.pop_layer(); Self::select_repo_ui(app); })
                            .button("Quit", |app| app.quit())
                            .title("I failed")
                        );
                    }
                } else {

                    let parsed_json: Result<Vec<IssueData>, serde_json::Error> = serde_json::from_str(&result);

                    match parsed_json {
                        Ok(result) => {
                            let mut view = LinearLayout::horizontal();
                            let mut left_side = LinearLayout::vertical();
                            let mut right_side = LinearLayout::vertical();

                            left_side.add_child(Panel::new(
                                TextView::new(format!("Issues for {repo_to_open}"))
                                    .style(Style::from(ColorStyle::highlight()).combine(Style::from(ColorStyle::secondary())))
                                    .align(Align::center())
                            ));

                            if !result.is_empty() {
                                left_side.add_child(TextView::new("Search").align(Align::center()));

                                let mut input = EditView::new();

                                let remove_styles = |app: &mut Cursive, issue_data: &IssueData| {
                                    let key = &issue_data.title;
                                    let item: Option<ViewRef<SelectView<IssueData>>> = app.find_name(key.as_str());
                                    match item {
                                        Some(mut item) => {
                                            match item.get_item_mut(0) {
                                                Some((a, _)) => 
                                                    *a = SpannedString::single_span(format!("{}", a.source()),  Style::default()),

                                                None => ()
                                            };
                                        }

                                        None => (),
                                    }
                                };

                                let input_result = result.clone();
                                input.set_on_submit(move |app, data| {
                                    let mut matches = (0, None);
                                    for issue_data in &input_result {
                                        remove_styles(app, &issue_data);
                                        if data.len() >= 3 && Self::is_match(data.to_string(), issue_data) {
                                            let key = &issue_data.title;
                                            let item: Option<ViewRef<SelectView<IssueData>>> = app.find_name(key.as_str());
                                            matches.0 += 1;
                                            match item {
                                                Some(mut item) => {
                                                    match item.get_item_mut(0) {
                                                        Some((a, b)) => {
                                                            *a = SpannedString::single_span(a.source(), Style::highlight().combine(ColorStyle::front(Color::Rgb(0, 0, 0))));
                                                            matches.1 = Some(b.clone());
                                                        },
                                                        None => ()
                                                    };
                                                }

                                                None => (),
                                            }
                                        }
                                    }

                                    if matches.0 == 1 {
                                        let mut right_side: ViewRef<LinearLayout> = app.find_name("right_side").unwrap();
                                        right_side.clear();
                                        Self::open_issue(&matches.1.unwrap(), &mut right_side);
                                    }
                                });

                                left_side.add_child(input);
                            }

                            for i in 0..result.len() {
                                let issue_data = &result[i];
                                let clickable = 
                                    SelectView::new()
                                        .item(format!("{}", issue_data.title), issue_data.clone())
                                        .on_submit(|app, item: &IssueData| {
                                            app.call_on_name("right_side", |right_side: &mut LinearLayout| {
                                                right_side.clear();
                                                Self::open_issue(item, right_side);
                                            });
                                        })
                                        .with_name(issue_data.title.as_str());

                                left_side.add_child(clickable);
                            }

                            if !result.is_empty() {
                                Self::open_issue(&result[0], &mut right_side);
                            } else {
                                right_side.add_child(TextView::new("No issue selected").align(Align::center()));
                                left_side.add_child(TextView::new("No issues available").align(Align::center()));
                            }

                            view.add_child(left_side.scrollable());
                            view.add_child(TextView::new(" "));
                            view.add_child(right_side.with_name("right_side").full_screen().scrollable());

                            let dialog =                                 
                                Dialog::around(view.full_screen())
                                    .button("Back", |app| { app.pop_layer(); Self::select_repo_ui(app); })
                                    .button("Help", |app| Self::help_ui(app))
                                    .button("Quit", |app| app.quit());

                            app.add_layer(dialog);
                        }

                        Err(error) => {
                            app.add_layer(
                                Dialog::around(TextView::new(format!("An error occured when trying to read issue data of the repository `{repo_to_open}`! Error {error}")))
                                .button("Back", |app| { app.pop_layer(); Self::select_repo_ui(app); })
                                .button("Quit", |app| app.quit())
                                .title("I failed")
                            );
                        }
                    }
                }
            }

            Err(error) => {
                app.add_layer(
                    Dialog::around(TextView::new(format!("Failed to fetch data on repository `{repo_to_open}`! Error {error}")))
                    .button("Back", |app| { app.pop_layer(); Self::select_repo_ui(app); })
                    .button("Quit", |app| app.quit())
                    .title("I failed")
                );
            }
        }
    }

    fn open_issue(issue_data: &IssueData, right_side: &mut LinearLayout) {
        let mut label_data = String::new(); 
        for label in &issue_data.labels {
            if label_data.is_empty() {
                label_data = format!("<{}>", label.name);
            } else {
                label_data = format!("{label_data}, <{}>", label.name);
            }
        }

        if label_data.trim().is_empty() {
            label_data = "<<None>>".to_string();
        }

        right_side.add_child(
            TextView::new(&issue_data.title)
                .align(Align::center())
                .style(
                    Style::from(ColorStyle::secondary())
                        .combine(ColorStyle::front(Color::Rgb(0, 0, 0)))
                )
        );

        let state_view = 
            LinearLayout::horizontal()
                .child(TextView::new("State: ").style(ColorStyle::secondary()))
                .child(TextView::new(&issue_data.state.to_lowercase()).style(ColorStyle::tertiary()));

        let label_view = 
            LinearLayout::horizontal()
                .child(TextView::new("Labels: ").style(ColorStyle::secondary()))
                .child(TextView::new(label_data).align(Align::top_right()).style(ColorStyle::tertiary()))
                .full_width();

        right_side.add_child(label_view);
        right_side.add_child(state_view);
        right_side.add_child(TextView::new("\nBody:").style(Style::from(Effect::Underline).combine(ColorStyle::secondary())));
        right_side.add_child(TextView::new(if issue_data.body.trim().is_empty() { "<No body>" } else { &issue_data.body }));
        right_side.add_child(TextView::new("\nComments:").style(Style::from(Effect::Underline).combine(ColorStyle::secondary())));
        
        for comment in &issue_data.comments {
            right_side.add_child(
                TextView::new(format!("Comment by: {}\n", comment.author.login))
                    .style(Effect::Underline)
            );

            right_side.add_child(TextView::new(if comment.body.trim().is_empty() { "<No text in comment>" } else { &comment.body }));
        }

        if issue_data.comments.is_empty() {
            right_side.add_child(TextView::new("<No comments>"));
        }
    }

    fn help_ui(app: &mut Cursive) {
        let mut list = LinearLayout::vertical();

        match app.find_name::<LinearLayout>("help") {
            Some(..) => {
                app.pop_layer();
                return;
            },
            None => ()
        };

        list.add_child(
            TextView::new("Running this CLI without any arguments")
                .style(
                    Style::from(ColorStyle::secondary())
                        .combine(Effect::Underline)
                )
        );
        list.add_child(TextView::new(
            "You're greeted with a search bar, and a few buttons representing your repositories. Search for your repositories, or click on the button to redirected to the issues of that repository."
        ));
        list.add_child(TextView::new(
            "Searching for a repository involves having to type in at least 3 characters and pressing enter. If there are 2 or more matches, they are highlighted. Otherwise, you are redirected to the issues of that repository."
        ));

        list.add_child(
            TextView::new("\nRunning this CLI with a repository as an argument `<orgname/owner>/repository` or after selecting your repository from the launch UI")
                .style(
                    Style::from(ColorStyle::secondary())
                        .combine(Effect::Underline)
                )
        );

        list.add_child(TextView::new(
            "To your left, you have the details about an automatically selected first issue, if you have at least one issue"
        ));

        list.add_child(TextView::new(
            "To your right, a search bar, use commands is: <open/closed>, user: <any username that's involved in the issue>, mentions: <any username mentioned in the body / comments>, or just plain search for your issue, I suppose a *form* of fuzzy search is supported. Select any issue you wish to view."
        ));

        list.add_child(
            TextView::new("\nGeneral navigation")
                .style(
                    Style::from(ColorStyle::secondary())
                        .combine(Effect::Underline)
                )
        );

        list.add_child(TextView::new(
            "Press <ESC> at any time to quit the app. Press `q` to do the same function when not focused to an input. Press `h` to toggle this help dialog when not focused to an input."
        ));

        app.add_layer(
            Dialog::around(list.with_name("help").scrollable())
            .title("Help")
            .button("Back", |app| { app.pop_layer(); })
            .button("Quit", |app| app.quit())
            .full_screen()
        )
    }

    fn is_match(input: String, data: &IssueData) -> bool {
        if input.trim().len() < 3 { return false; }
        let lowered = input.to_lowercase();
        let space_split = lowered.split(" ").collect::<Vec<_>>();

        for mut i in 0..space_split.len() {
            let item = space_split[i];
            if item.trim().is_empty() { continue; }

            // check for `is: open`, `is: closed`
            if item.starts_with("is:") {
                let mut check = item.strip_prefix("is:").unwrap().trim();
                
                while check.is_empty() && space_split.len() > i {
                    i += 1;
                    check = space_split[i];
                }

                match check {
                    check if data.state.to_lowercase().contains(&check) => return true,
                    _ => ()
                }
            }

            // Search for @<whatever comes next> in the comments and body of the issue.
            // This is obviously not foolproof
            else if item.starts_with("mentions:") {
                let mut check = item.strip_prefix("mentions:").unwrap().trim();
                
                while check.is_empty() && space_split.len() > i + 1 {
                    i += 1;
                    check = space_split[i];
                }

                // Check for mention in comment data.
                for comment in &data.comments {
                    if comment.body.to_lowercase().contains(&format!("@{check}")) {
                        return true;
                    }
                }

                // Check for mention in the body of the issue
                match check {
                    check if data.body.to_lowercase().contains(&format!("@{check}")) => return true,

                    _ => ()
                }
            }

            else if item.starts_with("user:") {
                let mut check = item.strip_prefix("user:").unwrap().trim();
                
                while check.is_empty() && space_split.len() > i + 1 {
                    i += 1;
                    check = space_split[i];
                }
                
                // Check for mention in comment data.
                for comment in &data.comments {
                    if comment.author.login.to_lowercase().contains(&check) {
                        return true;
                    }
                }

                // Check for mention in the body of the issue
                match check {
                    check if data.author.login.to_lowercase().contains(&check) => return true,

                    _ => ()
                }

            }
        }
        Self::is_data_match(&input, data)
    }

    fn is_data_match(input: &str, data: &IssueData) -> bool {            
        let item = input.trim().to_lowercase();
        if item.len() < 3 { return false; }

        let mut to_search = vec![
            data.body.to_lowercase(),
        ];

        let mut to_fuzzy_search = vec![
            data.title.to_lowercase(),
        ];

        let comment_data = 
            data
                .comments
                .iter()
                .map(|comment_data| 
                    format!("{} {}", &comment_data.author.login.to_lowercase(), &comment_data.body.to_lowercase())
                )
                .collect::<Vec<_>>();

        let label_data = 
                data
                    .labels
                    .iter()
                    .map(|label_data| label_data.name.to_lowercase().clone())
                    .collect::<Vec<_>>();

        to_search.extend_from_slice(comment_data.as_slice());
        to_fuzzy_search.extend_from_slice(label_data.as_slice());

        // input is foo bar, item is bar -> true; input is bar, item is foobar -> true
        for searched in to_search {
            if (item.contains(&searched) || searched.contains(&item)) && !searched.trim().is_empty() {
                return true;
            }
        }

        // Fuzzy search labels and title. Idk if labels are required, but whatever.
        for searched in to_fuzzy_search {
            if !searched.trim().is_empty() && Self::is_within_acceptable_error(searched, item.to_string(), 0) {
                return true;
            }
        }

        false
    }

    fn is_simple_match(input: &str, search_str: &str) -> bool {
        if 
            !input.trim().is_empty() && 
            !search_str.trim().is_empty() && 
            (Self::is_within_acceptable_error(input.to_string(), search_str.to_string(), 0) || search_str.contains(&input))
        {
            return true;
        }
        false
    }

    // A slightly working fuzzy search algorithm, if it even is one?
    // I tried using available crates, but I couldn't get them to work the way I wanted to
    // Perhaps I used them wrong.
    fn is_within_acceptable_error(mut a: String, mut b: String, iterations: usize) -> bool {

        if a.len() < 2 || b.len() < 2 { return false; }

        // The threshold of the diminished distance at which, this function returns false
        // This number is pretty arbitrary
        // It *seems* to be a good balance of accepting mistakes and not returning true for obvious non-matches
        let threshold = 0.37;
        // Number of times to shift to next delimiter
        let iteration_threshold = 5;

        // Look for a space to test starting there
        if a.get(0..1) != b.get(0..1) {
            // a is the longest string
            if b.len() > a.len() {
                std::mem::swap(&mut a, &mut b);
            }

            let matches_delim = 
                |c| 
                    matches!(c, ' ' | '-' | '[' | ']' | '(' | ')' | '|' | '{' | '}' | '+' | '_' | '`' | '~' | ':' | ';' | ',' | '.' | '\\' | '?');
            if let Some(location) = a.find(matches_delim) {
                a = a[location..].to_string();
            } 
            // No matching start location was found
            else {
                return false;
            }
        }
        let min_len = a.len().min(b.len());
        let mut distance = 0;

        // Compute distance between aligned strings
        for i in 1..min_len {
            // compare each character
            if a[(i - 1)..i] != b[(i - 1)..i] {
                distance += 1;
            }
        }

        if (distance as f32 / min_len as f32) < threshold {
            true
        } else {
            // Shift the strings by 1 and attempt to search for a match
            if a.len() > b.len() {
                a = a.get(1..).unwrap_or("").to_string();
            } else {
                b = b.get(1..).unwrap_or("").to_string();
            }

            if iterations > iteration_threshold { return false; }

            Self::is_within_acceptable_error(a, b, iterations + 1)
        }
    }
}