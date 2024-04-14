use crate::hashing;
use crate::databasewriter;
pub struct User {
    name : String, 
    email : String, 
    password : hashing::Hash,
}

impl User {

    pub fn new() -> User {
        User {
            name: "".to_string(),
            email : "".to_string(),
            password : hashing::Hash::new()
        }
    }    

    pub fn get_user_object(self) -> User {
        return self;
    }

    pub fn set_user_object(self,name: String, email: String, password: String) {
        let mut password_hash : hashing::Hash = hashing::Hash::new();
        let hash = password_hash.password(&password, true, "55718428862".to_string());
        
        let writer : databasewriter::Writer = databasewriter::Writer::new();
    
        writer.read_database();
        
        let mut object_builder : String = "".to_string();

        object_builder.push_str(&name);
        object_builder.push_str(";");
        object_builder.push_str(&email);
        object_builder.push_str(";");
        object_builder.push_str(&hash);
        object_builder.push_str(";");

        writer.write_database(object_builder.to_string());

        println!("{}", writer.read_database_id(0));

        
    }

    

}
