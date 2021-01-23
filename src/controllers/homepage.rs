use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Serialize;

use crate::helpers::user_optional::{UserOptional, UserOptionalContext};
use crate::{application_context::ApplicationContext, helpers::snippets::SnippetList};

// GET /
pub async fn homepage(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = UserOptional::from_session(&conn, &session)?;
    let show_controls = user.is_admin();
    let udevgames = SnippetList::new(&conn, 0, 3, "udevgames", show_controls, false, true)?;
    let links = SnippetList::new(&conn, 0, 5, "links", show_controls, false, true)?;

    #[derive(Debug, Serialize)]
    struct Context {
        auth: UserOptionalContext,
        show_controls: bool,
        udevgames: SnippetList,
        links: SnippetList,
    }

    let context = Context {
        auth: user.to_context(),
        show_controls,
        udevgames: udevgames,
        links: links,
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("homepage.html.tera", &context)))
}
