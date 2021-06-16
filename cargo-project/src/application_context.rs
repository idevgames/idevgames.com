use crate::{db::DbPool, github_client::GithubClient};

#[derive(Clone)]
pub struct ApplicationContext {
    pub github_client: GithubClient,
    pub db_pool: DbPool,
}
