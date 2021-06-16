// do not delete https://github.com/diesel-rs/diesel/issues/1894
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

mod application_context;
mod cli;
mod controllers;
mod db;
mod github_client;
mod helpers;
mod icons;
mod models;
mod schema;

use application_context::ApplicationContext;
use clap::Clap;
use cli::Opts;
use db::get_pool;
use dotenv::dotenv;
use github_client::GithubClient;
use std::{any::type_name, env, str::FromStr};

#[rocket::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // in a big Java app this would be Spring or Dagger, but we aren't a
    // big Java app nor do we have lots of dependencies to cart around,
    // so a simple immutable bean context is plenty adequate for
    // purpose.
    let db_pool = get_pool(&env_str("DATABASE_URL"), env_parse::<u32>("IDG_MAXDBCONNS"));
    let github_client = GithubClient::new(&env_str("GH_CLIENT_ID"), &env_str("GH_CLIENT_SECRET"));
    let application_context = ApplicationContext {
        db_pool,
        github_client,
    };

    let opts = Opts::parse();
    opts.do_the_thing(&application_context).await;
}

pub fn env_str(var: &str) -> String {
    env::var(var).expect(&format!(
        "Please provide {} as an environment var or in a .env",
        var
    ))
}

pub fn env_parse<T: FromStr>(var: &str) -> T {
    let s = env_str(var);
    match s.parse() {
        Ok(a) => a,
        _ => panic!(
            "String {} was unparsable to some FromtStr type {}",
            s,
            type_name::<T>()
        ),
    }
}
