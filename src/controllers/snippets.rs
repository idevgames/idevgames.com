use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse};
use chrono::{NaiveDateTime, ParseError as DTParseError};
use serde::{Deserialize, Serialize};

use crate::{
    application_context::ApplicationContext,
    helpers::{
        admin_only::{AdminOnly, AdminOnlyContext},
        snippets::SnippetList,
        user_optional::{UserOptional, UserOptionalContext},
    },
    icons::Icon,
    models::Snippet,
};

use super::HandlerError;

#[derive(Debug, Deserialize, Serialize)]
pub struct SnippetContext {
    id: i32,
    taxonomy: String,
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
            taxonomy: snippet.taxonomy.clone(),
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

// maps with https://getbootstrap.com/docs/5.0/components/alerts/
#[derive(Debug, Serialize)]
enum FlashSeverity {
    #[allow(dead_code)]
    Primary,
    #[allow(dead_code)]
    Secondary,
    Success,
    #[allow(dead_code)]
    Danger,
    #[allow(dead_code)]
    Warning,
    #[allow(dead_code)]
    Info,
    #[allow(dead_code)]
    Light,
    #[allow(dead_code)]
    Dark,
}

#[derive(Debug, Serialize)]
struct Flash {
    severity: FlashSeverity,
    message: String,
}

#[derive(Debug, Serialize)]
struct FormContext {
    auth: AdminOnlyContext,
    create_mode: bool,
    action_url: String,
    taxonomy: String,
    snippet: SnippetContext,
    icons: Vec<Icon>,
    flash: Option<Flash>,
}

#[derive(Debug, Deserialize)]
pub struct IndexQueryParams {
    page: Option<i32>,
    show_hidden: Option<bool>,
}

// GET /snippets/{taxonomy}?page={page_number}&show_hidden={show_hidden}
pub async fn index(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path(taxonomy): web::Path<String>,
    query: web::Query<IndexQueryParams>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = UserOptional::from_session(&conn, &session)?;
    let show_hidden = user.is_admin() && query.show_hidden.unwrap_or(false);
    let page = query.page.unwrap_or(0);
    let page_size = 5;

    let snippetlist = SnippetList::new(
        &conn,
        page,
        page_size,
        &taxonomy,
        user.is_admin(),
        true,
        !show_hidden,
    )?;

    #[derive(Debug, Serialize)]
    struct Context {
        auth: UserOptionalContext,
        snippetlist: SnippetList,
    }

    let context = Context {
        auth: user.to_context(),
        snippetlist: snippetlist,
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippetlist.html.tera", &context)))
}

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
        flash: None,
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippet_form.html.tera", &context)))
}

// POST /snippets/{taxonomy} create a new snippet
pub async fn create(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path(taxonomy): web::Path<String>,
    form: web::Form<SnippetContext>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = AdminOnly::from_session(&conn, &session)?;

    // TODO: when the date doesn't parse show the create form again with an
    //       error. the user base at the moment isn't really big enough to
    //       bother with such niceties, really.

    let snippet = Snippet::create(
        &conn,
        user.get_id(),
        &taxonomy,
        form.hidden,
        form.icon.as_deref(),
        &form.title,
        &form.shared_by,
        &parse_date(&form.shared_on)?,
        &form.summary,
        &form.description,
        &form.href,
    )?;

    Ok(HttpResponse::MovedPermanently()
        .header(
            "Location",
            format!("/snippets/{}/{}/edit", taxonomy, snippet.id),
        )
        .finish())
}

// GET /snippets/{taxonomy}/{snippet_id}/edit form for editing an existing snippet
pub async fn edit(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path((taxonomy, snippet_id)): web::Path<(String, i32)>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = AdminOnly::from_session(&conn, &session)?;
    let snippet = Snippet::find_by_id(&conn, snippet_id)?;

    if snippet.taxonomy != taxonomy {
        return Err(HandlerError::NotFound);
    }

    let context = FormContext {
        auth: user.to_context(),
        create_mode: false,
        action_url: format!("/snippets/{}/{}", taxonomy, snippet.id),
        taxonomy: taxonomy,
        snippet: SnippetContext::from(&snippet),
        icons: Icon::get_all(),
        flash: None,
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippet_form.html.tera", &context)))
}

// PATCH/PUT /snippets/{taxonomy}/{snippet_id} update a specific snippet
pub async fn update(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path((taxonomy, snippet_id)): web::Path<(String, i32)>,
    form: web::Form<SnippetContext>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = AdminOnly::from_session(&conn, &session)?;
    let mut snippet = Snippet::find_by_id(&conn, snippet_id)?;

    if snippet.taxonomy != taxonomy {
        return Err(HandlerError::NotFound);
    }

    snippet.hidden = form.hidden;
    snippet.title = form.title.clone();
    snippet.icon = form.icon.clone();
    snippet.shared_by = form.shared_by.clone();
    snippet.shared_on = parse_date(&form.shared_on)?;
    snippet.summary = form.summary.clone();
    snippet.description = form.description.clone();
    snippet.href = form.href.clone();

    snippet.update(&conn)?;

    let context = FormContext {
        auth: user.to_context(),
        create_mode: false,
        action_url: format!("/snippets/{}/{}", taxonomy, snippet.id),
        taxonomy: taxonomy,
        snippet: SnippetContext::from(&snippet),
        icons: Icon::get_all(),
        flash: Some(Flash {
            severity: FlashSeverity::Success,
            message: "Updated".to_owned(),
        }),
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippet_form.html.tera", &context)))
}

// DELETE /snippets/{taxonomy}/{snippet_id} delet this pls
pub async fn delete(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
    web::Path((taxonomy, snippet_id)): web::Path<(String, i32)>,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = AdminOnly::from_session(&conn, &session)?;
    let snippet = Snippet::find_by_id(&conn, snippet_id)?;

    if snippet.taxonomy != taxonomy {
        return Err(HandlerError::NotFound);
    }

    snippet.delete(&conn)?;

    #[derive(Debug, Serialize)]
    struct Context {
        auth: AdminOnlyContext,
    }

    let context = Context {
        auth: user.to_context(),
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("snippet_deleted.html.tera", &context)))
}

// GET /snippets/{taxonomy}/{snippet_id} display a specific snippet
pub async fn show(
  ctxt: web::Data<ApplicationContext>,
  session: Session,
  web::Path((taxonomy, snippet_id)): web::Path<(String, i32)>,
) -> Result<HttpResponse, super::HandlerError> {
  let conn = ctxt.db_pool.get()?;
  let user = UserOptional::from_session(&conn, &session)?;
  let _ = taxonomy; //Useless for this context but included to keep the API consistent.

  #[derive(Debug, Serialize)]
  struct Context {
    auth: UserOptionalContext,
    snippet: SnippetContext,
    is_admin: bool,
  }

  let snippet = Snippet::find_by_id(&conn, snippet_id)?;

  let context = Context {
    auth: user.to_context(),
    snippet: SnippetContext::from(&snippet),
    is_admin: user.is_admin(),
  };

  Ok(HttpResponse::Ok()
    .set(ContentType::html())
    .body(ctxt.render_template("snippet.html.tera", &context)))
}

fn parse_date(date: &str) -> Result<NaiveDateTime, DTParseError> {
    NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date), "%Y-%m-%d %H:%M:%S")
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
