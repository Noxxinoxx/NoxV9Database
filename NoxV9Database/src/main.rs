mod database;
mod init;
mod hashing;
mod databasewriter;
mod authentication;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::authentication::Authenticator;
// use crate::databasewriter::Writer; import not currently used in this file


/*
    Command handeler for tcp res data.
    This function takes in a String type witch is the tcp res data
    and return a Vec with the split tcp command data.
    A command can look like this.
    Exemple: &cc:cluster_name:&data:data1,data2,data3:
    This command will take create a cluster with name cluster_name and add the data : data1, data2, data3.
*/
fn handle_command(req_data : String) -> Vec<String>{
    let cluster_name = req_data.split(":");
    let data : Vec<String> = cluster_name.into_iter().map(|x| x.to_string()).collect();
    return data;
}

/*
    This function will handle the data that we get from a tcp res.
    so the data1, data2, data3.
    It will split the data and add it to a Vec<String> and then return it.
*/
fn handel_data_from_command(data : String) -> Vec<String>{
    let spliter = data.split(",");
    let final_data : Vec<String> = spliter.into_iter().map(|x| x.to_string()).collect();

    return final_data;
}

/*
    This function handels the conneted clients and what commands they are sending to the database.
    it takes in a TcpStream witch contains some data that are getting sent from the client.
*/
fn handle_client(mut stream : TcpStream) {
    
    println!("connected to the server with ip: {}", stream.local_addr().unwrap());

    let mut buffer = [0; 1024];

    stream.read(&mut buffer).expect("Failed to read from client");

    let req = String::from_utf8_lossy(&buffer[..]);
    
    println!("recived : {}", req);


    /*
        Command: &cc,
        Description: the cc command stands for create cluster this is called when we want to create a new cluster in the database.
        Format: token:&cc:cluster_name:&data:1,1,1,1,2,55,hej:
    */
    if req.contains("&cc") {
        
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            let name : String = data.get(1).unwrap().to_string();
            //dbg!(data);

            let db_data : String = data.get(3).unwrap().to_string();
            let co_data = handel_data_from_command(db_data);


            let _co : () = database::new_custom_object(co_data, name);

            let res = "Success, you created a cluster and wrote the struture to it.".as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write response");
        }
    }
    /*
        Command: &ac,
        Description: the ac command stands for add cluster this is called when we want to add data to an allready existring cluster in the database.
        Format: token:&ac:cluster_name:&data:1,1,1,1,2,55,hej:
    */
    else if req.contains("&ac") {

        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        if Authenticator::verify_token(&data[0]) {
            let name : String = data.get(2).unwrap().to_string();

            let db_data : String = data.get(4).unwrap().to_string();
            let co_data = handel_data_from_command(db_data);

            let _co = database::update_database(co_data, name);

            let res = "Success, you wrote data to the cluster".as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write a response");
        }
    }
    /*
        Command: &gc,
        Description: the gc command stands for get cluster this is called when we want to get all the data from a cluster in the database.
        Format: token:&gc:cluster_name:
    */
    else if req.contains("&gc") {
        //&gc:database1234.csv:
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            let name : String = data.get(2).unwrap().to_string();

            let co = database::get_database(name);
            println!("{}", &co.as_str());
            let res = co.as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token.".as_bytes()).expect("Failed to write response.");
        }
    }
    /*
        Command: &gic,
        Description: the gic command stands for get index cluster this is called when we want to get data from an index line in a cluster.
        Format: token:&gic:cluster_name:&data:index_as_int:
    */
    else if req.contains("&gic") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        println!("{:?}", &data);

        if Authenticator::verify_token(&data[0]) {
            let name : String = data.get(2).unwrap().to_string();

            let db_data : String = data.get(4).unwrap().to_string();
            let co_data = handel_data_from_command(db_data);

            let index_db_index : i32 = co_data.get(0).unwrap().to_string().parse().unwrap();

            let co = database::get_index_database(name, index_db_index);
            println!("{}", &co.as_str());
            let res = co.as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token.".as_bytes()).expect("Failed to write a response");
        }
    }
    /*
        Command: &rc,
        Description: the rc command stands for reset cluster this is called when we want reset/clear a cluster from all its data.
        Format: token:&rc:cluster_name:
    */
    else if req.contains("&rc") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            let name : String = data.get(2).unwrap().to_string();

            let co = database::clear_database(&name);
            println!("{}", &co.as_str());
            let res = co.as_bytes();
            stream.write(res).expect("Failed to write response!");

        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write a response.");
        }
    }
    /*
        Command: &udjt,
        Description: the udjt command stands for unity done job true and it will change a varible in the Unity_Done_With_Job.csv cluster gets called by unity when a job is done.
        Format: token:&udjt:
    */
    else if req.contains("&udjt") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            println!("you allways get here");
            database::clear_database(&"Unity_Done_With_Job.csv".to_string());
            let mut vec : Vec<String> = Vec::new();
            vec.push("Job".to_string());
            let _database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());

            let mut vec : Vec<String> = Vec::new();
            vec.push("true".to_string());
            let _database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());

            let res = "Success!".as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write response");
        }
    }
    /*
        Command: &udjf,
        Description: the udjf command stands for unity done job false, and it will change a varible in the Unity_Done_With_Job.csv cluster gets called by Noa test tool when a new job is started.
        Format: token:&udjf:
    */
    else if req.contains("&udjf") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            println!("unity database should be false here!");
            database::clear_database(&"Unity_Done_With_Job.csv".to_string());
            let mut vec : Vec<String> = Vec::new();
            vec.push("Job".to_string());
            let _database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());

            let mut vec : Vec<String> = Vec::new();
            vec.push("false".to_string());
            let _database = database::update_database(vec,"Unity_Done_With_Job.csv".to_string());


            let res = "Success!".as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token.".as_bytes()).expect("Failed to write response");
        }

    }
    /*
        Command: &udjget,
        Description: the udjget is a get command and will send the current varible in the Unity_Done_With_Job.csv cluster.
        Format: token:&udjget:
    */
    else if req.contains("&udjget"){
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            /* let name : String = data.get(2).unwrap().to_string();

            let db_data : String = data.get(4).unwrap().to_string();
            let co_data = handel_data_from_command(db_data);

            let index_db_index : i32 = co_data.get(0).unwrap().to_string().parse().unwrap();

            let co = database::get_index_database(name, index_db_index);
            let mut builder : String = "&ud".to_owned();
            builder.push_str(&co);
            println!("{}", &builder.as_str());
            let res = builder.as_bytes(); */

            let co = database::get_index_database("Unity_Done_With_Job.csv".to_string(), 1);
            let res = co.as_bytes();

            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write a response.");
        }

    } 
    /*
    Command: &udbi,
    Description: udbi stands for update database by index, and takes in an index to where the new data should be put.
    Format: token:&udbi:cluster_name:index:data:
    */
    else if req.contains("&udbi") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();

        if Authenticator::verify_token(&data[0]) {
            let cluster_name : String = data.get(2).unwrap().to_string();
            let co_data = handel_data_from_command(data.get(4).unwrap().to_string());
            let db_index : i32 = co_data.get(3).unwrap().to_string().parse().unwrap();
            database::update_database_by_index(cluster_name, db_index, data);

            let res = "Success!".as_bytes();
            stream.write(res).expect("Failed to write response!");
        } else {
            stream.write("Invalid token".as_bytes()).expect("Failed to write a response.");
        }
    }
    /*
    Command: &login,
    Description: login simply stands for log in and takes username, password and userID (index)
    Format: &login:username:password:userID:
    Note: UserID is the index of where the info is stored in the database, hence it is important
    that the right username and password share the same index in the two different databases.
    */
    else if req.contains("&login") {
        let clone_req = req.clone().to_string();
        let data = handle_command(clone_req).clone();
        let user_id = data[3].parse::<i32>().unwrap();

        // Checks so that index is not longer than db length, to prevent panic.
        if Authenticator::check_db_length(user_id)
            && Authenticator::verify_password(data[2].to_string(), user_id - 1)
            && Authenticator::verify_username(data[1].to_string(), user_id - 1){

            let response = Authenticator::token_generator();
            stream.write(response.as_bytes()).expect("Could not retrieve token");
        } else {
            stream.write("User not found".as_bytes()).expect("Could not write response");
        }
    }
}

fn main(){

    let _home_ip = String::from("192.168.1.242:3001");
    let _work_ip = String::from("192.168.50.128:3001");

    let listener = TcpListener::bind(_work_ip).expect("Failed to bind address");
    println!("server listening on 192.168.68.72:3001");

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