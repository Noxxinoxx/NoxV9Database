mod database;
mod hashing;
mod databasewriter;
mod tools;
mod commandhandler;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt,AsyncWriteExt};
use tokio::net::TcpStream;

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
async fn handle_client(mut stream :&mut TcpStream) -> Result<(), Box<dyn std::error::Error>>   {
    
    println!("connected to the server with ip: {}", stream.local_addr().unwrap());

    let mut buffer = [0; 10240];

    let req = stream.read(&mut buffer).await.expect("failed to read the data from client!");
    
    if req == 0{return Ok(());}



    let data_string = String::from_utf8_lossy(&buffer[..]).to_string();
    
    println!("recived : {}", data_string);

    let data = commandhandler::command_handler(data_string);
    
    if let Err(e) = stream.write_all(data.as_bytes()).await{eprintln!("failed to write back to socket!");return Ok(());}


    println!("data that is returned from the database {}",data);
    
    Ok(())
    
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let listener = TcpListener::bind("localhost:3001").await?;
    println!("server litenening on 192.168.50.12:3001");

    loop {
        let (mut socket ,_) = listener.accept().await?;
 
    
        tokio::spawn(async move {
            if let Err(e) = handle_client(&mut socket).await {
               println!("failed to process connection; error = {}", e);
            } 

        });
    
        


    }

}
