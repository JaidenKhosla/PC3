use std::{path::Path, process::Command, time::Duration};

use rand::{RngExt};
use sqlx::{Postgres, pool::{Pool, PoolConnection}, postgres::PgPoolOptions};

use crate::{Res, util::file_util::get_all_files};


macro_rules! URL_TEMPLATE  {
    () => {
        "postgresql://{}:{}@{}:{}/{}"
    };
}

#[allow(unused)]
const IMAGE: &str = "postgres:19beta1-alpine3.24";

#[allow(unused)]
const USER: &str = "server";
#[allow(unused)]
const ADDRESS: &str = "localhost";
#[allow(unused)]
const PORT: u16 = 15827; //Inteneral port will ALWAYS be 5432
#[allow(unused)]
const DATABASE_NAME: &str = "db";

#[allow(unused)]
const MAX_CONNECTIONS: u32 = 256;

const POOL_CONNECTION_ATTEMPTS: u32 = 150;

#[allow(unused)]
pub struct Database
{
    user: String,
    password: String,
    address: String,
    pub database_pool: Pool<Postgres>
}

impl Database
{
    #[allow(unused)]
    pub async fn new() -> Res![Self]
    {
        let password = Database::generate_password();

        let address = format!(URL_TEMPLATE!(), USER, password, ADDRESS, PORT, DATABASE_NAME);

        let mut cmd = Command::new("docker");
        let _ = cmd.args([
        "run", "--rm", "--name", &DATABASE_NAME, 
        "-e", &format!("{}{}", "POSTGRES_DB=", DATABASE_NAME), 
        "-e", &format!("{}{}", "POSTGRES_USER=", USER), 
        "-e", &format!("{}{}", "POSTGRES_PASSWORD=", password), 
        "-p", &format!("{}:5432", PORT), 
        "-d", IMAGE
        ]);



        let output = cmd.spawn().expect("Spawning command for database failed.").wait_with_output().expect("Database spin up command failed to execute.");
        
        println!("Output: {:?}\n Errors: {:?}", output.stdout, output.stderr);
        
        if !output.status.success() {
            panic!("docker run failed (status {}): {}", output.status, String::from_utf8_lossy(&output.stderr));
        }


        let mut database_pool = Database::connect_pool(&address).await.unwrap();
        let mut is_connected = false;

        Ok(
            Database
            {
                user: USER.to_string(),
                password: password.clone(),
                address,
                database_pool
            }
        )   
    }

    async fn connect_pool(address: &str) -> Res![Pool<Postgres>]
    {
        for attempt in 1..=POOL_CONNECTION_ATTEMPTS
        {
            let database_pool = PgPoolOptions::new().max_connections(MAX_CONNECTIONS).connect(&address).await;

            match database_pool
            {
                Ok(pool) => {
                    return Ok(pool);
                },
                Err(e) => {
                    println!("Failed to connect to pool on attempt #{}! Error: {}", attempt, e);

                    tokio::time::sleep(Duration::from_millis(500)).await;

                    continue;
                }
            }
        }

        Err("Failed to connect to pool!".to_string())
    }

    #[allow(unused)]
    pub async fn import(&self, dir_path: &Path) -> Res![_ ()]
    {
        let files = get_all_files(dir_path);

        for file in files
        {
            if file.extension().unwrap().to_str().unwrap() != "sql"
            {
                continue;
            };

            let mut cmd = Command::new("docker");
            cmd.args(["exec", "-it", &DATABASE_NAME, "psql", "-U", &USER, "-d", &DATABASE_NAME, "-f", file.to_str().unwrap()]);

            let output = cmd.spawn().expect("Failed to run execution of import!").wait_with_output().expect("Failed to yield output for the execution of import!");

            if !output.status.success()
            {
                panic!("Import execution failed! Status {} {}", output.status, String::from_utf8_lossy(&output.stderr));
            }

        }

        Ok(())
    }

    //pg_dump -U username -h localhost -d my_database --clean --if-exists > relational_backup.sql
    #[allow(unused)]
    pub async fn export(&self, dir_path: &Path) -> Res![_ ()]
    {
        let save_path = dir_path.join("save.sql");

        let mut cmd = Command::new("docker");
        cmd.args(["exec", "-it", &DATABASE_NAME, "pg_dump", "-U", &USER, "-p", &PORT.to_string(), "-h", &ADDRESS, "--clean", "--if-exists", "-f", save_path.to_str().unwrap()]);

        let output = cmd.spawn().expect("Failed to execute pg_dump!").wait_with_output().expect("Failed to yield output for pg_dump!");

        if !output.status.success()
        {
            panic!("Pg_dump failed! Status: {} {}", output.status, String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    #[allow(unused)]
    pub async fn borrow_connection(&self) -> Res![_ PoolConnection<Postgres>]
    {
        Ok(self.database_pool.acquire().await?)
    }

    #[allow(unused)]
    fn generate_password() -> String
    {
        rand::rng().sample_iter(rand::distr::Alphanumeric).take(32).map(char::from).collect()
    }
}

