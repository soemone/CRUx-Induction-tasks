// Used to prevent the compiler from producing warnings on code that I cannot change due to the format of the 
// JSON produced by the GitHub CLI
#![allow(unused, non_snake_case)]

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct OwnerData {
    pub id: String,
    pub login: String,
}

#[derive(Deserialize, Clone)]
pub struct RepoData {
    pub name: String,
    pub owner: OwnerData,
    pub visibility: String,
}

#[derive(Deserialize, Clone)]
pub struct AuthorData {
    pub id: String,
    pub is_bot: bool,
    pub login: String,
    pub name: String,
}
#[derive(Deserialize, Clone)]
pub struct CommentAuthorData {
    pub login: String,
}

#[derive(Deserialize, Clone)]
pub struct CommentReactionGroupUsers {
    pub totalCount: usize,
}

#[derive(Deserialize, Clone)]
pub struct CommentReactionGroup {
    pub content: String,
    pub users: CommentReactionGroupUsers,
}

#[derive(Deserialize, Clone)]
pub struct CommentData {
    pub id: String,
    pub author: CommentAuthorData,
    pub authorAssociation: String,
    pub body: String,
    pub createdAt: String,
    pub includesCreatedEdit: bool,
    pub isMinimized: bool,
    pub minimizedReason: String,
    pub reactionGroups: Vec<CommentReactionGroup>,
    pub url: String,
    pub viewerDidAuthor: bool,
}
#[derive(Deserialize, Clone)]
pub struct IssueLabelData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub color: String,
}

#[derive(Deserialize, Clone)]
pub struct IssueData {
    pub title: String,
    pub state: String,
    pub comments: Vec<CommentData>,
    pub labels: Vec<IssueLabelData>,
    pub body: String,
    pub author: AuthorData,
}
