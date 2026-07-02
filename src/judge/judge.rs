use std::{collections::LinkedList, fs::{self, File}, io::Write, ops::Add, path::{PathBuf, Path}, time::Duration};
use futures::{FutureExt, StreamExt};
use json::JsonValue;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpStream}, process::Command};
use crate::{judge::language::Language, problem_evaluation::problem::{JudgeProblem, Testcase}, status::Status, util::{dependencies::verify_dependencies, file_util::get_all_files, interfaces::Serializable, json_util::read_json}};
use uuid::Uuid;

use crate::Res;

#[allow(unused)]
const DEFAULT_PORT: u32 = 51909;

#[allow(unused)]
const INTERNAL_PORT: u32 = 80;

#[allow(unused)]
const SHELL_CODE: &str = include_str!("judge_shell.rs");

#[allow(unused)]
pub struct ServerJudge<'a> //what the server stores
{
    pub username: String,
    password: String,
    
    pub queue: LinkedList<JudgeProblem<'a>>
}

impl<'a> Serializable<ServerJudge<'a>, bool> for ServerJudge<'a>
{
    fn from_path(path: &Path) -> Res![ServerJudge<'a>]
    {
        let root_path = path.parent().unwrap();
        let problem_dir_path = root_path.join("problems");

        let problem_dir = get_all_files(&problem_dir_path);

        let json_object = read_json(path).unwrap();

        let username = json_object["username"].to_string();
        let password = json_object["password"].to_string();
        let queue = LinkedList::from_iter(problem_dir.iter().map(|problem_path| JudgeProblem::from(problem_path).unwrap()));

        Ok(
            ServerJudge { username, password, queue }
        )
    }

    fn to_json_value(&self) -> JsonValue
    {
        let mut json_object = JsonValue::new_object();

        json_object["username"] = JsonValue::String(self.username.to_string());
        json_object["password"] = JsonValue::String(self.password.to_string());

        json_object
    }

    fn from_json_value(_: JsonValue) -> Res![bool]
    {
        todo!("from_json_value is not implemented by ServerJudge!")
    }
}
 

pub struct ClientJudge //what each judge stores
{
    id: String,
    port: u32,
    judge_directory: PathBuf
}

impl ClientJudge
{
    #[allow(unused)]
    pub fn new(port: Option<u32>, profile_path: &str) -> Self
    {
        let port = port.unwrap_or(DEFAULT_PORT);
        
        let id = Uuid::new_v4().to_string();
        
        let judge_directory = PathBuf::from(profile_path);
        
        ClientJudge { id, port, judge_directory: judge_directory }         
    }
    
    #[allow(unused)]
    pub fn generate_configuration(&self) -> &Self
    {
        todo!("Generate configuration file");
    }
    
    #[allow(unused)]
    pub async fn generate_shell(&self) -> Result<&Self, Box<dyn std::error::Error>>
    {
        let mut file = File::create("judge_shell.rs").unwrap();

        file.write_all(&SHELL_CODE.bytes().collect::<Vec<u8>>()).unwrap();

        let mut command = Command::new("rustc");
        
        command.arg("judge_shell.rs");
        
        command.spawn().unwrap().wait().await.unwrap();
        
        fs::copy("judge_shell.exe", &self.judge_directory).expect("Judge Shell Copy Failure: ");
        
        Ok(self)
    }
    
    
    
    #[allow(unused)]
    pub async fn spawn(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = verify_dependencies(false, true);        
        
        let mut cmd = Command::new("docker");

        cmd.args(["run", "--rm", "-d", "--name", &self.id, "-p", &format!("{}:{}", self.port, INTERNAL_PORT), "-v", &format!("{}:/judge", self.judge_directory.to_str().unwrap()), "judge", "sleep", "infinity"]);
        
        let id = String::from_utf8_lossy(&cmd.spawn().expect("Failed: running the container").wait_with_output().await.unwrap().stdout).to_string();
        
        
        let mut cmd = Command::new("docker");
        
        cmd.args(["exec", &id, "./judge_shell"]);
        
        cmd.spawn().expect("Issue trying to execute shell on docker container").wait().await.unwrap();
        
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
        
        let ip_address = format!("localhost:{}", &self.port);
        let programming_language_extension = submission_file_path.extension().unwrap().to_string_lossy().to_string();
        let programming_language = Language::extension2language(&programming_language_extension).unwrap();
        
        let mut tcp_stream = TcpStream::connect(ip_address).await.unwrap();
        
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
    
    
    async fn send_message(stream: &mut TcpStream, content: &str, time_limit: u32) -> Result<String, Status>
    {
    
        stream.write(&content.as_bytes()).await.unwrap();
        
        stream.flush().await.unwrap();
        
        let mut buff: Vec<u8> = vec![];
        
        let mut reader = BufReader::new(stream);
        
        let contents = reader.read_until(b'\0', &mut buff).into_stream();
        
        tokio::pin!(contents); 

        let future_time = std::time::Instant::now().add(Duration::new(time_limit.into(),0));

        while let Some(_) = contents.next().await && std::time::Instant::now() <= future_time {}

        if std::time::Instant::now() > future_time
        {
            Err(Status::TIME_LIMIT_EXCEEDED)
        }
        else {
            Ok(String::from_utf8(buff).unwrap())
        }
        
    }
}
