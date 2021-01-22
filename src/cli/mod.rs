mod migrate;
mod permission;
mod serve;

use crate::application_context::ApplicationContext;
use clap::{crate_authors, crate_version, Clap};

use self::{migrate::Migrate, permission::Permission, serve::Serve};

#[derive(Clap, Debug)]
enum SubCommand {
    Migrate(Migrate),
    Permission(Permission),
    Serve(Serve),
}

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub async fn do_the_thing(&self, ctxt: &ApplicationContext) {
        match &self.subcmd {
            SubCommand::Migrate(m) => m.migrate(&ctxt),
            SubCommand::Permission(p) => p.do_the_thing(&ctxt).await,
            SubCommand::Serve(s) => s.serve(&ctxt).await,
        }
    }
}
