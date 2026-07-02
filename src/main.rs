mod profile;
mod judge;

mod status;
mod teams;


mod util;
mod database;
mod configuration;
mod problem_evaluation;
mod server;
// use programming_language_service::{language};
use crate::{profile::Profile, teams::Team};
use std::path::Path;
use crate::database::database::Database;
use crate::util::dependencies::verify_dependencies;
use crate::configuration::init;
use crate::judge::judge::ClientJudge;

#[tokio::main]
async fn main() {
 

}

