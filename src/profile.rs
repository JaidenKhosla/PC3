use std::io::Error;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;

use crate::{problem::JudgeProblem, teams::Team};
use crate::util::json_util::read_json;

#[allow(unused)]
pub struct Profile<'a>
{
    time_start: u64, //EPOCH Time
    time_left: u64, //in milliseconds

    teams: Vec<Team>,
    problems: HashMap<String, JudgeProblem<'a>>,
    root_filepath: PathBuf
}

impl<'a> Profile<'a>
{
    #[allow(unused)]
    pub fn from(path: PathBuf) -> Result<Self, Error>
    {
        let result = || -> Result<Self, Error>
        {
            let config_path = path.join("config.json");
            let team_path = path.join("teams.json");
    
            let config_json = read_json(config_path.as_path()).unwrap();

            let teams = Team::from(team_path.as_path()).unwrap();

            let problem_path = path.join("problems");

            let problems = fs::read_dir(problem_path).unwrap()
            .map(|file| file.unwrap().path())
            .map(|path| JudgeProblem::from(path));
            ;

            let profile = Profile {
                    time_start: config_json["time_start"].as_u64().unwrap(),
                    time_left: config_json["time_left"].as_u64().unwrap(),

                    teams: Team::from(team_path.as_path()).unwrap(),
                    problems: HashMap::new(),
                    root_filepath: path
                };

            Ok(profile)
        }();

        result
    }
}