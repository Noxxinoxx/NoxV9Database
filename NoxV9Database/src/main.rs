mod database;
mod init;
mod hashing;
mod databasewriter;


use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_command(req_data : String) -> Vec<String>{
    //format for tcp command.
    //&cc = create cluster.
    //&cc:name:&data:1,1,1,1,1;
    //&ac = add cluster.
    //&ac:name:&data:1,1,1,1,1;
    
    let cluster_name = req_data.split(":");
    let data : Vec<String> = cluster_name.into_iter().map(|x| x.to_string()).collect();
    //let data : Vec<String> = cluster_name.collect();
    //dbg!(data);

    return data;
}

fn handel_data_from_command(data : String) -> Vec<String>{
    let spliter = data.split(",");
    let final_data : Vec<String> = spliter.into_iter().map(|x| x.to_string()).collect();

    return final_data;
}

fn handle_client(mut stream : TcpStream) {
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");

    let req = String::from_utf8_lossy(&buffer[..]);
    println!("recived : {}", req);

    //impl a handler to see if a user is auths for the tcp server.

    

    if(req.contains("&cc")) {
        
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();
        //dbg!(data);

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

    
        let co : () = database::new_custom_object(co_data, name);

        let res = "Success, you created a cluster and wrote the struture to it.".as_bytes();
        stream.write(res).expect("Failed to write response!");

    }else if(req.contains("&ac")) {

        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

    
        let co = database::update_database(co_data, name);

        let res = "Success, you wrote data to the cluster".as_bytes();
        stream.write(res).expect("Failed to write response!");
    }else if(req.contains("&gc")) {
        //&gc:database1234.csv:
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();
    
        let co = database::get_database(name);
        println!("{}", &co.as_str());
        let res = co.as_bytes();
        stream.write(res).expect("Failed to write response!");
    }else if(req.contains("&gic")) {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

        let index_db_index : i32 = co_data.get(0).unwrap().to_string().parse().unwrap();
    
        let co = database::get_index_database(name, index_db_index);
        println!("{}", &co.as_str());
        let res = co.as_bytes();
        stream.write(res).expect("Failed to write response!");
    }

    
}


fn main(){

    let listener = TcpListener::bind("192.168.68.72:3001").expect("Failed to bind adress");
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
/* 
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.68.72:3001")?;

    let mut buffer = [0;1024];

    stream.write("&ac:database1234.csv:&data:(10,10,10),(100, 100, 100), 12, 8, 2, 1:".as_bytes())?;
    stream.read(&mut buffer);

    println!("{}", String::from_utf8_lossy(&buffer[..]));


    Ok(())
} // the stream is closed


 */