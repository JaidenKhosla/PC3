pub(crate) use std::process::Command;
use std::{collections::hash_map::HashMap};

/*(
Execution Code:

{FILE_NAME}
{FILE_NAME_WITH_EXTENSION}
{PATH}
{PARENT PATH}

) */
pub struct Language<'a>
{
    pub title: &'a str,
    pub header_command: &'a str,
    pub execution_code: &'a str,
    // pub installation_code: String,
    // timeout_period: u32 //in seconds/
}

impl<'a> Language<'a>
{
    pub fn new(title: &'a str, header_command: &'a str, execution_code: &'a str) -> Language<'a>
    {
        Language {
            title,
            header_command,
            execution_code,
        }
    }

    pub fn exists_within_device(&self) -> bool 
    {
        return Command::new(&self.header_command).output().is_ok();
    }

    pub fn execute(&self, file_path: &str) -> Result<String, String>
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

        let cmds = self.execution_code
        .replace("{PATH}", file_path)
        .replace("{FILE_NAME}", file_name)
        .replace("{FILE_NAME_WITH_EXTENSION}", file)
        .replace("{PARENT_PATH}", parent_path);

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

        }

        return Ok(output.to_string());
    }
}

