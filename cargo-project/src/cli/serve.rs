use crate::application_context::ApplicationContext;
use clap::Clap;
use rocket::{config::Config as RocketConfig, figment::Figment, routes};

/// Start the iDevGames website
#[derive(Clap, Debug)]
pub struct Serve {}

impl Serve {
    pub async fn serve(&self, ctxt: &ApplicationContext) {
        let secret = crate::env_str("IDG_COOKIE_SECRET");
        let address = crate::env_str("IDG_ADDRESS");
        let port = crate::env_parse::<u16>("IDG_PORT");

        let config = Figment::from(RocketConfig::default())
            .merge(("address", address))
            .merge(("port", port))
            .merge(("secret_key", secret));

        let _ = rocket::custom(config)
            .manage(ctxt.clone())
            .mount(
                "/api",
                routes![
                    // GET      /api/session
                    crate::controllers::auth::get_session,
                    // GET      /api/session/github_authorization_url
                    crate::controllers::auth::github_authorization_url,
                    // GET      /api/session/github_callback?<code>
                    crate::controllers::auth::github_callback,
                    // DELETE   /api/session
                    crate::controllers::auth::logout,
                    // GET      /api/snippets?<taxonomy>&<page>&<show_hidden>
                    crate::controllers::snippets::get_snippets,
                    // GET      /api/snippets/<snippet_id>
                    crate::controllers::snippets::get_snippet,
                    // POST     /api/snippets
                    crate::controllers::snippets::create_snippet,
                    // PUT      /api/snippets/<snippet_id>
                    crate::controllers::snippets::update_snippet,
                    // DELETE   /api/snippets/<snippet_id>
                    crate::controllers::snippets::delete_snippet,
                ],
            )
            .launch()
            .await;
    }
}
