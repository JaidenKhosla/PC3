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
fn main() {
    // let x: u8 = Team::generate("team_name".to_string(),8u8);

    // println!("{}", x);
    let path = Path::new("./CompetitionProfile");

    Profile::from(path).unwrap();
    // Team::from(Path::new(""))
}
