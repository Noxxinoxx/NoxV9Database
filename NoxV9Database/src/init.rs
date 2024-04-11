/*

use std::fs::File;
use serde_json::Value;
use std::io::prelude::*;
struct options_struct {
    clusters : Vec<String>
}
//inits the database takes in one parameter with options.
pub fn init(options: Vec<String>) {
    //what we want to do here is pretty strait forward. 
    //we want create a database based on some input. 
    //this is because we want to use this database for more then one project. 
    //that also means we need to add api request for custom functions.

    //so we need the database to be handle data and send data. 

    //this function will be started from an api request from the server I will
    //load all the files we want to use and if we need to create any new structs.
    let clusters_struct : options_struct = options_struct {clusters: options};

    for cluster in clusters_struct.clusters {
        println!("{:?}",cluster);
        let connect = format!("/Users/nox/Desktop/Nox/NoxV9Database/NoxV9Database/Database/{cluster}.json");
        println!("{}", connect);
        let _ = create_file(connect);
    }    




}

//this function takes in a path string
//the function wants to return a Result<(), std::io::Error>
//at this point we have 0 error handeling.
fn create_file(path : String) -> std::io::Result<()> {
    let mut file = File::create_new(path)?;
    file.write_all(b"[]")?;
    println!("files created");
    Ok(())
}

*/