use std::process::exit;

use crate::{
    application_context::ApplicationContext,
    db::DbConn,
    models::github_user_records::GithubUserRecord,
    models::{permissions::Permission as PermissionModel, users::User, ModelError},
};
use clap::Clap;
use diesel::Connection;

/// Grants a permission to a user
#[derive(Debug, Clap)]
struct PermissionGrant {
    /// The user to grant the permission to, either by @login or numeric id
    #[clap(short, long)]
    user: String,

    /// The permission to grant to the user
    #[clap(short, long)]
    permission: String,
}

impl PermissionGrant {
    async fn grant(&self, ctxt: &ApplicationContext) {
        let github_user_record = GithubUserRecord::find_by_login(&get_connection(ctxt), &self.user)
            .expect("Could not query the database");
        let (_, user) = match github_user_record {
            Some(gu) => {
                let u = gu
                    .get_user(&get_connection(ctxt))
                    .expect("Could not retrieve user from database");
                (gu, u)
            }
            None => {
                let user_detail = ctxt
                    .github_client
                    .get_user_detail_by_login(&self.user)
                    .await
                    .expect("Unable to query Github API");
                let conn = get_connection(&ctxt.clone());
                conn.transaction::<(GithubUserRecord, User), ModelError, _>(|| {
                    let u = User::create(&conn)?;
                    let gu = GithubUserRecord::find_and_update(
                        &conn,
                        user_detail.id,
                        u.id,
                        &user_detail.login,
                        &user_detail.avatar_url,
                        &user_detail.html_url,
                    )?;
                    Ok((gu, u))
                })
                .expect("Could not save things to the database!")
            }
        };

        PermissionModel::grant_permission(&get_connection(ctxt), user.id, &self.permission)
            .expect("Unable to grant permission");

        println!("Permission granted!")
    }
}

/// Revokes a permission from a user
#[derive(Debug, Clap)]
struct PermissionRevoke {
    /// The user to revoke the permission from, as known by their github login.
    #[clap(short, long)]
    user: String,

    /// The permission to revoke from the user
    #[clap(short, long)]
    permission: String,
}

impl PermissionRevoke {
    fn revoke(&self, ctxt: &ApplicationContext) {
        let conn = &ctxt
            .db_pool
            .read()
            .get()
            .expect("Couldn't get a connection from the pool");
        let github_user = {
            let gu = GithubUserRecord::find_by_login(&conn, &self.user)
                .expect("Could not query the database!");

            match gu {
                Some(u) => u,
                None => {
                    // It's reasonable to expect that someone might type the
                    // wrong name in here, so favor the shorter error message
                    // without the stack trace associated with a panicking
                    // unwrap/expect.
                    eprint!("No such user {} exists in our records!", &self.user);
                    exit(-1);
                }
            }
        };

        let user = github_user
            .get_user(&conn)
            .expect("Could not query the database or no such user found");

        PermissionModel::revoke_permission(&conn, user.id, &self.permission)
            .expect("Could not revoke permission");
    }
}

/// Show permissions for a user, or users with a permission
#[derive(Debug, Clap)]
struct PermissionShow {
    /// Show all permissions for this user, as named by github login.
    #[clap(long, short)]
    user: Option<String>,

    /// Show all users with this permission
    #[clap(short, long)]
    permission: Option<String>,
}

impl PermissionShow {
    fn show(&self, ctxt: &ApplicationContext) {
        let conn = &ctxt
            .db_pool
            .read()
            .get()
            .expect("Couldn't get a connection to the database out of the pool");

        if let Some(user) = &self.user {
            let gu = GithubUserRecord::find_by_login(&conn, &user)
                .expect("Could not query the database")
                .expect("No such user with that login");
            let permissions = PermissionModel::find_by_user_id(&conn, gu.user_id)
                .expect("Could not query the database");

            println!("Permissions for user {}:", gu.login);

            for permission in permissions {
                println!("- {}", permission.name);
            }
        } else if let Some(permission) = &self.permission {
            let permissions = PermissionModel::find_by_name(&conn, &permission)
                .expect("Unable to query the database");
            let users = permissions.iter().map(|permission| {
                GithubUserRecord::find_by_user_id(&conn, permission.user_id)
                    .expect("Unable to query the database")
            });

            println!("Users with permission {}:", permission);

            for user in users {
                if let Some(user) = user {
                    println!("- {}", user.login);
                } else {
                    println!("- missing user");
                }
            }
        }
    }
}

#[derive(Debug, Clap)]
enum PermissionSubCommand {
    Grant(PermissionGrant),
    Revoke(PermissionRevoke),
    Show(PermissionShow),
}

/// Grant, revoke, and show permissions given to users
#[derive(Debug, Clap)]
pub struct Permission {
    #[clap(subcommand)]
    subcmd: PermissionSubCommand,
}

impl Permission {
    pub async fn do_the_thing(&self, ctxt: &ApplicationContext) {
        match &self.subcmd {
            PermissionSubCommand::Grant(g) => {
                g.grant(&ctxt).await;
            }
            PermissionSubCommand::Revoke(r) => {
                r.revoke(&ctxt);
            }
            PermissionSubCommand::Show(s) => {
                s.show(&ctxt);
            }
        }
    }
}

fn get_connection(ctxt: &ApplicationContext) -> DbConn {
    ctxt.db_pool
        .read()
        .get()
        .expect("Could not get a connection from the pool")
}
