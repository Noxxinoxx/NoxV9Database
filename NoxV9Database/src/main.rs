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


    

    if(req.contains("&cc")) {
        
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let name : String = data.get(1).unwrap().to_string();
        //dbg!(data);

        let db_data : String = data.get(3).unwrap().to_string();
        let co_data = handel_data_from_command(db_data);

    
        let co : database::CustomObject = database::CustomObject::new(co_data, name);

        let res = "hello".as_bytes();
        stream.write(res).expect("Failed to write response!");

    }else {
        let res = "hello world thing".as_bytes();
        stream.write(res).expect("Failed to write response!");
    }

    
}


fn main(){

    let listener = TcpListener::bind("192.168.68.72:3001").expect("Failed to bind adress");
    println!("server litenening on 192.168.68.72:3001");

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

    let user : database::User = database::User::new();
    let worte_database = user.set_user_object("Leo".to_string(), "Leo@outlook.com".to_string(), "LeosPasswordIsStrong!".to_string(), true);
    
    let mut co_data : Vec<String> = Vec::new();
    
    co_data.push("start_data".to_string());
    co_data.push("end_data".to_string());
    co_data.push("x".to_string());
    co_data.push("y".to_string());
    co_data.push("z".to_string());
    co_data.push("T".to_string());

    let co : database::CustomObject = database::CustomObject::new(co_data, "VR_Job_Data.csv".to_string());
    println!("{}",worte_database);

}
