mod programming_language_service;
mod team_service;
mod problem_service;

use team_service::teams::Team;

use std::{io::Read, process::Command};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

use programming_language_service::{language};

fn main() {
    // let x: u8 = Team::generate("team_name".to_string(),8u8);

    // println!("{}", x);
    let path = Path::new("./CompetitionProfile/teams.json");

    println!("{:?}",Team::from(path).unwrap());
    // Team::from(Path::new(""))
}
