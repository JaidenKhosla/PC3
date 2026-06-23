use futures::io;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{self, TcpStream}, process::Command};
use crate::{judge::language::Language, problem_evaluation::problem::{JudgeProblem, Testcase}, status::{self, Status}, util::dependencies::verify_dependencies};
use uuid::Uuid;

// use crate::profile::Profile;
const SHELL_CODE: &str = include_str!("judge_shell.rs");

use std::{collections::LinkedList, fmt::Error, fs::{self, File}, io::{Write}, path::PathBuf};
pub struct ServerJudge<'a> //what the server stores
{
    username: String,
    password: String,

    queue: LinkedList<JudgeProblem<'a>>
}

const DEFAULT_PORT: u32 = 51909;

const INTERNAL_PORT: u32 = 80;

pub struct ClientJudge //what each judge stores
{
    id: String,
    port: u32,
    judge_directory: PathBuf
}

impl ClientJudge
{
    pub fn new(port: Option<u32>, profile_path: &str) -> Self
    {
        let PORT = port.unwrap_or(DEFAULT_PORT);

        let id = Uuid::new_v4().to_string();

        let judge_directory = PathBuf::from(profile_path);

        ClientJudge { id, port: PORT, judge_directory: judge_directory }         
    }

    pub fn generate_configuration(&self) -> &Self
    {
        todo!("Generate configuration file");
    }

    pub async fn generate_shell(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let mut file = File::create("judge_shell.rs").unwrap();

        file.write_all(&SHELL_CODE.bytes().collect::<Vec<u8>>()).unwrap();

        let mut command = Command::new("rustc");

        command.arg("judge_shell.rs");

        command.spawn().unwrap().wait().await.unwrap();

        


        Ok(())
    }



    pub async fn spawn(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = verify_dependencies(false, true);        

        let mut cmd = Command::new("docker");

        cmd.args(["run", "--rm", "-d", "--name", &self.id, "-p", &format!("{}:{}", self.port, INTERNAL_PORT), "-v", &format!("{}:/judge", self.judge_directory.join("buffer").to_str().unwrap()), "judge", "sleep", "infinity"]);
        
        cmd.spawn().expect("Failed: running the container").wait().await.unwrap();

        Ok(())
    }

    #[allow(unused)]
    pub async fn evaluate_problem(&self, problem: &JudgeProblem<'_>, file_name: &str, contents: &[u8])
    {
        let problem_path = &problem.file_path;
        let submission_directory = problem_path.join("submission_buffer");

        fs::remove_dir_all(&submission_directory);
        fs::create_dir_all(&submission_directory);
        
        let submission_file_path = submission_directory.join(&file_name);

        let mut submission_file = File::create(&submission_file_path).unwrap();

        let _ = submission_file.write_all(&contents).unwrap();

        // let language = Language::file2language(&submission_file_path);

        let IP_ADDRESS = format!("localhost:{}", &self.port);
        let programming_language_extension = submission_file_path.extension().unwrap().to_string_lossy().to_string();
        let programming_language = Language::extension2language(&programming_language_extension).unwrap();

        let mut tcp_stream = TcpStream::connect(IP_ADDRESS).await.unwrap();

        let compilation_needed = !programming_language.compiliation_code.is_empty();

        // let buffered_reader = BufReader::new;
        if compilation_needed
        {
            let message = programming_language.get_compilation_line(submission_file_path.to_str().unwrap());
            
            println!("Compiling {}", &problem.title  );

            let output = ClientJudge::send_message(&mut tcp_stream, &message, programming_language.compilation_time).await.unwrap();

            println!("Compilation Message: {}", &output);
            
        }

        for testcase in problem.testcases.iter()
        {
            let Testcase { input_path, output_path, time_limit, memory_limit } = &testcase;
        
            let message = programming_language.get_execution_line(submission_file_path.to_str().unwrap());

            println!("Running {}", &input_path.to_str().unwrap());
            
            let output = ClientJudge::send_message(&mut tcp_stream, &message, *time_limit).await.unwrap();
            
            println!("Output: {}", &output);

        }
    }


    async fn send_message(stream: &mut TcpStream, content: &str, time_limit: u32) -> Result<String, Box<dyn std::error::Error>>
    {
        stream.write(&content.as_bytes()).await.unwrap();

        stream.flush().await.unwrap();

        let mut buff: Vec<u8> = vec![];

        let mut reader = BufReader::new(stream);

        let _ = reader.read_until(b'\0', &mut buff).await.unwrap();
        
        Ok(String::from_utf8(buff).unwrap())
    }
}