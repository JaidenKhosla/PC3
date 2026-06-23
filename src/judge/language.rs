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
    
    pub fn get_arguments(&self, file_path: &str, stdin: Option<&str>) -> [String; 3]
    {
      
        let parent_child_seperator = file_path.rfind("/").unwrap_or(file_path.len());

        let parent_path = file_path[0..parent_child_seperator].to_string();

        let file = file_path[parent_child_seperator+1..file_path.len()].to_string();

        let file_name = file[0..file.rfind(".").unwrap_or(file.len())].to_string();


        [parent_path, file, file_name]
    }

    fn parse_for_command(line: &str, file_path: &str, file_name: &str, file: &str, parent_path: &str) -> String
    {
        line.replace("{PATH}", file_path)
        .replace("{FILE_NAME}", file_name)
        .replace("{FILE_NAME_WITH_EXTENSION}", file)
        .replace("{PARENT_PATH}", parent_path)
    }

    pub fn get_compilation_line(&self, file_path: &str) -> String
    {
        let [parent_path, file, file_name] = self.get_arguments(file_path, None);

        Language::parse_for_command(self.compiliation_code, file_path, &file_name, &file, &parent_path)
    }
    
    pub fn get_execution_line(&self, file_path: &str) -> String
    {
        let [parent_path, file, file_name] = self.get_arguments(file_path, None);

        Language::parse_for_command(self.running_code, file_path, &file_name, &file, &parent_path)
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
}

