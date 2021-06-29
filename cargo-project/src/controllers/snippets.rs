use crate::{
    application_context::ApplicationContext,
    db::DbConn,
    helpers::{admin_only::AdminOnly, maybe_user::MaybeUser},
    models::{ModelError, Snippet},
};
use chrono::NaiveDateTime;
use rocket::{delete, get, post, put, serde::json::Json, State};
use serde::{Deserialize, Serialize};

/* #region GetSnippets */

#[get("/snippets?<taxonomy>&<page>&<show_hidden>")]
pub async fn get_snippets(
    user: MaybeUser,
    ctxt: &State<ApplicationContext>,
    taxonomy: &str,
    page: i32,
    show_hidden: bool,
) -> Result<Json<GetSnippetsOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.read().get()?;
    let show_hidden = user.is_admin() && show_hidden;
    let page_size = 5;

    let snippets =
        GetSnippetsOutput::new(&conn, page, page_size, taxonomy, !show_hidden)?;

    Ok(Json(snippets))
}

#[derive(Debug, Serialize)]
pub struct GetSnippetsOutput {
    snippets: Vec<Snippet>,
    current_page: i32,
    total_pages: i64,
}

impl GetSnippetsOutput {
    pub fn new(
        conn: &DbConn,
        page: i32,
        page_size: i32,
        taxonomy: &str,
        visible_only: bool,
    ) -> Result<Self, ModelError> {
        let snippets = Snippet::find_all_by_taxonomy(
            conn,
            visible_only,
            taxonomy,
            page.into(),
            page_size.into(),
        )?;
        let snippet_count = Snippet::count(conn, visible_only, taxonomy)?;
        let total_pages = std::cmp::max((snippet_count as f32 / page_size as f32).ceil() as i64, 1);

        Ok(Self {
            snippets: snippets,
            current_page: page,
            total_pages,
        })
    }
}

/* #endregion */
/* #region GetSnippet */

#[get("/snippets/<snippet_id>")]
pub async fn get_snippet(
    user: MaybeUser,
    ctxt: &State<ApplicationContext>,
    snippet_id: i32,
) -> Result<Json<GetSnippetOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.read().get()?;
    let can_view_hidden = user.is_admin();

    let snippet = Snippet::find_by_id(&conn, snippet_id)?;

    if snippet.hidden && !can_view_hidden {
        Err(super::HandlerError::NotFound)
    } else {
        Ok(Json(GetSnippetOutput { snippet }))
    }
}

#[derive(Debug, Serialize)]
pub struct GetSnippetOutput {
    snippet: Snippet,
}

/* #endregion */
/* #region CreateSnippet */

#[post("/snippets", data = "<input>")]
pub async fn create_snippet(
    user: AdminOnly,
    ctxt: &State<ApplicationContext>,
    input: Json<CreateSnippetInput>,
) -> Result<Json<CreateSnippetOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.read().get()?;

    let snippet = Snippet::create(
        &conn,
        user.user.0.id,
        &input.taxonomy,
        input.hidden,
        &input.icon,
        &input.title,
        &input.shared_by,
        &input.shared_on,
        &input.summary,
        &input.description,
        &input.href,
    )?;

    Ok(Json(CreateSnippetOutput { snippet }))
}

#[derive(Debug, Deserialize)]
pub struct CreateSnippetInput {
    taxonomy: String,
    hidden: bool,
    title: String,
    icon: String,
    shared_by: String,
    shared_on: NaiveDateTime,
    summary: String,
    description: String,
    href: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSnippetOutput {
    snippet: Snippet,
}

/* #endregion */
/* #region UpdateSnippet */

#[put("/snippets/<snippet_id>", data = "<input>")]
pub async fn update_snippet(
    _user: AdminOnly,
    ctxt: &State<ApplicationContext>,
    snippet_id: i32,
    input: Json<UpdateSnippetInput>,
) -> Result<Json<UpdateSnippetOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.read().get()?;
    let mut snippet = Snippet::find_by_id(&conn, snippet_id)?;

    snippet.hidden = input.hidden;
    snippet.title = input.title.clone();
    snippet.icon = input.icon.clone();
    snippet.shared_by = input.shared_by.clone();
    snippet.shared_on = input.shared_on.clone();
    snippet.summary = input.summary.clone();
    snippet.description = input.description.clone();
    snippet.href = input.href.clone();

    snippet.update(&conn)?;

    Ok(Json(UpdateSnippetOutput {}))
}

#[derive(Debug, Deserialize)]
pub struct UpdateSnippetInput {
    taxonomy: String,
    hidden: bool,
    title: String,
    icon: String,
    shared_by: String,
    shared_on: NaiveDateTime,
    summary: String,
    description: String,
    href: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateSnippetOutput {}

/* #endregion */
/* #region DeleteSnippet */

// DELETE /snippets/{taxonomy}/{snippet_id} delet this pls
#[delete("/snippets/<snippet_id>")]
pub async fn delete_snippet(
    _user: AdminOnly,
    ctxt: &State<ApplicationContext>,
    snippet_id: i32,
) -> Result<Json<DeleteSnippetOutput>, super::HandlerError> {
    let conn = ctxt.db_pool.read().get()?;
    let snippet = Snippet::find_by_id(&conn, snippet_id)?;

    snippet.delete(&conn)?;

    Ok(Json(DeleteSnippetOutput {}))
}

#[derive(Debug, Serialize)]
pub struct DeleteSnippetOutput {}

/* #endregion */
