mod profile;
mod judge;

mod status;
mod teams;


mod util;
mod database;
mod configuration;
mod problem_evaluation;
mod server;
pub(crate) mod collections;
// use programming_language_service::{language};
use crate::{profile::Profile, teams::Team};
use std::path::Path;
use crate::database::database::Database;
use crate::util::dependencies::verify_dependencies;
use crate::configuration::init;
use crate::judge::judge::ClientJudge;

#[tokio::main]
async fn main() {
    verify_dependencies(true, true);

    let db = Database::new().await.unwrap();

}

