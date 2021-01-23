use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::{NaiveDateTime, ParseError as DTParseError};
use serde::Serialize;

use crate::{
    application_context::ApplicationContext,
    helpers::admin_only::{AdminOnly, AdminOnlyContext},
    icons::Icon,
    models::Snippet,
};

#[derive(Debug, Serialize)]
struct SnippetContext {
    id: i32,
    hidden: bool,
    title: String,
    icon: Option<String>,
    shared_by: String,
    shared_on: String,
    summary: String,
    description: String,
    href: String,
}

impl From<&Snippet> for SnippetContext {
    fn from(snippet: &Snippet) -> Self {
        SnippetContext {
            id: snippet.id,
            hidden: snippet.hidden,
            title: snippet.title.clone(),
            icon: snippet.icon.clone(),
            shared_by: snippet.shared_by.clone(),
            shared_on: snippet.shared_on.format("%Y-%m-%d").to_string(),
            summary: snippet.summary.clone(),
            description: snippet.description.clone(),
            href: snippet.href.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
struct FormContext {
    auth: AdminOnlyContext,
    create_mode: bool,
    action_url: String,
    taxonomy: String,
    snippet: SnippetContext,
    icons: Vec<Icon>,
}

// TODO: implement this so that anyone can view it.
// GET /snippets/{taxonomy}?page={page_number}, shows a part of the snippets

// GET /snippets/{taxonomy}/new, renders the form
pub async fn new(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path(taxonomy): web::Path<String>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = AdminOnly::from_session(&conn, &session)?;

    let context = FormContext {
        auth: user.to_context(),
        create_mode: true,
        action_url: format!("/snippets/{}", taxonomy),
        taxonomy: taxonomy,
        snippet: SnippetContext::from(&Snippet::default()),
        icons: Icon::get_all(),
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippet_form.html.tera", &context)))
}

// TODO: the following should be implemented with adminonly
// POST /snippets/{taxonomy} create a new snippet
// GET /snippets/{taxonomy}/{snippet_id}/edit form for editing an existing snippet
// PATCH/PUT /snippets/{taxonomy}/{snippet_id} update a specific snippet
// DELETE /snippets/{taxonomy}/{snippet_id} delet this pls

// don't bother implementing this, these are snippets, not blog posts. while we
// have a big old discription field, i don't know that we want to use it yet, if
// at all.
// GET /snippets/{taxonomy}/{snippet_id} display a specific snippet

fn parse_date(date: &str) -> Result<NaiveDateTime, DTParseError> {
    NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", date),
        "%Y-%m-%d %H:%M:%S",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_date;

    #[test]
    fn date_parsing() {
        let foo = parse_date("2021-01-01");
        assert!(foo.is_ok());
    }
}
