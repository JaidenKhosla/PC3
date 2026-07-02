use std::io::{BufRead, Write};
use std::net::{self, TcpStream};
use std::io;

use std::process::Command;

// mod judge;

#[allow(unused)]
pub fn main()
{

    let socket_address = std::env::var("socket_address").expect("Failed to get the following env variable: socket_address");

    let listener = net::TcpListener::bind(&socket_address).expect("Failed to bind to address!");

    println!("Listening on {}", &socket_address);


    for unparsed_socket in listener.incoming()
    {
        println!("Potential connection found!");

        handle_connection(unparsed_socket);

    }
}   

pub fn handle_connection(unparsed_socket: Result<TcpStream, std::io::Error>) -> Result<(), Box<dyn std::error::Error>>
{   
    let mut socket = unparsed_socket.expect("Failed to unwrap socket!");

    let mut reader = io::BufReader::new(&socket);

    // let mut messages: Vec<String> = vec![];

    let mut buff: Vec<u8> = vec![];

    let bytes = reader.read_until(b'\0', &mut buff).expect("Reading socket failed!");

    let message = String::from_utf8_lossy(&buff);

    let output = on_message(&message ).expect("on_message raised an error!");
    
    socket.write(output.as_bytes()).expect("Writing to socket failed!");
    
    Ok(())
}

/*
LAYOUT GUIDE:

RUN_COMMAND

*/

#[allow(unused)]
pub fn on_message(message: &str ) -> Result<String, Box<dyn std::error::Error>>
{
    let splitted = message.split(" ").collect::<Vec<&str>>();

    let mut command = Command::new(splitted[0]);

    command.args(splitted[1..].into_iter());

    let ou = command.spawn().expect("Spawning command in on_message failed!").wait_with_output().expect("Failed to yield output in on_message");

    let output = format!("{}{}", String::from_utf8_lossy(&ou.stdout), String::from_utf8_lossy(&ou.stderr));

    Ok(output)
}