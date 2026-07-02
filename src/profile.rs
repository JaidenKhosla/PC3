use std::io::{Write};
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use std::fs::{self, File};

use crate::configuration::get_configuration_directory;
use crate::util::file_util::copy_directory;
use crate::util::interfaces::Serializable;
use crate::{problem_evaluation::problem::JudgeProblem, teams::Team};
use crate::Res; 

use crate::util::json_util::read_json;

use json::{self, JsonValue};

#[allow(unused)]
const PROFILE_TEMPLATE: &str = "assets/CompetitionProfile";

#[allow(unused)]
pub struct Profile<'a>
{
    name: String,
    author: String,
    time_start: u64, //EPOCH Time
    time_left: u64, //in milliseconds

    teams: Vec<Team>,
    problems: HashMap<String, JudgeProblem<'a>>,
    root_filepath: PathBuf
}

impl<'a> Profile<'a>
{
    #[allow(unused)]
    pub fn new(name: String, author: String, time_start: u64, time_left: u64) -> Self
    {
        let mut name = name;
        
        let mut profile_directory = get_configuration_directory().join("profiles").join(&name);


        let mut count: u8 = 1;
        while profile_directory.exists()
        {
            profile_directory = get_configuration_directory().join("profiles").join(&name);
            name = format!("{}{}", &name, &count);

            count+=1;
        };


        let _ = copy_directory(&get_configuration_directory().join(&PROFILE_TEMPLATE), &profile_directory);

        // json::stringify(root)


        let profile = Profile
        {
            name,
            author,
            time_start,
            time_left,
            teams: vec![],
            problems: HashMap::new(),
            root_filepath: profile_directory
        };

        profile
    }

    #[allow(unused)]
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let config_json = self.to_json_value();
        let config_string = config_json.dump();
        let config_path = self.root_filepath.join("config.json");

        let mut config_file = File::create(&config_path).unwrap();

        let _ = config_file.write(config_string.as_bytes());

        Ok(())
    }
}


impl<'a> Serializable<Profile<'a>, bool> for Profile<'a>
{
    #[allow(unused)]
    fn from_path(path: &Path) -> Res![Profile<'a>]
    {
        let result = || -> Res![Profile<'a>]
        {
            let config_path = path.join("config.json");
            let team_path = path.join("teams.json");
    
            let config_json = read_json(config_path.as_path()).unwrap();

            let teams = Team::from_path(&path).unwrap();

            let problem_path = path.join("problems");

            let problems = fs::read_dir(problem_path).unwrap()
            .map(|file| file.unwrap().path())
            .map(|path| JudgeProblem::from(&path));

            let profile = Profile {
                    name: config_json["name"].as_str().unwrap().to_string(),
                    author: config_json["author"].as_str().unwrap().to_string(),
                    time_start: config_json["time_start"].as_u64().unwrap(),
                    time_left: config_json["time_left"].as_u64().unwrap(),

                    teams,
                    problems: HashMap::new(),
                    root_filepath: path.to_path_buf()
                };

            Ok(profile)
        }();

        result
    }

    fn from_json_value(_: JsonValue) -> Res![bool] {
        todo!("from_json_value is not implemented for profile!")
    }

    #[allow(unused)]
    fn to_json_value(&self) -> JsonValue
    {
        let mut json_object = JsonValue::new_object();

        json_object["name"] = JsonValue::String(self.name.clone());
        json_object["author"] = JsonValue::String(self.author.clone());
        json_object["time_start"] = JsonValue::Number(self.time_start.into());
        json_object["time_left"] = JsonValue::Number(self.time_left.into());
        json_object["teams"] = JsonValue::Array(self.teams.iter().map(|team| team.to_json_value()).collect());

        json_object
    }
}