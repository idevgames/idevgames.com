use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Clap;

use crate::application_context::ApplicationContext;

/// Start the iDevGames website
#[derive(Clap, Debug)]
pub struct Serve {}

impl Serve {
    pub async fn serve(&self, ctxt: &ApplicationContext) {
        let secret = crate::env_str("IDG_COOKIE_SECRET");
        let address = crate::env_str("IDG_ADDRESS");
        let port = crate::env_parse::<u16>("IDG_PORT");

        let application_context = web::Data::new(ctxt.clone());

        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                /* TODO: .wrap(spehss helmet security headers)*/
                /* TODO: it would be nice to conditionally enable secure(true) when
                running in prod */
                .wrap(CookieSession::signed(&secret.as_bytes()).secure(false))
                .app_data(application_context.clone())
                .route("/", web::get().to(crate::controllers::homepage::homepage))
                .route("/login", web::get().to(crate::controllers::auth::login))
                .route(
                    "/github_callback",
                    web::get().to(crate::controllers::auth::github_callback),
                )
                .route(
                    "/logout",
                    web::delete().to(crate::controllers::auth::logout),
                )
                .service(Files::new("/static", "static"))
        })
        .bind(format!("{}:{}", address, port))
        .expect("Could not bind to address/port")
        .run()
        .await
        .unwrap();
    }
}
