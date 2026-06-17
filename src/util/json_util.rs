use std::path::Path;
use std::io::{BufReader, Error, Read};
use std::fs::File;
use json::{self, JsonValue};

pub fn read_json(path: &Path) -> Result<JsonValue, Error>
{
    let result = || -> Result<JsonValue, Error>
    {

        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
    
        let mut buffer: Vec<u8> = Vec::new();
    
    
        reader.read_to_end(&mut buffer).unwrap();
    
        let unparsed_json = String::from_utf8_lossy(&buffer);
    
        let parsed_json = json::parse(&unparsed_json).unwrap();

        Ok(parsed_json)
    }();

    result
}