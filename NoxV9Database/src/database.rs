use serde_json::Value;
use std::fs;
use serde_json::Serializer; 
use serde_json::Deserializer;



//mabye we want to change this to a socket connection insted of a standard http server.
//use a TCP server for this.
struct User {
    name : String, 
    email : String, 
    password : String,
}

impl User {

    pub fn new() -> User {
        User {
            name: "".to_string(),
            email : "".to_string(),
            password : "".to_string(),
        }
    }    



}
