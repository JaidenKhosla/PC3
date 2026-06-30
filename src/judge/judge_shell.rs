use std::io::{BufRead, Write};
use std::net;
use std::io;

use std::process::Command;

// mod judge;

#[allow(unused)]
pub fn main()
{

    let socket_address = std::env::var("socket_address").unwrap();

    let listener = net::TcpListener::bind(&socket_address).unwrap();

    println!("Listening on {}", &socket_address);


    for unparsed_socket in listener.incoming()
    {
        println!("Potential connection found!");

        let _ =  || -> Result<(), Box<dyn std::error::Error>> {
            let mut socket = unparsed_socket.unwrap();

            let mut reader = io::BufReader::new(&socket);

            // let mut messages: Vec<String> = vec![];

            let mut buff: Vec<u8> = vec![];

            let bytes = reader.read_until(b'\0', &mut buff).unwrap();

            let message = String::from_utf8_lossy(&buff);

            let output = on_message(&message ).unwrap();
            
            socket.write(output.as_bytes()).unwrap();
            
            Ok(())

        }().unwrap();
    }
}   

/*
LAYOUT GUIDE:

RUN_COMMAND

*/

#[allow(unused)]
pub fn on_message(message: &str ) -> Result<String, String>
{
    let splitted = message.split(" ").collect::<Vec<&str>>();

    let mut command = Command::new(splitted[0]);

    command.args(splitted[1..].into_iter());

    let ou = command.spawn().unwrap().wait_with_output().unwrap();

    let output = format!("{}{}", String::from_utf8_lossy(&ou.stdout), String::from_utf8_lossy(&ou.stderr));

    Ok(output)
}