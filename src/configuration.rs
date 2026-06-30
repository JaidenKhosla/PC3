use std::{path::PathBuf};

use dirs;
use crate::util::file_util::{copy_directory};
use include_dir::{Dir, include_dir};



#[allow(unused)]
const ASSETS: Dir = include_dir!("$CARGO_MANIFEST_DIR/config");

pub fn get_configuration_directory() -> PathBuf
{
    dirs::config_dir().unwrap().join("PC3")
}

#[allow(unused)]
pub fn init(delete_previous: bool) -> Result<(), Box<dyn std::error::Error>>
{

    let  configuration_directory = get_configuration_directory();
    // let local_directory = dirs::data_dir()

    if delete_previous && configuration_directory.exists()
    {
        let _ = std::fs::remove_dir_all(&configuration_directory);
    }

    println!("{:?}", &configuration_directory);

    let _ = copy_directory(&ASSETS.path(), &configuration_directory).unwrap();

    Ok(())
}

