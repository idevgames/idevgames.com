use actix_session::Session;
use actix_web::{http::header::ContentType, web, HttpResponse};
use serde::Serialize;

use crate::{
    application_context::ApplicationContext,
    helpers::user_optional::{UserOptional, UserOptionalContext},
};

// GET /control-plane
pub async fn show(
    ctxt: web::Data<ApplicationContext>,
    session: Session,
) -> Result<HttpResponse, super::HandlerError> {
    let conn = ctxt.db_pool.get()?;
    let user = UserOptional::from_session(&conn, &session)?;

    #[derive(Debug, Serialize)]
    struct Context {
        auth: UserOptionalContext,
    }

    let context = Context {
        auth: user.to_context(),
    };

    Ok(HttpResponse::Ok()
        .set(ContentType::html())
        .body(ctxt.render_template("control_plane.html.tera", &context)))
}
