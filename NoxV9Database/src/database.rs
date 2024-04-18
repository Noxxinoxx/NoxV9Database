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

    pub fn set_user_object(self,name: String, email: String, password: String, register: bool) {
        let mut password_hash : hashing::Hash = hashing::Hash::new();
        let writer : databasewriter::Writer = databasewriter::Writer::new();

        if(register) {
            password_hash.gen_hash_type(&password);
        }else {

            let database = writer.read_database();
            let mut hash_type = "".to_string();
            for i in 0..writer.get_database_length() - 1 {
                println!("{}", writer.read_database_id(i as i32));
                println!("{}", writer.get_data_points(i as i32, 1));

                if(name == writer.get_data_points(i as i32, 0)) {
                    hash_type = writer.get_data_points(i as i32, 2); 
                    println!("worked");
                    break;
                    
                }
            }

            println!("{}", hash_type);
            password_hash.set_hash_type(password_hash.retrieve_salt(&hash_type));    
        }

        let hash = password_hash.password(&password);


        println!("{}", writer.get_database_length());
    

        let mut object_builder : String = "".to_string();
        object_builder.push_str(&name);
        object_builder.push_str(",");
        object_builder.push_str(&email);
        object_builder.push_str(",");
        object_builder.push_str(&hash);

        writer.write_database(object_builder.to_string());

        println!("{}", writer.read_database_id(0));

        
    }

    

}
