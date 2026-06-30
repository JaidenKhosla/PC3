use std::io::{Read, Write};
use std::{fs::{self, File}};
use std::path::{PathBuf, Path};

#[allow(unused)]
pub fn get_all_files<'a>(path: &'a Path) -> Vec<PathBuf>
{
    let mut files: Vec<PathBuf> = vec![];

    let dir = fs::read_dir(&path).unwrap();

    for entry in dir
    {
        let path = entry.unwrap().path();

        if path.is_dir()
        {
            files.append(&mut get_all_files(&path));
        }
        else {
            files.push(path);
        }

    }

    files
}

#[allow(unused)]
pub fn copy_directory<'a>(from: &Path, to: &Path) -> Result<(), String>
{
    if !fs::exists(&from).unwrap() && !from.is_dir()
    {
        return Err("Not a valid directory!".to_string())
    }

    // let from_dir = fs::read_dir(&from).unwrap();

    let files = get_all_files(&from);

     for file_path in files.iter()
    {
        
        let mut file = fs::File::open(&file_path).unwrap();

        let mut content_buffer: Vec<u8> = vec![];

        let _ = file.read_to_end(&mut content_buffer);

        let to_path = to.join(file_path);

        let _ = fs::create_dir_all(to_path.parent().unwrap());

        let mut phys_file = File::create(&to_path).unwrap();

        let _ = phys_file.write_all(&content_buffer).unwrap();
    };

    Ok(())
}

