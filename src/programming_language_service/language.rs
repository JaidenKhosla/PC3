pub(crate) use std::process::Command;
/*(
Execution Code:

{FILE_NAME_OF_FILE}
{FILE_NAME_WITH_EXTENSION}
{PATH}
{PARENT PATH}

) */
pub struct language 
{
    title: String,
    header_command: String,
    execution_code: String,
    timeout_period: u32 //in seconds
}

impl language
{
    pub fn new(title: impl Into<String>, header_command: impl Into<String>, execution_code: impl Into<String>, timeout_period: u32) -> language
    {
        return language 
        {
            title: title.into(),
            header_command: header_command.into(),
            execution_code: execution_code.into(),
            timeout_period: timeout_period
        };
    }

    pub fn get_title(&self) -> &str
    {
        return &self.title;
    }

    pub fn get_header_command(&self) -> &str
    {
        return &self.header_command;
    }

    pub fn exists_within_device(&self) -> bool 
    {
        return Command::new(&self.header_command).output().is_ok();
    }

    pub fn execute(&self, file_path: &str) -> Result<String, String>
    {
        if(!*&self.exists_within_device())
        {
            return Err(format!(
            r#"Programming Language doesn't exist on device. 
            PROGRAMMING NAME: {}
            HEADER_COMMAND: {}"#, 
            self.get_title(), self.get_header_command()).to_string());
        }

        let parentChildSeperator = file_path.rfind("/").unwrap_or(file_path.len());

        let parentPath = &file_path[0..parentChildSeperator];

        let file = &file_path[parentChildSeperator+1..file_path.len()];

        let file_name = &file[0..file.rfind(".").unwrap_or(file.len())];

        let cmds = self.execution_code
        .replace("{PATH}", file_path)
        .replace("{FILE_NAME}", file_name)
        .replace("{FILE_NAME_WITH_EXTENSION}", file)
        .replace("{PARENT_PATH}", parentPath);

        let mut output = String::new();

        for line in cmds.split("\n")
        {
            println!("{}", line);
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

