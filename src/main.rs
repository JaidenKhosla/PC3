mod programming_language_service;

use std::{io::Read, process::Command};
use std::io::{BufRead, BufReader};
use std::fs::File;

use programming_language_service::{language};

fn main() {
    let language = language::language::new("python", "python", "python {PATH}", 30);
    let java = language::language::new("java", "java", "javac {PATH}\njava -classpath {PARENT_PATH} {FILE_NAME}", 30);
    let output = java.execute("src/needed_files/test.java");

    println!("{}", language.exists_within_device());
    println!("{}", &output.unwrap());
}
