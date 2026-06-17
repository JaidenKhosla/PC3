use core::fmt;
use std::path::Path;
use std::io::Error;
use std::collections::HashMap;

use rand;
use rand::RngExt;

use crate::problem::ClientProblem;
use crate::util::json_util;



pub struct Team
{
    name: String, 
    password: String,
    score: i64,
    user_count: u8,
    problem_history: HashMap<String, ClientProblem>
}

impl fmt::Debug for Team
{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
     {
        write!(f, "{} {} {} {} {:?}", self.name, self.password, self.score, self.user_count, self.problem_history)
     }
}

#[allow(unused)]
impl Team
{
    pub fn generate(team_name: String, length_of_password: u8, user_count: u8) -> Team
    {
        let mut rng = rand::rng();

        let mut password = String::new();

        for _ in 0..length_of_password
        {
            password.push(
                rng.sample(rand::distr::Alphanumeric) as char
            )
        }

        Team {
            name: team_name,
            password: password,
            score: 0,
            user_count: user_count,
            problem_history: HashMap::new()
        }
    }

    pub fn from(path: &Path) -> Result<Vec<Team>, Error>
    {
        let result = || -> Result<Vec<Team>, Error> {
            
            let parsed_json = json_util::read_json(path).unwrap();

            let teams_vector: Vec<Team> = parsed_json.members().map(|team_json| {
                    Team
                    {
                        name: team_json["name"].to_string(),
                        password: team_json["password"].to_string(),
                        score: team_json["score"].as_i64().unwrap(),
                        user_count: team_json["user_count"].as_u8().unwrap(),
                        problem_history: team_json["problem_history"].members().map(|problem| (problem["name"].to_string(), ClientProblem::from(problem.to_owned()).unwrap())).collect()
                    }
            }).collect();

            
            Ok(teams_vector)

        };
        
        result()
    }
}