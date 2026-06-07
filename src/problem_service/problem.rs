use std::option::Option;
use std::path::PathBuf;
use std::fmt::Error;
use std::str::FromStr;
use json::JsonValue;

use crate::{problem_service::problem::Status::TIME_LIMIT_EXCEEDED, programming_language_service::language::{self, Language}};

#[allow(non_camel_case_types)]
enum Status
{
    OK,
    ACCEPTED,
    COMPILATION_ERROR,
    RUNTIME_ERROR,
    WRONG_ANSWER,
    TIME_LIMIT_EXCEEDED,
    MEMORY_OVERFLOW,
}

impl FromStr for Status
{
    type Err = String;

    fn from_str(s: &str) -> Result<Status, Self::Err>
    {
        match s
        {
            "OK" => Ok(Status::OK),
            "ACCEPTED" => Ok(Status::ACCEPTED),
            "COMPILATION_ERROR" => Ok(Status::COMPILATION_ERROR),
            "RUNTIME_ERROR" => Ok(Status::RUNTIME_ERROR),
            "WRONG_ANSWER" => Ok(Status::RUNTIME_ERROR),
            "TIME_LIMIT_EXCEEDED" => Ok(Status::TIME_LIMIT_EXCEEDED),
            "MEMORY_OVERFLOW" => Ok(Status::MEMORY_OVERFLOW),
            _ => Err(format!("{} is an invalid status!", s))

        }
    }
}

pub struct Testcase
{
    input_path: String,
    output_path: String,
    time_limit: u32,
    memory_limit: u32
}

impl Testcase
{
    pub fn evaluate(&self, input_file: &PathBuf, language: &Language) -> Status
    {
        !todo!("Too lazy lol");
    }
}

pub struct JudgeProblem<'a>
{
    title: String,
    testcases: Vec<Testcase>,
    file_path: PathBuf,
    solution_path: PathBuf,
    solution_language: Option<Language<'a>>
}

impl JudgeProblem<'_>
{
    pub fn evaluate(&self, input_file: PathBuf, language: &Language) -> Vec<Status>
    {
        let mut result: Vec<Status> = Vec::new();

        for testcase in self.testcases.iter()
        {
            result.push(testcase.evaluate(&input_file, &language));
        }

        return result;
    }
}

pub struct ClientProblem
{
    name: String,
    attempts: u8,
    statuses: Vec<Status>,
    times: Vec<u32>
}

impl ClientProblem
{
    pub fn from(item: JsonValue) -> Result<Self, Error>
    {
        || -> Result<ClientProblem, Error> {
            
            Ok(
                ClientProblem
                {
                    name: item["name"].to_string(),
                    attempts: item["password"].as_u8().unwrap(),
                    statuses: item["statuses"].members().map(|x| Status::from_str(&x.to_string()).unwrap()).collect(),
                    times: item["times"].members().map(|x| x.as_u32().unwrap()).collect()
                }
            )
        }()
    }
}