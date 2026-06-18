mod language_list;
mod language;

mod problem;
mod profile;
mod judge;

mod status;
mod teams;


mod util;
mod database;
// use programming_language_service::{language};
use crate::{profile::Profile, teams::Team};
use std::path::Path;
use crate::database::database::Database;
use crate::util::dependencies::verify_dependencies;

#[tokio::main]
async fn main() {
    // let x: u8 = Team::generate("team_name".to_string(),8u8);

    // println!("{}", x);
    println!("{:?}",verify_dependencies(true).await);
    // Team::from(Path::new(""))
}
