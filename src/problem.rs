use core::fmt;
use std::io::{BufWriter, Error, Write};
use std::option::Option;
use std::path::{Path, PathBuf};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::Arc;
use json::JsonValue;
use std::fs::{self, DirEntry, File};
use std::collections::HashSet;

use std::thread;
use std::time;

use crate::language::Language;
use crate::status::Status;
use crate::util::json_util::read_json;


#[allow(unused)]
pub struct Testcase
{
    input_path: PathBuf,
    output_path: PathBuf,
    time_limit: u64,
    memory_limit: u64
}

#[allow(unused)]
impl Testcase
{
    pub fn evaluate(&self, submission_path: &PathBuf, language: &Language) -> Status
    { 
        let safeLanguage = Arc::new(language);
        let safeSubmissionPath = Arc::new(submission_path);

        let starting_time = time::Instant::now();

        let evaluationThread = thread::spawn(|| -> Status {
            return Status::ACCEPTED;
        });

        while( !evaluationThread.is_finished() && starting_time.elapsed().as_secs() < self.time_limit)
        {}

        // if(!evaluationThread.is_finished())
        // {
        //      Status::TIME_LIMIT_EXCEEDED
        // }

        todo!("Nor finished");

        
    }

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
                    time_limit: parsed_config["time_limit"].as_u64().unwrap()
            };

            Ok(testcase)
        }();

        result
    }
}

#[allow(unused)]
pub struct JudgeProblem<'a>
{
    title: String,
    testcases: Vec<Testcase>,
    file_path: PathBuf,
    solution_path: Option<PathBuf>,
    solution_language: Option<Language<'a>>
}

#[allow(unused)]
impl JudgeProblem<'_>
{
    pub fn evaluate(&self, input_file: PathBuf, language: &Language, solution: Vec<u8>) -> Vec<Status>
    {
        let mut result: Vec<Status> = Vec::new();

        let extension = Language::language2file(&language);

        let submission_path = self.file_path.join(format!("{}.{}", self.title, extension));

        BufWriter::new(File::open(&submission_path).unwrap()).write(&solution);
        
        for testcase in self.testcases.iter()
        {
            result.push(testcase.evaluate(&submission_path, &language));
        }
        
        return result;
    }

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

            let language = Language::file2language(&solution_file_path);

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
    name: String,
    attempts: u8,
    statuses: Vec<Status>,
    times: Vec<u64>
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
}

impl Debug for ClientProblem
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result 
    {
        write!(f, "{} {}", self.name, self.attempts)
    }
}