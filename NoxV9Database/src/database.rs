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
        let hash = password_hash.password(&password, false, "55718428862".to_string());
    
        println!("{}", hash);
    
        println!("{}", password_hash.retrieve_salt(&hash));

        let hash1 = password_hash.password(&password, false, password_hash.retrieve_salt(&hash));
        
        println!("{}", hash1);
        let writer : databasewriter::Writer = databasewriter::Writer::new();
        writer.read_database();

        println!("{}", writer.read_database_id(0));

        
    }


}
