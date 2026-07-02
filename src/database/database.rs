use std::{fmt::format, process::Command};

use rand::{RngExt, distr::Alphanumeric};
use sqlx;

use crate::Res;


macro_rules! URL_TEMPLATE  {
    () => {
        "postgresql://{}:{}@{}:{}/{}"
    };
}

const IMAGE: &str = "postgres:19beta1-alpine3.24";

const USER: &str = "SERVER";
const ADDRESS: &str = "localhost";
const PORT: u16 = 5432; //Inteneral port will ALWAYS be 5432
const DATABASE_NAME: &str = "db";

pub struct Database
{
    user: String,
    password: String,
    address: String
}

impl Database
{
    pub fn new() -> Self
    {
        let password = Database::generate_password();

        Database
        {
            user: USER.to_string(),
            password: password.clone(),
            address: format!(URL_TEMPLATE!(), USER, password, ADDRESS, PORT, DATABASE_NAME)
        }

        
    }

    pub async fn spin_up(&self) -> Res![&Self]
    {
        let mut cmd = Command::new("docker");
        let _ = cmd.args(["run", "--name", &DATABASE_NAME, "-e", &format!("{}{}", "POSTGRES_DB=", DATABASE_NAME), "-e", &format!("{}{}", "POSTGRES_USER=", USER), "-e", &format!("{}{}", "POSTGRES_PASSWORD=", self.password), "-p", &format!("{}:5432", PORT), "-d", IMAGE]);

        cmd.spawn().expect("Spawning command for database failed.").wait().expect("Database spin up command failed to execute.");

        Ok(self)
    }

    fn generate_password() -> String
    {
        rand::rng().sample_iter(Alphanumeric).take(32).map(char::from).collect()
    }
}

