use serde::Serialize;
use tera::{Context, Tera};

use crate::{db::DbPool, github_client::GithubClient};

#[derive(Clone)]
pub struct ApplicationContext {
    pub github_client: GithubClient,
    pub db_pool: DbPool,
    pub tera: Tera,
}

impl ApplicationContext {
    pub fn render_template(
        &self,
        template_name: &str,
        template_context: &impl Serialize,
    ) -> String {
        self.tera
            .render(
                template_name,
                &Context::from_serialize(template_context).unwrap(),
            )
            .expect("Could not render template")
    }
}
