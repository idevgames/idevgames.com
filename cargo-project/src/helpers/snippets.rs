use serde::Serialize;

use crate::{
    db::DbConn,
    models::{ModelError, Snippet},
};

/// A context used to show snippets outside of edit forms.
#[derive(Debug, Serialize)]
pub struct SnippetContext {
    id: i32,
    taxonomy: String,
    title: String,
    icon: String,
    shared_by: String,
    shared_on: String,
    summary: String,
    href: String,
}

impl From<&Snippet> for SnippetContext {
    fn from(snippet: &Snippet) -> Self {
        SnippetContext {
            id: snippet.id,
            taxonomy: snippet.taxonomy.clone(),
            title: snippet.title.clone(),
            icon: snippet.icon.clone(),
            shared_by: snippet.shared_by.clone(),
            shared_on: snippet.shared_on.format("%Y-%m-%d").to_string(),
            summary: snippet.summary.clone(),
            href: snippet.href.clone(),
        }
    }
}

/// Drives the Snippets macro.
#[derive(Debug, Serialize)]
pub struct SnippetList {
    taxonomy: String,
    snippets: Vec<SnippetContext>,
    showing_hidden: bool,
    show_controls: bool,
    show_pages: bool,
    current_page: i32,
    total_pages: i64,
}

impl SnippetList {
    pub fn new(
        conn: &DbConn,
        page: i32,
        page_size: i32,
        taxonomy: &str,
        show_controls: bool,
        show_pages: bool,
        visible_only: bool,
    ) -> Result<SnippetList, ModelError> {
        let snippets = Snippet::find_all_by_taxonomy(
            conn,
            visible_only,
            taxonomy,
            page.into(),
            page_size.into(),
        )?;
        let snippet_count = Snippet::count(conn, visible_only, taxonomy)?;
        let total_pages = std::cmp::max((snippet_count as f32 / page_size as f32).ceil() as i64, 1);

        Ok(SnippetList {
            taxonomy: taxonomy.to_owned(),
            showing_hidden: !visible_only,
            snippets: snippets
                .iter()
                .map(|snippet| SnippetContext::from(snippet))
                .collect(),
            show_controls,
            show_pages,
            current_page: page,
            total_pages,
        })
    }
}
