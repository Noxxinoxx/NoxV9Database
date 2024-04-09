use serde_json::Value;
use std::fs;
use serde_json::Serializer; 
use serde_json::Deserializer;
#[derive(Serialize, Deserialize)]
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
