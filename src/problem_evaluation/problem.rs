use core::fmt;
use std::io::Error;
use std::option::Option;
use std::path::{Path, PathBuf};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use json::{JsonValue, number::Number};
use std::fs::{self, DirEntry};
use std::collections::HashSet;

use crate::judge::language::Language;
use crate::status::Status;
use crate::util::json_util::read_json;


#[allow(unused)]
pub struct Testcase
{
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub time_limit: u32,
    pub memory_limit: u64
}

#[allow(unused)]
impl Testcase
{
    pub fn from(root: &Path, name: &str) -> Result<Self, Error>
    {
        // let testcases_folder = root.join(path)
        let result = || -> Result<Self, Error>
        {
            let testcase_config = root.join(format!("{}.json", name));
            let input_file = root.join(format!("{}.in", name));
            let output_file = root.join(format!("{}.out", name));

            let parsed_config = read_json(&testcase_config.as_path()).unwrap();

            let testcase: Testcase = Testcase {
                    input_path: input_file,
                    output_path: output_file,
                    memory_limit: parsed_config["memory_limit"].as_u64().unwrap(),
                    time_limit: parsed_config["time_limit"].as_u32().unwrap()
            };

            Ok(testcase)
        }();

        result
    }
}

#[allow(unused)]
pub struct JudgeProblem<'a>
{
    pub title: String,
    pub testcases: Vec<Testcase>,
    pub file_path: PathBuf,
    solution_path: Option<PathBuf>,
    solution_language: Option<Language<'a>>
}

#[allow(unused)]
impl JudgeProblem<'_>
{

    pub fn from(root: PathBuf) -> Result<Self, Error>
    {
        // let root = root.clone();
        let file_path = root.clone();

        || -> Result<JudgeProblem, Error> {
            let title: String = root.file_name().unwrap().to_string_lossy().to_string();
            let file_path = root.to_path_buf();
            
            let testcases_path = root.join("testcases");

            let mut testcase_names: HashSet<String> = HashSet::new();

            let testcases = testcase_names.iter().map(|file_name| Testcase::from(&testcases_path, file_name).unwrap()).collect::<Vec<Testcase>>();

            fs::read_dir(testcases_path).unwrap()
            .map(|unparsed_file| unparsed_file.unwrap().file_name().to_string_lossy().to_string())
            .for_each(|file| {testcase_names.insert(file);});

            // let language = Language::file2language()

            let files = fs::read_dir(root.join("solution")).unwrap().map(|file| file.unwrap()).filter(|file| file.metadata().unwrap().is_file() && file.file_name().to_string_lossy().to_string() == title).collect::<Vec<DirEntry>>();

            let solution_file = &files[0];

            let solution_file_path = solution_file.path();

            let language = Language::file2language(&solution_file_path).unwrap();

            // let root_ref = root.clone();

            Ok(
                JudgeProblem
                {
                    title: title,
                    testcases: testcases,
                    file_path: file_path,
                    solution_path: Some(solution_file_path),
                    solution_language: Some(language)
                }
            )

        }()
    }
}

#[allow(unused)]
pub struct ClientProblem
{
    pub name: String,
    pub attempts: u8,
    pub statuses: Vec<Status>,
    pub times: Vec<u64>
}

#[allow(unused)]
impl ClientProblem
{
    pub fn from(item: JsonValue) -> Result<Self, Error>
    {
        || -> Result<ClientProblem, Error> {
        
            Ok(
                ClientProblem
                {
                    name: item["name"].to_string(),
                    attempts: item["attempts"].as_u8().unwrap(),
                    statuses: item["statuses"].members().map(|x| Status::from_str(&x.to_string()).unwrap()).collect(),
                    times: item["times"].members().map(|x| x.as_u64().unwrap()).collect()
                }
            )
        }()
    }

    pub fn to_json_value(&self) -> JsonValue
    {
        let mut problem_object = JsonValue::new_object();
        problem_object["name"] = JsonValue::String(self.name.clone());
        problem_object["attempts"] = JsonValue::Number(self.attempts.into());
        problem_object["statuses"] = JsonValue::Array(self.statuses.iter().map(|x| JsonValue::String(x.to_string())).collect());
        problem_object["times"] = JsonValue::Array(self.times.iter().map(|x| JsonValue::Number(Number::from(*x))).collect());


        problem_object
    }
}

impl Debug for ClientProblem
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        write!(f, "{} {}", self.name, self.attempts)
    }
}