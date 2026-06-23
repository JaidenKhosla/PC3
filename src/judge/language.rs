use std::path::Path;
pub(crate) use std::process::Command;
// use
use crate::judge::language_list;
/*(
Execution Code:

{FILE_NAME}
{FILE_NAME_WITH_EXTENSION}
{PATH}
{PARENT PATH}

) */
#[derive(PartialEq)]
pub struct Language<'a>
{
    pub title: &'a str,
    pub header_command: &'a str,
    pub compiliation_code: &'a str,
    pub running_code: &'a str,
    pub compilation_time: u32
    // pub installation_code: String,
    // timeout_period: u32 //in seconds/
}

#[allow(unused)]
impl<'a> Language<'a>
{
    pub fn exists_within_device(&self) -> bool 
    {
        return Command::new(&self.header_command).output().is_ok();
    }
    
    pub fn execute(&self, file_path: &str, compile: bool, execute: bool, stdin: Option<&str>) -> Result<String, String>
    {
        if(!self.exists_within_device())
        {
            return Err(format!(
            r#"Programming Language doesn't exist on device. 
            PROGRAMMING NAME: {}
            HEADER_COMMAND: {}"#, 
            self.title, self.header_command).to_string());
        }

        let parent_child_seperator = file_path.rfind("/").unwrap_or(file_path.len());

        let parent_path = &file_path[0..parent_child_seperator];

        let file = &file_path[parent_child_seperator+1..file_path.len()];

        let file_name = &file[0..file.rfind(".").unwrap_or(file.len())];

        // let cmds = self.execution_code
        let mut output = String::new();

        if compile
        {
            let parsed_cmd = Language::parse_for_command(&self.compiliation_code, &file_path, &file_name, &file, &parent_path);
            // let outpu
            let ou = Language::execute_line(&parsed_cmd);

            output += &ou.unwrap();

        }
        if execute
        {
            let dependency = if stdin.is_some() { format!(" < {}", stdin.unwrap())} else { String::new() };

            let line = format!("{}{}", self.running_code, dependency);

            let parsed_command = Language::parse_for_command(&line, &file_path, &file_name, &file, &parent_path);
    
            output += &Language::execute_line(&parsed_command).unwrap();
        }

        return Ok(output.to_string());
    }

    pub fn file2language(path: &Path) -> Result<Language<'static>, String>
    {
        let extension = path.extension().unwrap().to_str().unwrap();

       Language::extension2language(extension)
    }

    pub fn extension2language(extension: &str) -> Result<Language<'static>, String>
    {
        match extension
        {
            "java" => Ok(language_list::JAVA),
            "py" => Ok(language_list::PYTHON),
            _ => Err("Not a valid language!".to_string())
        }
    }

    pub fn language2file(language: &Language) -> Result<String, String>
    {
        match language
        {
            &language_list::JAVA => Ok("java".to_string()),
            &language_list::PYTHON => Ok("py".to_string()),
            _ => Err("Not a valid language!".to_string())
        }
    }

    fn parse_for_command(line: &str, file_path: &str, file_name: &str, file: &str, parent_path: &str) -> String
    {
        line.replace("{PATH}", file_path)
        .replace("{FILE_NAME}", file_name)
        .replace("{FILE_NAME_WITH_EXTENSION}", file)
        .replace("{PARENT_PATH}", parent_path)
    }

    fn execute_line(cmds: &str) -> Result<String, String>
    {
        let mut output = String::new();

        for line in cmds.split("\n")
        {
            let split_line = line.split(" ").collect::<Vec<&str>>();

            let args = &split_line[1..split_line.len()];

            let mut command: Command = Command::new(split_line[0]);

            command.args(args.iter());
            
            match command.output()
            {
                Ok(res) => {
                   let std_out =  String::from_utf8_lossy(&res.stdout);

                    output += &std_out;
                },

                Err(err) => {
                    output += &err.to_string();

                    return Err(output.to_string());
                }
            }

        };

        Ok(output)
    }
}

