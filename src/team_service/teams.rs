use core::fmt;
use std::path::Path;
use std::fs::{self, File};
use std::io::{BufReader, Error, Read};

pub(crate) use rand;
use rand::RngExt;


use json;



pub struct Team
{
    name: String, 
    password: String,
    score: i64,
    user_count: u8,
}

impl fmt::Debug for Team
{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
     {
        write!(f, "{} {} {} {}", self.name, self.password, self.score, self.user_count)
     }
}

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
            user_count: user_count
        }
    }

    pub fn from(path: &Path) -> Result<Vec<Team>, Error>
    {
        let result = || -> Result<Vec<Team>, Error> {
            
            let file = File::open(path).unwrap();
            let mut reader = BufReader::new(file);

            let mut buffer: Vec<u8> = Vec::new();

            reader.read_to_end(&mut buffer);

            let unparsed_json = String::from_utf8_lossy(&buffer);

            let parsed_json = json::parse(&unparsed_json).unwrap();

            let teams_vector: Vec<Team> = parsed_json.members().map(|team_json| {
                    Team
                    {
                        name: team_json["name"].to_string(),
                        password: team_json["password"].to_string(),
                        score: team_json["score"].as_i64().unwrap(),
                        user_count: team_json["user_count"].as_u8().unwrap()

                    }
            }).collect();

            
            Ok(teams_vector)

        };
        
        result()
    }
    
}