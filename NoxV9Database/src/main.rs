mod database;
mod hashing;
mod databasewriter;
mod tools;
mod commandhandler;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


struct Request {
    command : String,
    data : Vec<HashMap<String, Vec<String>>>
}


/*
{
    "command" : "nsj",
    "data" : [
        {"unity": ["data1", "data2", "data3", "data4"]}    
    
    ],
    "auth" : "token"
}
*/

/*
    This function handels the conneted clients and what commands they are sending to the database.
    it takes in a TcpStream witch contains some data that are getting sent from the client.
*/
fn handle_client(mut stream : TcpStream) {
    
    println!("connected to the server with ip: {}", stream.local_addr().unwrap());

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read from client");

    let req = String::from_utf8_lossy(&buffer[..]).to_string();
    
    println!("recived : {}", req);

    let data = commandhandler::command_handler(req);
    
    stream.write(data.as_bytes());

    println!("{}",data);
    
   
    
}

fn main(){

    let listener = TcpListener::bind("192.168.50.12:3001").expect("Failed to bind adress");
    println!("server litenening on 192.168.50.12:3001");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to connect to server and client: {}", e);
            }
        }
    }

}